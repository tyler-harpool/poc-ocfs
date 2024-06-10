use std::{collections::HashMap, path::Path, time::Instant};
use actix_multipart::form::MultipartForm;
use actix_web::{Responder, HttpResponse};
use futures::stream::{self, StreamExt};
use rusty_tesseract::{Args, Image};
use serde::Serialize;
use regex::Regex;
use crate::models::Upload;
use pdfium_render::prelude::*;
use log::{error, info};
use tokio::task;
use image::DynamicImage;
use std::sync::{Arc, Mutex};

// Ensure the images directory exists
async fn ensure_images_dir() -> std::io::Result<()> {
    let path = Path::new("images");
    if !path.exists() {
        tokio::fs::create_dir(path).await?;
    }
    Ok(())
}

fn extract_filename_from_path(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem_str| stem_str.to_owned())
}

async fn export_pdf_to_images(path: &impl AsRef<Path>, password: Option<&str>) -> Result<Vec<DynamicImage>, PdfiumError> {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_system_library().unwrap(),
    );
    let document = pdfium.load_pdf_from_file(path, password)?;

    let render_config = PdfRenderConfig::new()
        .set_target_width(1000) // Reduced target width to speed up processing
        .set_maximum_height(1000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    let images: Vec<DynamicImage> = document.pages().iter().filter_map(|page| {
        match page.render_with_config(&render_config) {
            Ok(rendered_page) => {
                match rendered_page.as_image().as_rgba8() {
                    Some(image) => {
                        // Convert the RGBA image to RGB
                        let rgb_image = DynamicImage::ImageRgba8(image.clone()).to_rgb8();
                        Some(DynamicImage::ImageRgb8(rgb_image))
                    },
                    None => {
                        error!("Failed to convert page to RGBA8 image");
                        None
                    }
                }
            },
            Err(e) => {
                error!("Failed to render page: {:?}", e);
                None
            }
        }
    }).collect();

    Ok(images)
}

fn extract_citations(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\d+\s+(U\.S\.|F\.\d+d|S\.Ct\.|L\.Ed\.\d+|F\.\d+|F\.\d+\d+|F\.\d+)\s+\d+\b|\b\d+\s+U\.S\.C\. § \d+(\([a-zA-Z0-9]+\))?\b|\b[A-Z][a-zA-Z]+ v\. [A-Z][a-zA-Z]+\b|\bIn re [A-Z][a-zA-Z]+\b").unwrap();
    re.find_iter(text).map(|mat| mat.as_str().to_string()).collect()
}

fn extract_judges(text: &str) -> Vec<String> {
    let re = Regex::new(r"(?i)\b(?:Judge|Justice)\s+(?:[A-Z]\.\s+)?[A-Z][a-zA-Z]+\b").unwrap();
    re.captures_iter(text).map(|cap| cap[0].to_string()).collect()
}

fn guess_title_from_citations(citations: &[String]) -> Option<String> {
    let mut citation_counts = HashMap::new();
    for citation in citations {
        *citation_counts.entry(citation).or_insert(0) += 1;
    }
    citation_counts.into_iter().max_by_key(|&(_, count)| count).map(|(citation, _)| citation).cloned()
}

fn extract_case_number(text: &str) -> Option<String> {
    let re = Regex::new(r"\bNo\.\s*\d+[-–]\d+\b").unwrap();
    re.find(text).map(|mat| mat.as_str().to_string())
}

#[derive(Serialize)]
struct ProcessingResult {
    title: Option<String>,
    output: Vec<ResponseWithTags>,
    elapsed_time: f64, // seconds
}

#[derive(Serialize)]
struct ResponseWithTags {
    text: String,
    page: i32,
    tags: Vec<String>,
    judges: Vec<String>,
}

pub async fn upload_file(form: MultipartForm<Upload>) -> impl Responder {
    let temp_file_path = form.file.file.path();

    info!("temp_file_path: {:?}", temp_file_path);

    let start_time = Instant::now();

    let tesseract_args = Args {
        lang: "eng".to_owned(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ $§()123456789.!#:,&-".into(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3)
    };

    let temp_file_path = Path::new(&temp_file_path).to_path_buf();
    let _filename_stem = extract_filename_from_path(&temp_file_path).unwrap_or_else(|| "uploaded".to_string());

    let images = match export_pdf_to_images(&temp_file_path, None).await {
        Ok(images) => images,
        Err(e) => {
            error!("Failed to export PDF to images: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to export PDF to images");
        }
    };

    let tesseract_args = Arc::new(tesseract_args);
    let title = Arc::new(Mutex::new(None));

    let output: Vec<_> = stream::iter(images.into_iter().enumerate())
        .map(|(page, image)| {
            let tesseract_args = Arc::clone(&tesseract_args);
            let title = Arc::clone(&title);
            task::spawn_blocking(move || {
                let img = Image::from_dynamic_image(&image).ok()?;
                let text = rusty_tesseract::image_to_string(&img, &tesseract_args).ok()?;
                let cleaned_text = text.replace('\n', " ");
                if page == 0 {
                    let mut title_lock = title.lock().unwrap();
                    if title_lock.is_none() {
                        let citations = extract_citations(&cleaned_text);
                        *title_lock = guess_title_from_citations(&citations);
                        if title_lock.is_none() {
                            *title_lock = extract_case_number(&cleaned_text);
                        }
                    }
                }
                let tags = extract_citations(&cleaned_text);
                let judges = extract_judges(&cleaned_text);
                Some(ResponseWithTags {
                    text: cleaned_text,
                    page: page as i32, // Convert usize to i32
                    tags,
                    judges
                })
            })
        })
        .buffer_unordered(num_cpus::get() * 2) // Dynamically adjust concurrency
        .filter_map(|result| async { result.ok().flatten() })
        .collect()
        .await;

    // Remove the uploaded PDF file after processing
    if let Err(e) = tokio::fs::remove_file(&temp_file_path).await {
        error!("Failed to delete PDF file: {:?}", e);
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();

    let title = Arc::try_unwrap(title).unwrap().into_inner().unwrap();

    let result = ProcessingResult {
        title,
        output,
        elapsed_time,
    };

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // Start your server here
    Ok(())
}

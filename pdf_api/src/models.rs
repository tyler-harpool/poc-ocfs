use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub text: String,
    pub page: i32
}

#[derive(MultipartForm)]
pub struct Upload {
    pub file: TempFile,
}
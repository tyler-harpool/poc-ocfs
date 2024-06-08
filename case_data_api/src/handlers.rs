use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use common::utils::log_request;
use serde::Serialize;
use sqlx::{PgPool, Row};
use tracing::{error, info};

use crate::models::{CaseData, UpdateCaseData};

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn create_case_data(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(input): Json<CaseData>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO cases (
            case_number, client_name, case_type, case_status,
            date_opened, date_closed, assigned_attorney, notes
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(&input.case_number)
        .bind(&input.client_name)
        .bind(&input.case_type)
        .bind(&input.case_status)
        .bind(&input.date_opened)
        .bind(&input.date_closed)
        .bind(&input.assigned_attorney)
        .bind(&input.notes)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => {
            let id: i32 = record.get("id");
            log_request(&headers, Some(id), "Case data created");

            let response_data = CaseData {
                id: Some(id),
                case_number: input.case_number.clone(),
                client_name: input.client_name.clone(),
                case_type: input.case_type.clone(),
                case_status: input.case_status.clone(),
                date_opened: input.date_opened,
                date_closed: input.date_closed,
                assigned_attorney: input.assigned_attorney.clone(),
                notes: input.notes.clone(),
            };

            (StatusCode::CREATED, Json(response_data)).into_response()
        }
        Err(e) => {
            error!("Failed to create case data: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_case_data(
    _headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateCaseData>,
) -> impl IntoResponse {
    info!("Updating case data with ID: {}", id);

    let query = r#"
        UPDATE cases
        SET
            case_number = $1,
            client_name = $2,
            case_type = $3,
            case_status = $4,
            date_opened = $5,
            date_closed = $6,
            assigned_attorney = $7,
            notes = $8
        WHERE id = $9
    "#;


    match sqlx::query(query)
        .bind(input.case_number)
        .bind(input.client_name)
        .bind(input.case_type)
        .bind(input.case_status)
        .bind(input.date_opened)
        .bind(input.date_closed)
        .bind(input.assigned_attorney)
        .bind(input.notes)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("No case data found with ID: {}", id);
                (StatusCode::NOT_FOUND, "No case data found").into_response()
            } else {
                info!("Successfully updated case data with ID: {}", id);
                (StatusCode::OK, "Case data updated successfully").into_response()
            }
        }
        Err(e) => {
            error!("Failed to update case data with ID: {}: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update case data",
            )
                .into_response()
        }
    }
}

pub async fn get_case_data(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(&HeaderMap::new(), Some(id), "Fetching case");

    let query = "SELECT * FROM cases WHERE id = $1";

    match sqlx::query_as::<_, CaseData>(query)
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(case) => (StatusCode::OK, Json(case)).into_response(),
        Err(e) => {
            error!("Failed to fetch case for id {}: {:?}", id, e);
            (StatusCode::NOT_FOUND, Json("Case not found")).into_response()
        }
    }
}

pub async fn delete_case_data(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(
        &HeaderMap::new(),
        Some(id),
        "Attempting to delete case data",
    );

    let query = "DELETE FROM cases WHERE id = $1";

    match sqlx::query(query).bind(id).execute(&pool).await {
        Ok(result) => {
            let affected = result.rows_affected();
            if affected == 0 {
                error!("Case data not found with ID: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    Json(DeleteResponse {
                        message: format!("Case data not found with ID: {}", id),
                    }),
                )
                    .into_response()
            } else {
                info!(
                    "Successfully deleted {} case data record(s) with ID: {}",
                    affected, id
                );
                (
                    StatusCode::NO_CONTENT,
                    Json(DeleteResponse {
                        message: format!("Successfully deleted case data with ID: {}", id),
                    }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!("Failed to delete case data with ID: {}: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteResponse {
                    message: format!("Failed to delete case data with ID: {}", id),
                }),
            )
                .into_response()
        }
    }
}

pub async fn list_all_case_data(
    _headers: HeaderMap, // Prefix with an underscore if not used
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    log_request(&_headers, None, "Listing all case data");

    let query = "SELECT * FROM cases";

    match sqlx::query_as::<_, CaseData>(query).fetch_all(&pool).await {
        Ok(case_data_list) => (StatusCode::OK, Json(case_data_list)).into_response(),
        Err(e) => {
            error!("Failed to fetch case data list: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to fetch case data list"),
            )
                .into_response()
        }
    }
}

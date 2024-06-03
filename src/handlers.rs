use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
    Extension,
};
use serde::Serialize;
use sqlx::{PgPool, Row};
use tracing::{info, error};

use crate::models::{CaseData, UpdateCaseData};

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn create_case_data(
    Extension(pool): Extension<PgPool>,
    Json(input): Json<CaseData>,
) -> impl IntoResponse {
    info!("Creating case data: {:?}", input);

    let result = sqlx::query(
        "INSERT INTO CaseData (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) RETURNING *"
    )
        .bind(&input.civ)
        .bind(&input.fam)
        .bind(&input.prob)
        .bind(&input.dep)
        .bind(&input.juv)
        .bind(&input.crim)
        .bind(&input.traf)
        .bind(&input.data_element)
        .bind(&input.definition)
        .bind(&input.values)
        .bind(&input.currently_collected)
        .bind(&input.if_no_is_this_needed)
        .bind(&input.if_yes_where)
        .bind(&input.comments)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => {
            let case_data = CaseData {
                id: record.get("id"),
                civ: record.get("civ"),
                fam: record.get("fam"),
                prob: record.get("prob"),
                dep: record.get("dep"),
                juv: record.get("juv"),
                crim: record.get("crim"),
                traf: record.get("traf"),
                data_element: record.get("data_element"),
                definition: record.get("definition"),
                values: record.get("values"),
                currently_collected: record.get("currently_collected"),
                if_no_is_this_needed: record.get("if_no_is_this_needed"),
                if_yes_where: record.get("if_yes_where"),
                comments: record.get("comments"),
            };

            info!("Case data created with id: {}", case_data.id);

            (StatusCode::CREATED, Json(case_data)).into_response()
        }
        Err(e) => {
            error!("Failed to create case data: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_case_data(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateCaseData>,
) -> impl IntoResponse {
    info!("Updating case data with ID: {}", id);

    let query = sqlx::query!(
        r#"
        UPDATE CaseData
        SET
            civ = COALESCE($1, civ),
            fam = COALESCE($2, fam),
            prob = COALESCE($3, prob),
            dep = COALESCE($4, dep),
            juv = COALESCE($5, juv),
            crim = COALESCE($6, crim),
            traf = COALESCE($7, traf),
            data_element = COALESCE($8, data_element),
            definition = COALESCE($9, definition),
            values = COALESCE($10, values),
            currently_collected = COALESCE($11, currently_collected),
            if_no_is_this_needed = COALESCE($12, if_no_is_this_needed),
            if_yes_where = COALESCE($13, if_yes_where),
            comments = COALESCE($14, comments)
        WHERE id = $15
        "#,
        input.civ,
        input.fam,
        input.prob,
        input.dep,
        input.juv,
        input.crim,
        input.traf,
        input.data_element,
        input.definition,
        input.values,
        input.currently_collected,
        input.if_no_is_this_needed,
        input.if_yes_where,
        input.comments,
        id
    );

    match query.execute(&pool).await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("No case data found with ID: {}", id);
                return (StatusCode::NOT_FOUND, "No case data found").into_response();
            }
            info!("Successfully updated case data with ID: {}", id);
            (StatusCode::OK, "Case data updated successfully").into_response()
        }
        Err(e) => {
            error!("Failed to update case data with ID: {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update case data").into_response()
        }
    }
}

pub async fn get_case_data(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, CaseData>(
        "SELECT * FROM CaseData WHERE id = $1"
    )
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(case_data) => (StatusCode::OK, Json(case_data)).into_response(),
        Err(e) => {
            error!("Failed to fetch case data for id {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error")).into_response()
        }
    }
}

pub async fn delete_case_data(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    info!("Attempting to delete case data with ID: {}", id);

    // Parse the ID to an integer
    let id: i32 = match id.parse() {
        Ok(id) => id,
        Err(_) => {
            error!("Invalid ID format: {}", id);
            return (StatusCode::BAD_REQUEST, Json(DeleteResponse {
                message: format!("Invalid ID format: {}", id),
            })).into_response();
        }
    };

    let result = sqlx::query(
        "DELETE FROM CaseData WHERE id = $1"
    )
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            info!("Successfully deleted case data with ID: {}", id);
            (StatusCode::OK, Json(DeleteResponse {
                message: format!("Successfully deleted case data with ID: {}", id),
            })).into_response()
        }
        Err(e) => {
            error!("Failed to delete case data with ID: {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DeleteResponse {
                message: format!("Failed to delete case data with ID: {}", id),
            })).into_response()
        }
    }
}

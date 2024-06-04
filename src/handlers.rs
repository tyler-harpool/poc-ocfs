use axum::{
    extract::{Extension, Path},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
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
    headers:HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(input): Json<CaseData>,

) -> impl IntoResponse {
    let message = headers.get("X-Test-Client").unwrap();
    info!("Request from test client: {:?}", 1);
    info!("Creating case data: {:?}", input);
    println!("test, {:?}", message);


    let result = sqlx::query(
        "INSERT INTO CaseData (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) RETURNING id"
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
            let id: i32 = record.get("id");

            info!("Case data created with id: {}", id);

            let response_data = CaseData {
                id: Some(id),
                civ: input.civ.clone(),
                fam: input.fam.clone(),
                prob: input.prob.clone(),
                dep: input.dep.clone(),
                juv: input.juv.clone(),
                crim: input.crim.clone(),
                traf: input.traf.clone(),
                data_element: input.data_element.clone(),
                definition: input.definition.clone(),
                values: input.values.clone(),
                currently_collected: input.currently_collected.clone(),
                if_no_is_this_needed: input.if_no_is_this_needed.clone(),
                if_yes_where: input.if_yes_where.clone(),
                comments: input.comments.clone(),
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
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // info!("Request from test client: {:?}", test_client.0);
    info!("Fetching case data with ID: {}", id);

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
            (StatusCode::NOT_FOUND, Json("Case data not found")).into_response()
        }
    }
}

pub async fn delete_case_data(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,

) -> impl IntoResponse {
    // info!("Request from test client: {:?}", test_client.0);
    info!("Attempting to delete case data with ID: {}", id);

    let result = sqlx::query(
        "DELETE FROM CaseData WHERE id = $1"
    )
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(rows_affected) => {
            let affected = rows_affected.rows_affected();
            if affected == 0 {
                error!("Case data not found with ID: {}", id);
                (StatusCode::NOT_FOUND, Json(DeleteResponse {
                    message: format!("Case data not found with ID: {}", id),
                })).into_response()
            } else {
                info!("Successfully deleted {} case data record(s) with ID: {}", affected, id);
                (StatusCode::NO_CONTENT, Json(DeleteResponse {
                    message: format!("Successfully deleted case data with ID: {}", id),
                })).into_response()
            }
        }
        Err(e) => {
            error!("Failed to delete case data with ID: {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DeleteResponse {
                message: format!("Failed to delete case data with ID: {}", id),
            })).into_response()
        }
    }
}

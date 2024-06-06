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

use crate::models::{Charge, UpdateCharge};

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn create_charge(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(input): Json<Charge>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO charges (
            civ, fam, prob, dep, juv, crim, traf, data_element,
            definition, values, currently_collected, if_no_is_this_needed,
            if_yes_where, comments
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING id
    "#;

    let result = sqlx::query(query)
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
            log_request(&headers, Some(id), "Charge created");

            let response_data = Charge {
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
            error!("Failed to create charge: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_charge(
    _headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateCharge>,
) -> impl IntoResponse {
    info!("Updating charge with ID: {}", id);

    let query = r#"
        UPDATE charges
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
    "#;

    match sqlx::query(query)
        .bind(input.civ)
        .bind(input.fam)
        .bind(input.prob)
        .bind(input.dep)
        .bind(input.juv)
        .bind(input.crim)
        .bind(input.traf)
        .bind(input.data_element)
        .bind(input.definition)
        .bind(input.values)
        .bind(input.currently_collected)
        .bind(input.if_no_is_this_needed)
        .bind(input.if_yes_where)
        .bind(input.comments)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("No charge found with ID: {}", id);
                (StatusCode::NOT_FOUND, "No charge found").into_response()
            } else {
                info!("Successfully updated charge with ID: {}", id);
                (StatusCode::OK, "Charge updated successfully").into_response()
            }
        }
        Err(e) => {
            error!(
                "Failed to update charge with ID: {}: {:?}",
                id, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update charge",
            )
                .into_response()
        }
    }
}

pub async fn get_charge(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(&HeaderMap::new(), Some(id), "Fetching charge");

    let query = "SELECT * FROM charges WHERE id = $1";

    match sqlx::query_as::<_, Charge>(query)
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(charge) => (StatusCode::OK, Json(charge)).into_response(),
        Err(e) => {
            error!("Failed to fetch charge for id {}: {:?}", id, e);
            (StatusCode::NOT_FOUND, Json("Charge not found")).into_response()
        }
    }
}

pub async fn delete_charge(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(
        &HeaderMap::new(),
        Some(id),
        "Attempting to delete charge",
    );

    let query = "DELETE FROM charges WHERE id = $1";

    match sqlx::query(query).bind(id).execute(&pool).await {
        Ok(result) => {
            let affected = result.rows_affected();
            if affected == 0 {
                error!("Charge not found with ID: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    Json(DeleteResponse {
                        message: format!("Charge not found with ID: {}", id),
                    }),
                )
                    .into_response()
            } else {
                info!(
                    "Successfully deleted {} charge record(s) with ID: {}",
                    affected, id
                );
                (
                    StatusCode::NO_CONTENT,
                    Json(DeleteResponse {
                        message: format!("Successfully deleted charge with ID: {}", id),
                    }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!(
                "Failed to delete charge with ID: {}: {:?}",
                id, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteResponse {
                    message: format!("Failed to delete charge with ID: {}", id),
                }),
            )
                .into_response()
        }
    }
}

pub async fn list_all_charges(
    _headers: HeaderMap, // Prefix with an underscore if not used
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    log_request(&_headers, None, "Listing all charges");

    let query = "SELECT * FROM charges";

    match sqlx::query_as::<_, Charge>(query)
        .fetch_all(&pool)
        .await
    {
        Ok(charge_list) => {
            (StatusCode::OK, Json(charge_list)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch charge list: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to fetch charge list"),
            )
                .into_response()
        }
    }
}

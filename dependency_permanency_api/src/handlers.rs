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

use crate::models::{DependencyPermanency, UpdateDependencyPermanency};

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn create_dependency_permanency(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(input): Json<DependencyPermanency>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO dependencyPermanency (
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
            log_request(&headers, Some(id), "Dependency Permanency created");

            let response_data = DependencyPermanency {
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
            error!("Failed to create dependency permanency: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_dependency_permanency(
    _headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateDependencyPermanency>,
) -> impl IntoResponse {
    info!("Updating dependency permanency with ID: {}", id);

    let query = r#"
        UPDATE dependencyPermanency
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
                error!("No dependency permanency found with ID: {}", id);
                (StatusCode::NOT_FOUND, "No dependency permanency found").into_response()
            } else {
                info!("Successfully updated dependency permanency with ID: {}", id);
                (StatusCode::OK, "Dependency permanency updated successfully").into_response()
            }
        }
        Err(e) => {
            error!(
                "Failed to update dependency permanency with ID: {}: {:?}",
                id, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update dependency permanency",
            )
                .into_response()
        }
    }
}

pub async fn get_dependency_permanency(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(
        &HeaderMap::new(),
        Some(id),
        "Fetching dependency permanency",
    );

    let query = "SELECT * FROM dependencyPermanency WHERE id = $1";

    match sqlx::query_as::<_, DependencyPermanency>(query)
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(dependency_permanency) => (StatusCode::OK, Json(dependency_permanency)).into_response(),
        Err(e) => {
            error!(
                "Failed to fetch dependency permanency for id {}: {:?}",
                id, e
            );
            (
                StatusCode::NOT_FOUND,
                Json("Dependency permanency not found"),
            )
                .into_response()
        }
    }
}

pub async fn delete_dependency_permanency(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(
        &HeaderMap::new(),
        Some(id),
        "Attempting to delete dependency permanency",
    );

    let query = "DELETE FROM dependencyPermanency WHERE id = $1";

    match sqlx::query(query).bind(id).execute(&pool).await {
        Ok(result) => {
            let affected = result.rows_affected();
            if affected == 0 {
                error!("Dependency permanency not found with ID: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    Json(DeleteResponse {
                        message: format!("Dependency permanency not found with ID: {}", id),
                    }),
                )
                    .into_response()
            } else {
                info!(
                    "Successfully deleted {} dependency permanency record(s) with ID: {}",
                    affected, id
                );
                (
                    StatusCode::NO_CONTENT,
                    Json(DeleteResponse {
                        message: format!(
                            "Successfully deleted dependency permanency with ID: {}",
                            id
                        ),
                    }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!(
                "Failed to delete dependency permanency with ID: {}: {:?}",
                id, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteResponse {
                    message: format!("Failed to delete dependency permanency with ID: {}", id),
                }),
            )
                .into_response()
        }
    }
}

pub async fn list_all_dependency_permanencies(
    _headers: HeaderMap, // Prefix with an underscore if not used
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    log_request(&_headers, None, "Listing all dependency permanencies");

    let query = "SELECT * FROM dependencyPermanency";

    match sqlx::query_as::<_, DependencyPermanency>(query)
        .fetch_all(&pool)
        .await
    {
        Ok(dependency_permanency_list) => {
            (StatusCode::OK, Json(dependency_permanency_list)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch dependency permanency list: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to fetch dependency permanency list"),
            )
                .into_response()
        }
    }
}

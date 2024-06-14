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

use crate::models::{Docket, UpdateDocket};

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn create_docket(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(input): Json<Docket>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO dockets (
            date_created, date_modified, source, appeal_from_str, assigned_to_str, 
            referred_to_str, panel_str, date_last_index, date_cert_granted, date_cert_denied, 
            date_argued, date_reargued, date_reargument_denied, date_filed, date_terminated, 
            date_last_filing, case_name_short, case_name, case_name_full, slug, 
            docket_number, docket_number_core, pacer_case_id, cause, nature_of_suit, 
            jury_demand, jurisdiction_type, appellate_fee_status, appellate_case_type_information, 
            mdl_status, filepath_local, filepath_ia, filepath_ia_json, ia_upload_failure_count, 
            ia_needs_upload, ia_date_first_change, view_count, date_blocked, blocked, 
            appeal_from_id, assigned_to_id, court_id, idb_data_id, originating_court_information_id, 
            referred_to_id
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, 
            $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, 
            $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44
        ) RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(&input.date_created)
        .bind(&input.date_modified)
        .bind(&input.source)
        .bind(&input.appeal_from_str)
        .bind(&input.assigned_to_str)
        .bind(&input.referred_to_str)
        .bind(&input.panel_str)
        .bind(&input.date_last_index)
        .bind(&input.date_cert_granted)
        .bind(&input.date_cert_denied)
        .bind(&input.date_argued)
        .bind(&input.date_reargued)
        .bind(&input.date_reargument_denied)
        .bind(&input.date_filed)
        .bind(&input.date_terminated)
        .bind(&input.date_last_filing)
        .bind(&input.case_name_short)
        .bind(&input.case_name)
        .bind(&input.case_name_full)
        .bind(&input.slug)
        .bind(&input.docket_number)
        .bind(&input.docket_number_core)
        .bind(&input.pacer_case_id)
        .bind(&input.cause)
        .bind(&input.nature_of_suit)
        .bind(&input.jury_demand)
        .bind(&input.jurisdiction_type)
        .bind(&input.appellate_fee_status)
        .bind(&input.appellate_case_type_information)
        .bind(&input.mdl_status)
        .bind(&input.filepath_local)
        .bind(&input.filepath_ia)
        .bind(&input.filepath_ia_json)
        .bind(&input.ia_upload_failure_count)
        .bind(&input.ia_needs_upload)
        .bind(&input.ia_date_first_change)
        .bind(&input.view_count)
        .bind(&input.date_blocked)
        .bind(&input.blocked)
        .bind(&input.appeal_from_id)
        .bind(&input.assigned_to_id)
        .bind(&input.court_id)
        .bind(&input.idb_data_id)
        .bind(&input.originating_court_information_id)
        .bind(&input.referred_to_id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => {
            let id: i32 = record.get("id");
            log_request(&headers, Some(id), "Docket created");

            let response_data = Docket {
                id: Some(id),
                date_created: input.date_created.clone(),
                date_modified: input.date_modified.clone(),
                source: input.source,
                appeal_from_str: input.appeal_from_str.clone(),
                assigned_to_str: input.assigned_to_str.clone(),
                referred_to_str: input.referred_to_str.clone(),
                panel_str: input.panel_str.clone(),
                date_last_index: input.date_last_index.clone(),
                date_cert_granted: input.date_cert_granted.clone(),
                date_cert_denied: input.date_cert_denied.clone(),
                date_argued: input.date_argued.clone(),
                date_reargued: input.date_reargued.clone(),
                date_reargument_denied: input.date_reargument_denied.clone(),
                date_filed: input.date_filed.clone(),
                date_terminated: input.date_terminated.clone(),
                date_last_filing: input.date_last_filing.clone(),
                case_name_short: input.case_name_short.clone(),
                case_name: input.case_name.clone(),
                case_name_full: input.case_name_full.clone(),
                slug: input.slug.clone(),
                docket_number: input.docket_number.clone(),
                docket_number_core: input.docket_number_core.clone(),
                pacer_case_id: input.pacer_case_id.clone(),
                cause: input.cause.clone(),
                nature_of_suit: input.nature_of_suit.clone(),
                jury_demand: input.jury_demand.clone(),
                jurisdiction_type: input.jurisdiction_type.clone(),
                appellate_fee_status: input.appellate_fee_status.clone(),
                appellate_case_type_information: input.appellate_case_type_information.clone(),
                mdl_status: input.mdl_status.clone(),
                filepath_local: input.filepath_local.clone(),
                filepath_ia: input.filepath_ia.clone(),
                filepath_ia_json: input.filepath_ia_json.clone(),
                ia_upload_failure_count: input.ia_upload_failure_count.clone(),
                ia_needs_upload: input.ia_needs_upload.clone(),
                ia_date_first_change: input.ia_date_first_change.clone(),
                view_count: input.view_count.clone(),
                date_blocked: input.date_blocked.clone(),
                blocked: input.blocked.clone(),
                appeal_from_id: input.appeal_from_id.clone(),
                assigned_to_id: input.assigned_to_id.clone(),
                court_id: input.court_id.clone(),
                idb_data_id: input.idb_data_id.clone(),
                originating_court_information_id: input.originating_court_information_id.clone(),
                referred_to_id: input.referred_to_id.clone(),
            };

            (StatusCode::CREATED, Json(response_data)).into_response()
        }
        Err(e) => {
            error!("Failed to create docket: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_docket(
    _headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateDocket>,
) -> impl IntoResponse {
    info!("Updating docket with ID: {}", id);

    let query = r#"
        UPDATE dockets
        SET
            date_created = COALESCE($1, date_created),
            date_modified = COALESCE($2, date_modified),
            source = COALESCE($3, source),
            appeal_from_str = COALESCE($4, appeal_from_str),
            assigned_to_str = COALESCE($5, assigned_to_str),
            referred_to_str = COALESCE($6, referred_to_str),
            panel_str = COALESCE($7, panel_str),
            date_last_index = COALESCE($8, date_last_index),
            date_cert_granted = COALESCE($9, date_cert_granted),
            date_cert_denied = COALESCE($10, date_cert_denied),
            date_argued = COALESCE($11, date_argued),
            date_reargued = COALESCE($12, date_reargued),
            date_reargument_denied = COALESCE($13, date_reargument_denied),
            date_filed = COALESCE($14, date_filed),
            date_terminated = COALESCE($15, date_terminated),
            date_last_filing = COALESCE($16, date_last_filing),
            case_name_short = COALESCE($17, case_name_short),
            case_name = COALESCE($18, case_name),
            case_name_full = COALESCE($19, case_name_full),
            slug = COALESCE($20, slug),
            docket_number = COALESCE($21, docket_number),
            docket_number_core = COALESCE($22, docket_number_core),
            pacer_case_id = COALESCE($23, pacer_case_id),
            cause = COALESCE($24, cause),
            nature_of_suit = COALESCE($25, nature_of_suit),
            jury_demand = COALESCE($26, jury_demand),
            jurisdiction_type = COALESCE($27, jurisdiction_type),
            appellate_fee_status = COALESCE($28, appellate_fee_status),
            appellate_case_type_information = COALESCE($29, appellate_case_type_information),
            mdl_status = COALESCE($30, mdl_status),
            filepath_local = COALESCE($31, filepath_local),
            filepath_ia = COALESCE($32, filepath_ia),
            filepath_ia_json = COALESCE($33, filepath_ia_json),
            ia_upload_failure_count = COALESCE($34, ia_upload_failure_count),
            ia_needs_upload = COALESCE($35, ia_needs_upload),
            ia_date_first_change = COALESCE($36, ia_date_first_change),
            view_count = COALESCE($37, view_count),
            date_blocked = COALESCE($38, date_blocked),
            blocked = COALESCE($39, blocked),
            appeal_from_id = COALESCE($40, appeal_from_id),
            assigned_to_id = COALESCE($41, assigned_to_id),
            court_id = COALESCE($42, court_id),
            idb_data_id = COALESCE($43, idb_data_id),
            originating_court_information_id = COALESCE($44, originating_court_information_id),
            referred_to_id = COALESCE($45, referred_to_id)
        WHERE id = $46
    "#;

    match sqlx::query(query)
        .bind(input.date_created)
        .bind(input.date_modified)
        .bind(input.source)
        .bind(input.appeal_from_str)
        .bind(input.assigned_to_str)
        .bind(input.referred_to_str)
        .bind(input.panel_str)
        .bind(input.date_last_index)
        .bind(input.date_cert_granted)
        .bind(input.date_cert_denied)
        .bind(input.date_argued)
        .bind(input.date_reargued)
        .bind(input.date_reargument_denied)
        .bind(input.date_filed)
        .bind(input.date_terminated)
        .bind(input.date_last_filing)
        .bind(input.case_name_short)
        .bind(input.case_name)
        .bind(input.case_name_full)
        .bind(input.slug)
        .bind(input.docket_number)
        .bind(input.docket_number_core)
        .bind(input.pacer_case_id)
        .bind(input.cause)
        .bind(input.nature_of_suit)
        .bind(input.jury_demand)
        .bind(input.jurisdiction_type)
        .bind(input.appellate_fee_status)
        .bind(input.appellate_case_type_information)
        .bind(input.mdl_status)
        .bind(input.filepath_local)
        .bind(input.filepath_ia)
        .bind(input.filepath_ia_json)
        .bind(input.ia_upload_failure_count)
        .bind(input.ia_needs_upload)
        .bind(input.ia_date_first_change)
        .bind(input.view_count)
        .bind(input.date_blocked)
        .bind(input.blocked)
        .bind(input.appeal_from_id)
        .bind(input.assigned_to_id)
        .bind(input.court_id)
        .bind(input.idb_data_id)
        .bind(input.originating_court_information_id)
        .bind(input.referred_to_id)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("No docket found with ID: {}", id);
                (StatusCode::NOT_FOUND, "No docket found").into_response()
            } else {
                info!("Successfully updated docket with ID: {}", id);
                (StatusCode::OK, "Docket updated successfully").into_response()
            }
        }
        Err(e) => {
            error!("Failed to update docket with ID: {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update docket").into_response()
        }
    }
}

pub async fn get_docket(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(&HeaderMap::new(), Some(id), "Fetching docket");

    let query = "SELECT * FROM dockets WHERE id = $1";

    match sqlx::query_as::<_, Docket>(query)
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(docket) => (StatusCode::OK, Json(docket)).into_response(),
        Err(e) => {
            error!("Failed to fetch docket for id {}: {:?}", id, e);
            (StatusCode::NOT_FOUND, Json("Docket not found")).into_response()
        }
    }
}

pub async fn delete_docket(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    log_request(&HeaderMap::new(), Some(id), "Attempting to delete docket");

    let query = "DELETE FROM dockets WHERE id = $1";

    match sqlx::query(query).bind(id).execute(&pool).await {
        Ok(result) => {
            let affected = result.rows_affected();
            if affected == 0 {
                error!("Docket not found with ID: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    Json(DeleteResponse {
                        message: format!("Docket not found with ID: {}", id),
                    }),
                )
                    .into_response()
            } else {
                info!(
                    "Successfully deleted {} docket record(s) with ID: {}",
                    affected, id
                );
                (
                    StatusCode::NO_CONTENT,
                    Json(DeleteResponse {
                        message: format!("Successfully deleted docket with ID: {}", id),
                    }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!("Failed to delete docket with ID: {}: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteResponse {
                    message: format!("Failed to delete docket with ID: {}", id),
                }),
            )
                .into_response()
        }
    }
}

pub async fn list_all_dockets(
    _headers: HeaderMap, // Prefix with an underscore if not used
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let query = "SELECT * FROM dockets LIMIT 100";

    match sqlx::query_as::<_, Docket>(query).fetch_all(&pool).await {
        Ok(docket_list) => {
            let total_records = docket_list.len();
            log_request(
                &_headers,
                Some(total_records.try_into().unwrap()),
                "Listing all dockets",
            );

            (StatusCode::OK, Json(docket_list)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch docket list: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to fetch docket list"),
            )
                .into_response()
        }
    }
}

use database_utils::{establish_connection, run_migrations};
use dotenv::dotenv;
use log::{debug, info};
use pretrial_intake_api::{create_app, setup_logging}; // Correctly reference the pretrial_intake_api crate
use reqwest::Client;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

async fn start_test_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let pool = establish_connection().await;
        run_migrations(&pool).await;

        let app = create_app(pool);

        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    addr
}

#[tokio::test]
async fn test_create_get_update_delete_pretrial_intake() {
    dotenv().ok();

    // Set up logging
    setup_logging();

    let _ = env_logger::builder().is_test(true).try_init();

    let addr = start_test_server().await;
    sleep(Duration::from_secs(1)).await;

    let client = Client::new();
    let base_url = format!("http://{}", addr);

    // Step 1: Create Pretrial Intake
    info!("Step 1: Creating Pretrial Intake");
    let create_response = client
        .post(&format!("{}/pretrial_intake", base_url))
        .header("X-Test-Client", "IntegrationTest")
        .json(&json!({
            "civ": "Civil data",
            "fam": "Family data",
            "prob": "Probate data",
            "dep": "Dependency data",
            "juv": "Juvenile data",
            "crim": "Criminal data",
            "traf": "Traffic data",
            "data_element": "Element data",
            "definition": "Definition data",
            "values": json!({}),
            "currently_collected": "Yes",
            "if_no_is_this_needed": "No",
            "if_yes_where": "Some place",
            "comments": "Some comments"
        }))
        .send()
        .await
        .unwrap();

    let create_status = create_response.status();
    let create_body = create_response.text().await.unwrap();
    debug!(
        "Create Response Status: {}, Body: {}",
        create_status, create_body
    );

    if create_status != 201 {
        panic!(
            "Failed to create pretrial intake. Status: {}, Body: {}",
            create_status, create_body
        );
    }

    assert_eq!(create_status, 201);
    let create_body: serde_json::Value = serde_json::from_str(&create_body).unwrap();
    let pretrial_intake_id = create_body
        .get("id")
        .and_then(|id| id.as_i64())
        .expect("ID missing") as i32;

    info!("Created Pretrial Intake with ID: {}", pretrial_intake_id);

    // Step 2: Get Pretrial Intake
    info!("Step 2: Getting Pretrial Intake");
    let get_response = client
        .get(&format!("{}/pretrial_intake/{}", base_url, pretrial_intake_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), 200);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body["civ"], "Civil data");

    debug!("Get Response Body: {}", get_body);

    // Step 3: Update Pretrial Intake
    info!("Step 3: Updating Pretrial Intake with ID: {}", pretrial_intake_id);
    let update_response = client
        .patch(&format!("{}/pretrial_intake/{}", base_url, pretrial_intake_id))
        .header("X-Test-Client", "IntegrationTest")
        .json(&json!({
            "civ": "Updated Civil data",
            "fam": "Updated Family data",
            "prob": "Updated Probate data",
            "dep": "Updated Dependency data",
            "juv": "Updated Juvenile data",
            "crim": "Updated Criminal data",
            "traf": "Updated Traffic data",
            "data_element": "Updated Element data",
            "definition": "Updated Definition data",
            "values": json!({}),
            "currently_collected": "No",
            "if_no_is_this_needed": "Yes",
            "if_yes_where": "Updated place",
            "comments": "Updated comments"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(update_response.status(), 200);

    info!("Updated Pretrial Intake with ID: {}", pretrial_intake_id);

    // Step 4: Verify Update
    info!(
        "Step 4: Verifying Update for Pretrial Intake with ID: {}",
        pretrial_intake_id
    );
    let get_response = client
        .get(&format!("{}/pretrial_intake/{}", base_url, pretrial_intake_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), 200);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body["civ"], "Updated Civil data");

    debug!("Get After Update Response Body: {}", get_body);

    // Step 5: Delete Pretrial Intake
    info!("Step 5: Deleting Pretrial Intake with ID: {}", pretrial_intake_id);
    let delete_response = client
        .delete(&format!("{}/pretrial_intake/{}", base_url, pretrial_intake_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    assert!(
        delete_response.status() == 200
            || delete_response.status() == 204
            || delete_response.status() == 404
    );

    info!("Deleted Pretrial Intake with ID: {}", pretrial_intake_id);

    // Step 6: Verify Deletion
    info!(
        "Step 6: Verifying Deletion for Pretrial Intake with ID: {}",
        pretrial_intake_id
    );
    let get_response = client
        .get(&format!("{}/pretrial_intake/{}", base_url, pretrial_intake_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    let get_status = get_response.status();
    let get_body = get_response.text().await.unwrap();
    debug!(
        "Get after Delete Response Status: {}, Body: {}",
        get_status, get_body
    );

    assert_eq!(get_status, 404);
}

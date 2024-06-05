use log::{debug, info};
use reqwest::Client;
use serde_json::json;
use tokio;

#[tokio::test]
async fn test_create_get_update_delete_list_participant() {
    // Initialize the logger
    let _ = env_logger::builder().is_test(true).try_init();



    let client = Client::new();

    // Step 1: Create Participant
    info!("Step 1: Creating Participant");
    let create_response = client
        .post("http://localhost:3001/participants")
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
            "Failed to create participant. Status: {}, Body: {}",
            create_status, create_body
        );
    }

    assert_eq!(create_status, 201);
    let create_body: serde_json::Value = serde_json::from_str(&create_body).unwrap();
    let participant_id = match create_body.get("id") {
        Some(id) => id.as_i64().unwrap_or_else(|| {
            panic!(
                "Failed to parse participant ID from response body: {}",
                create_body
            )
        }) as i32,
        None => panic!(
            "Failed to retrieve participant ID from response body: {}",
            create_body
        ),
    };

    info!("Created Participant with ID: {}", participant_id);

    // Step 2: Get Participant
    info!("Step 2: Getting Participant");
    let get_response = client
        .get(&format!("http://localhost:3001/participants/{}", participant_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), 200);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body["civ"], "Civil data");

    debug!("Get Response Body: {}", get_body);

    // Step 3: Update Participant
    info!("Step 3: Updating Participant with ID: {}", participant_id);
    let update_response = client
        .patch(&format!("http://localhost:3001/participants/{}", participant_id))
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

    info!("Updated Participant with ID: {}", participant_id);

    // Step 4: Verify Update
    info!(
        "Step 4: Verifying Update for Participant with ID: {}",
        participant_id
    );
    let get_response = client
        .get(&format!("http://localhost:3001/participants/{}", participant_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    assert_eq!(get_response.status(), 200);
    let get_body: serde_json::Value = get_response.json().await.unwrap();
    assert_eq!(get_body["civ"], "Updated Civil data");

    debug!("Get After Update Response Body: {}", get_body);

    // Step 5: Delete Participant
    info!("Step 5: Deleting Participant with ID: {}", participant_id);
    let delete_response = client
        .delete(&format!("http://localhost:3001/participants/{}", participant_id))
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    // Accept both 200 and 204 as valid responses
    assert!(delete_response.status() == 200 || delete_response.status() == 204);

    info!("Deleted Participant with ID: {}", participant_id);

    // Step 6: Verify Deletion
    info!(
        "Step 6: Verifying Deletion for Participant with ID: {}",
        participant_id
    );
    let get_response = client
        .get(&format!("http://localhost:3001/participants/{}", participant_id))
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

    // Step 7: List Participants
    info!("Step 7: Listing Participants");
    let list_response = client
        .get("http://localhost:3001/participants")
        .header("X-Test-Client", "IntegrationTest")
        .send()
        .await
        .unwrap();
    assert_eq!(list_response.status(), 200);
    let list_body: serde_json::Value = list_response.json().await.unwrap();
    debug!("List Response Body: {:?}", list_body);
}

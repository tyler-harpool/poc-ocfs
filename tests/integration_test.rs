use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_create_get_update_delete_case_data() {
    let client = Client::new();

    // Create Case Data
    let response = client.post("http://localhost:3000/case_data")
        .json(&json!({
            "civ": "Civil data",
            "fam": "Family data",
            "prob": "Probate data",
            "dep": "Dependency data",
            "juv": "Juvenile data",
            "crim": "Criminal data",
            "traf": "Traffic data",
            "id": "12345",
            "data_element": "Element data",
            "definition": "Definition data",
            "values": {},
            "currently_collected": "Yes",
            "if_no_is_this_needed": "No",
            "if_yes_where": "Some place",
            "comments": "Some comments"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 201);

    // Get Case Data
    let response = client.get("http://localhost:3000/case_data/12345")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Update Case Data
    let response = client.patch("http://localhost:3000/case_data/12345")
        .json(&json!({
            "civ": "Updated Civil data",
            "fam": "Updated Family data",
            "prob": "Updated Probate data",
            "dep": "Updated Dependency data",
            "juv": "Updated Juvenile data",
            "crim": "Updated Criminal data",
            "traf": "Updated Traffic data",
            "id": "12345",
            "data_element": "Updated Element data",
            "definition": "Updated Definition data",
            "values": {},
            "currently_collected": "No",
            "if_no_is_this_needed": "Yes",
            "if_yes_where": "Updated place",
            "comments": "Updated comments"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    // Delete Case Data
    let response = client.delete("http://localhost:3000/case_data/12345")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 204);
}

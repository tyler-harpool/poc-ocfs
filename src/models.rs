use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use serde_json::Value; // Import serde_json::Value
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct CaseData {
    pub id: i32,  // Change this to i32
    pub civ: Option<String>,
    pub fam: Option<String>,
    pub prob: Option<String>,
    pub dep: Option<String>,
    pub juv: Option<String>,
    pub crim: Option<String>,
    pub traf: Option<String>,

    pub data_element: String,
    pub definition: Option<String>,
    pub values: Option<serde_json::Value>,
    pub currently_collected: Option<String>,
    pub if_no_is_this_needed: Option<String>,
    pub if_yes_where: Option<String>,
    pub comments: Option<String>,
}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UpdateCaseData {
    pub civ: Option<String>,
    pub fam: Option<String>,
    pub prob: Option<String>,
    pub dep: Option<String>,
    pub juv: Option<String>,
    pub crim: Option<String>,
    pub traf: Option<String>,
    pub data_element: Option<String>,
    pub definition: Option<String>,
    pub values: Option<Value>, // Change type to Option<Value>
    pub currently_collected: Option<String>,
    pub if_no_is_this_needed: Option<String>,
    pub if_yes_where: Option<String>,
    pub comments: Option<String>,
}


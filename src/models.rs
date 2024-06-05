use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use serde_json::Value;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct CaseData {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<i32>, // Use i32 instead of i64
    pub civ: Option<String>,
    pub fam: Option<String>,
    pub prob: Option<String>,
    pub dep: Option<String>,
    pub juv: Option<String>,
    pub crim: Option<String>,
    pub traf: Option<String>,
    pub data_element: String,
    pub definition: Option<String>,
    pub values: Option<Value>,
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
    pub values: Option<Value>,
    pub currently_collected: Option<String>,
    pub if_no_is_this_needed: Option<String>,
    pub if_yes_where: Option<String>,
    pub comments: Option<String>,
}

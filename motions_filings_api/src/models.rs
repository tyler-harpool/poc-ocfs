use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct MotionFiling {
    pub id: Option<i32>,
    pub civ: String,
    pub fam: String,
    pub prob: String,
    pub dep: String,
    pub juv: String,
    pub crim: String,
    pub traf: String,
    pub data_element: String,
    pub definition: String,
    pub values: Option<serde_json::Value>, // Allow NULL values
    pub currently_collected: Option<String>, // Allow NULL values
    pub if_no_is_this_needed: Option<String>, // Allow NULL values
    pub if_yes_where: Option<String>, // Allow NULL values
    pub comments: Option<String>, // Allow NULL values
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateMotionFiling {
    pub civ: Option<String>,
    pub fam: Option<String>,
    pub prob: Option<String>,
    pub dep: Option<String>,
    pub juv: Option<String>,
    pub crim: Option<String>,
    pub traf: Option<String>,
    pub data_element: Option<String>,
    pub definition: Option<String>,
    pub values: Option<serde_json::Value>, // Allow NULL values
    pub currently_collected: Option<String>, // Allow NULL values
    pub if_no_is_this_needed: Option<String>, // Allow NULL values
    pub if_yes_where: Option<String>, // Allow NULL values
    pub comments: Option<String>, // Allow NULL values
}

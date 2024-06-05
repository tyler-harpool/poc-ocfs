use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct Pleading {
    pub id: Option<i32>,
    pub civ: String,
    pub fam: String,
    pub prob: String,
    pub dep: String,
    pub juv: String,
    pub crim: String,
    pub traf: String,
    #[sqlx(rename = "dataelement")]
    pub data_element: String,
    pub definition: String,
    pub values: serde_json::Value,
    #[sqlx(rename = "currentlycollected")]
    pub currently_collected: String,
    #[sqlx(rename = "ifnoisthisneeded")]
    pub if_no_is_this_needed: String,
    #[sqlx(rename = "ifyeswhere")]
    pub if_yes_where: String,
    pub comments: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePleading {
    pub civ: Option<String>,
    pub fam: Option<String>,
    pub prob: Option<String>,
    pub dep: Option<String>,
    pub juv: Option<String>,
    pub crim: Option<String>,
    pub traf: Option<String>,
    #[serde(rename = "dataelement")]
    pub data_element: Option<String>,
    pub definition: Option<String>,
    pub values: Option<serde_json::Value>,
    #[serde(rename = "currentlycollected")]
    pub currently_collected: Option<String>,
    #[serde(rename = "ifnoisthisneeded")]
    pub if_no_is_this_needed: Option<String>,
    #[serde(rename = "ifyeswhere")]
    pub if_yes_where: Option<String>,
    pub comments: Option<String>,
}

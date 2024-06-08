use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CaseData {
    pub id: Option<i32>,
    pub case_number: String,
    pub client_name: String,
    pub case_type: String,
    pub case_status: String,
    pub date_opened: Option<NaiveDate>,
    pub date_closed: Option<NaiveDate>,
    pub assigned_attorney: String,
    pub notes: Option<String>,
}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UpdateCaseData {
    pub id: Option<i32>,
    pub case_number: String,
    pub client_name: String,
    pub case_type: String,
    pub case_status: String,
    pub date_opened: Option<chrono::NaiveDate>,
    pub date_closed: Option<chrono::NaiveDate>,
    pub assigned_attorney: String,
    pub notes: Option<String>,
}

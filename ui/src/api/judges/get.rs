use leptos::Serializable;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


pub fn judge(path: &str) -> String {
    format!("http://localhost:3018/judges/{path}")
}

#[cfg(not(feature = "ssr"))]
pub async fn fetch_api<T>(path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = web_sys::AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if e.g., we've navigated away from this page
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;

    T::de(&json).ok()
}

#[cfg(feature = "ssr")]
pub async fn fetch_api<T>(path: &str) -> Option<T>
where
    T: Serializable,
{
    let json = reqwest::get(path)
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()?
        .text()
        .await
        .ok()?;
    T::de(&json).map_err(|e| log::error!("{e}")).ok()
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Judges {
    pub id: Option<i32>,
    pub nid: Option<String>,
    pub jid: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub suffix: Option<String>,
    pub birth_month: Option<i32>,
    pub birth_day: Option<i32>,
    pub birth_year: Option<i32>,
    pub birth_city: Option<String>,
    pub birth_state: Option<String>,
    pub death_month: Option<i32>,
    pub death_day: Option<i32>,
    pub death_year: Option<i32>,
    pub death_city: Option<String>,
    pub death_state: Option<String>,
    pub gender: Option<String>,
    pub race_or_ethnicity: Option<String>,
    pub court_type_1: Option<String>,
    pub court_name_1: Option<String>,
    pub appointment_title_1: Option<String>,
    pub appointing_president_1: Option<String>,
    pub party_of_appointing_president_1: Option<String>,
    pub reappointing_president_1: Option<String>,
    pub party_of_reappointing_president_1: Option<String>,
    pub aba_rating_1: Option<String>,
    pub seat_id_1: Option<String>,
    pub statute_authorizing_new_seat_1: Option<String>,
    pub recess_appointment_date_1: Option<DateTime<Utc>>,
    pub nomination_date_1: Option<DateTime<Utc>>,
    pub committee_referral_date_1: Option<DateTime<Utc>>,
    pub hearing_date_1: Option<DateTime<Utc>>,
    pub judiciary_committee_action_1: Option<String>,
    pub committee_action_date_1: Option<DateTime<Utc>>,
    pub senate_vote_type_1: Option<String>,
    pub ayes_nays_1: Option<String>,
    pub confirmation_date_1: Option<DateTime<Utc>>,
    pub commission_date_1: Option<DateTime<Utc>>,
    pub service_as_chief_judge_begin_1: Option<String>,
    pub service_as_chief_judge_end_1: Option<String>,
    pub second_service_as_chief_judge_begin_1: Option<String>,
    pub second_service_as_chief_judge_end_1: Option<String>,
    pub senior_status_date_1: Option<DateTime<Utc>>,
    pub termination_1: Option<String>,
    pub termination_date_1: Option<DateTime<Utc>>,
    pub court_type_2: Option<String>,
    pub court_name_2: Option<String>,
    pub appointment_title_2: Option<String>,
    pub appointing_president_2: Option<String>,
    pub party_of_appointing_president_2: Option<String>,
    pub reappointing_president_2: Option<String>,
    pub party_of_reappointing_president_2: Option<String>,
    pub aba_rating_2: Option<String>,
    pub seat_id_2: Option<String>,
    pub statute_authorizing_new_seat_2: Option<String>,
    pub recess_appointment_date_2: Option<DateTime<Utc>>,
    pub nomination_date_2: Option<DateTime<Utc>>,
    pub committee_referral_date_2: Option<DateTime<Utc>>,
    pub hearing_date_2: Option<DateTime<Utc>>,
    pub judiciary_committee_action_2: Option<String>,
    pub committee_action_date_2: Option<DateTime<Utc>>,
    pub senate_vote_type_2: Option<String>,
    pub ayes_nays_2: Option<String>,
    pub confirmation_date_2: Option<DateTime<Utc>>,
    pub commission_date_2: Option<DateTime<Utc>>,
    pub service_as_chief_judge_begin_2: Option<i32>,
    pub service_as_chief_judge_end_2: Option<i32>,
    pub second_service_as_chief_judge_begin_2: Option<i32>,
    pub second_service_as_chief_judge_end_2: Option<i32>,
    pub senior_status_date_2: Option<DateTime<Utc>>,
    pub termination_2: Option<String>,
    pub termination_date_2: Option<DateTime<Utc>>,
    pub court_type_3: Option<String>,
    pub court_name_3: Option<String>,
    pub appointment_title_3: Option<String>,
    pub appointing_president_3: Option<String>,
    pub party_of_appointing_president_3: Option<String>,
    pub reappointing_president_3: Option<String>,
    pub party_of_reappointing_president_3: Option<String>,
    pub aba_rating_3: Option<String>,
    pub seat_id_3: Option<String>,
    pub statute_authorizing_new_seat_3: Option<String>,
    pub recess_appointment_date_3: Option<DateTime<Utc>>,
    pub nomination_date_3: Option<DateTime<Utc>>,
    pub committee_referral_date_3: Option<DateTime<Utc>>,
    pub hearing_date_3: Option<DateTime<Utc>>,
    pub judiciary_committee_action_3: Option<String>,
    pub committee_action_date_3: Option<DateTime<Utc>>,
    pub senate_vote_type_3: Option<String>,
    pub ayes_nays_3: Option<String>,
    pub confirmation_date_3: Option<DateTime<Utc>>,
    pub commission_date_3: Option<String>,
    pub service_as_chief_judge_begin_3: Option<i32>,
    pub service_as_chief_judge_end_3: Option<i32>,
    pub second_service_as_chief_judge_begin_3: Option<i32>,
    pub second_service_as_chief_judge_end_3: Option<i32>,
    pub senior_status_date_3: Option<DateTime<Utc>>,
    pub termination_3: Option<String>,
    pub termination_date_3: Option<DateTime<Utc>>,
    pub court_type_4: Option<String>,
    pub court_name_4: Option<String>,
    pub appointment_title_4: Option<String>,
    pub appointing_president_4: Option<String>,
    pub party_of_appointing_president_4: Option<String>,
    pub reappointing_president_4: Option<String>,
    pub party_of_reappointing_president_4: Option<String>,
    pub aba_rating_4: Option<String>,
    pub seat_id_4: Option<String>,
    pub statute_authorizing_new_seat_4: Option<String>,
    pub recess_appointment_date_4: Option<DateTime<Utc>>,
    pub nomination_date_4: Option<DateTime<Utc>>,
    pub committee_referral_date_4: Option<DateTime<Utc>>,
    pub hearing_date_4: Option<DateTime<Utc>>,
    pub judiciary_committee_action_4: Option<String>,
    pub committee_action_date_4: Option<DateTime<Utc>>,
    pub senate_vote_type_4: Option<String>,
    pub ayes_nays_4: Option<String>,
    pub confirmation_date_4: Option<DateTime<Utc>>,
    pub commission_date_4: Option<DateTime<Utc>>,
    pub service_as_chief_judge_begin_4: Option<i32>,
    pub service_as_chief_judge_end_4: Option<i32>,
    pub second_service_as_chief_judge_begin_4: Option<DateTime<Utc>>,
    pub second_service_as_chief_judge_end_4: Option<DateTime<Utc>>,
    pub senior_status_date_4: Option<DateTime<Utc>>,
    pub termination_4: Option<String>,
    pub termination_date_4: Option<DateTime<Utc>>,
    pub court_type_5: Option<String>,
    pub court_name_5: Option<String>,
    pub appointment_title_5: Option<String>,
    pub appointing_president_5: Option<String>,
    pub party_of_appointing_president_5: Option<String>,
    pub reappointing_president_5: Option<String>,
    pub party_of_reappointing_president_5: Option<String>,
    pub aba_rating_5: Option<String>,
    pub seat_id_5: Option<String>,
    pub statute_authorizing_new_seat_5: Option<String>,
    pub recess_appointment_date_5: Option<DateTime<Utc>>,
    pub nomination_date_5: Option<DateTime<Utc>>,
    pub committee_referral_date_5: Option<DateTime<Utc>>,
    pub hearing_date_5: Option<DateTime<Utc>>,
    pub judiciary_committee_action_5: Option<String>,
    pub committee_action_date_5: Option<DateTime<Utc>>,
    pub senate_vote_type_5: Option<String>,
    pub ayes_nays_5: Option<String>,
    pub confirmation_date_5: Option<DateTime<Utc>>,
    pub commission_date_5: Option<DateTime<Utc>>,
    pub service_as_chief_judge_begin_5: Option<i32>,
    pub service_as_chief_judge_end_5: Option<i32>,
    pub second_service_as_chief_judge_begin_5: Option<DateTime<Utc>>,
    pub second_service_as_chief_judge_end_5: Option<DateTime<Utc>>,
    pub senior_status_date_5: Option<DateTime<Utc>>,
    pub termination_5: Option<String>,
    pub termination_date_5: Option<DateTime<Utc>>,
    pub court_type_6: Option<String>,
    pub court_name_6: Option<String>,
    pub appointment_title_6: Option<String>,
    pub appointing_president_6: Option<String>,
    pub party_of_appointing_president_6: Option<String>,
    pub reappointing_president_6: Option<String>,
    pub party_of_reappointing_president_6: Option<String>,
    pub aba_rating_6: Option<String>,
    pub seat_id_6: Option<String>,
    pub statute_authorizing_new_seat_6: Option<String>,
    pub recess_appointment_date_6: Option<DateTime<Utc>>,
    pub nomination_date_6: Option<DateTime<Utc>>,
    pub committee_referral_date_6: Option<DateTime<Utc>>,
    pub hearing_date_6: Option<DateTime<Utc>>,
    pub judiciary_committee_action_6: Option<String>,
    pub committee_action_date_6: Option<DateTime<Utc>>,
    pub senate_vote_type_6: Option<String>,
    pub ayes_nays_6: Option<String>,
    pub confirmation_date_6: Option<DateTime<Utc>>,
    pub commission_date_6: Option<DateTime<Utc>>,
    pub service_as_chief_judge_begin_6: Option<DateTime<Utc>>,
    pub service_as_chief_judge_end_6: Option<DateTime<Utc>>,
    pub second_service_as_chief_judge_begin_6: Option<DateTime<Utc>>,
    pub second_service_as_chief_judge_end_6: Option<DateTime<Utc>>,
    pub senior_status_date_6: Option<DateTime<Utc>>,
    pub termination_6: Option<String>,
    pub termination_date_6: Option<DateTime<Utc>>,
    pub other_federal_judicial_service_1: Option<String>,
    pub other_federal_judicial_service_2: Option<String>,
    pub other_federal_judicial_service_3: Option<String>,
    pub other_federal_judicial_service_4: Option<String>,
    pub school_1: Option<String>,
    pub degree_1: Option<String>,
    pub degree_year_1: Option<String>,
    pub school_2: Option<String>,
    pub degree_2: Option<String>,
    pub degree_year_2: Option<String>,
    pub school_3: Option<String>,
    pub degree_3: Option<String>,
    pub degree_year_3: Option<String>,
    pub school_4: Option<String>,
    pub degree_4: Option<String>,
    pub degree_year_4: Option<String>,
    pub school_5: Option<String>,
    pub degree_5: Option<String>,
    pub degree_year_5: Option<String>,
    pub professional_career: Option<String>,
    pub other_nominations_recess_appointments: Option<String>,
}


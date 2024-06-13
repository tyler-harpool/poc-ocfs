use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use tracing::{error, info};

use crate::models::{Judges, UpdateJudges};

#[derive(Serialize)]
struct DeleteResponse {
    message: String,
}

pub async fn create_Judges(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Json(input): Json<Judges>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO Judges (
            nid, jid, last_name, first_name, middle_name, suffix,
            birth_month, birth_day, birth_year, birth_city, birth_state,
            death_month, death_day, death_year, death_city, death_state,
            gender, race_or_ethnicity, court_type_1, court_name_1,
            appointment_title_1, appointing_president_1, party_of_appointing_president_1,
            reappointing_president_1, party_of_reappointing_president_1, aba_rating_1,
            seat_id_1, statute_authorizing_new_seat_1, recess_appointment_date_1,
            nomination_date_1, committee_referral_date_1, hearing_date_1,
            judiciary_committee_action_1, committee_action_date_1, senate_vote_type_1,
            ayes_nays_1, confirmation_date_1, commission_date_1,
            service_as_chief_Judges_begin_1, service_as_chief_Judges_end_1,
            second_service_as_chief_Judges_begin_1, second_service_as_chief_Judges_end_1,
            senior_status_date_1, termination_1, termination_date_1,
            court_type_2, court_name_2, appointment_title_2,
            appointing_president_2, party_of_appointing_president_2, reappointing_president_2,
            party_of_reappointing_president_2, aba_rating_2, seat_id_2,
            statute_authorizing_new_seat_2, recess_appointment_date_2, nomination_date_2,
            committee_referral_date_2, hearing_date_2, judiciary_committee_action_2,
            committee_action_date_2, senate_vote_type_2, ayes_nays_2,
            confirmation_date_2, commission_date_2, service_as_chief_Judges_begin_2,
            service_as_chief_Judges_end_2, second_service_as_chief_Judges_begin_2,
            second_service_as_chief_Judges_end_2, senior_status_date_2, termination_2,
            termination_date_2, court_type_3, court_name_3, appointment_title_3,
            appointing_president_3, party_of_appointing_president_3, reappointing_president_3,
            party_of_reappointing_president_3, aba_rating_3, seat_id_3,
            statute_authorizing_new_seat_3, recess_appointment_date_3, nomination_date_3,
            committee_referral_date_3, hearing_date_3, judiciary_committee_action_3,
            committee_action_date_3, senate_vote_type_3, ayes_nays_3,
            confirmation_date_3, commission_date_3, service_as_chief_Judges_begin_3,
            service_as_chief_Judges_end_3, second_service_as_chief_Judges_begin_3,
            second_service_as_chief_Judges_end_3, senior_status_date_3, termination_3,
            termination_date_3, court_type_4, court_name_4, appointment_title_4,
            appointing_president_4, party_of_appointing_president_4, reappointing_president_4,
            party_of_reappointing_president_4, aba_rating_4, seat_id_4,
            statute_authorizing_new_seat_4, recess_appointment_date_4, nomination_date_4,
            committee_referral_date_4, hearing_date_4, judiciary_committee_action_4,
            committee_action_date_4, senate_vote_type_4, ayes_nays_4,
            confirmation_date_4, commission_date_4, service_as_chief_Judges_begin_4,
            service_as_chief_Judges_end_4, second_service_as_chief_Judges_begin_4,
            second_service_as_chief_Judges_end_4, senior_status_date_4, termination_4,
            termination_date_4, court_type_5, court_name_5, appointment_title_5,
            appointing_president_5, party_of_appointing_president_5, reappointing_president_5,
            party_of_reappointing_president_5, aba_rating_5, seat_id_5,
            statute_authorizing_new_seat_5, recess_appointment_date_5, nomination_date_5,
            committee_referral_date_5, hearing_date_5, judiciary_committee_action_5,
            committee_action_date_5, senate_vote_type_5, ayes_nays_5,
            confirmation_date_5, commission_date_5, service_as_chief_Judges_begin_5,
            service_as_chief_Judges_end_5, second_service_as_chief_Judges_begin_5,
            second_service_as_chief_Judges_end_5, senior_status_date_5, termination_5,
            termination_date_5, court_type_6, court_name_6, appointment_title_6,
            appointing_president_6, party_of_appointing_president_6, reappointing_president_6,
            party_of_reappointing_president_6, aba_rating_6, seat_id_6,
            statute_authorizing_new_seat_6, recess_appointment_date_6, nomination_date_6,
            committee_referral_date_6, hearing_date_6, judiciary_committee_action_6,
            committee_action_date_6, senate_vote_type_6, ayes_nays_6,
            confirmation_date_6, commission_date_6, service_as_chief_Judges_begin_6,
            service_as_chief_Judges_end_6, second_service_as_chief_Judges_begin_6,
            second_service_as_chief_Judges_end_6, senior_status_date_6, termination_6,
            termination_date_6, other_federal_judicial_service_1, other_federal_judicial_service_2,
            other_federal_judicial_service_3, other_federal_judicial_service_4,
            school_1, degree_1, degree_year_1, school_2, degree_2, degree_year_2,
            school_3, degree_3, degree_year_3, school_4, degree_4, degree_year_4,
            school_5, degree_5, degree_year_5, professional_career,
            other_nominations_recess_appointments
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16,
            $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31,
            $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42, $43, $44, $45, $46,
            $47, $48, $49, $50, $51, $52, $53, $54, $55, $56, $57, $58, $59, $60, $61,
            $62, $63, $64, $65, $66, $67, $68, $69, $70, $71, $72, $73, $74, $75, $76,
            $77, $78, $79, $80, $81, $82, $83, $84, $85, $86, $87, $88, $89, $90, $91,
            $92, $93, $94, $95, $96, $97, $98, $99, $100, $101, $102, $103, $104, $105,
            $106, $107, $108, $109, $110, $111, $112, $113, $114, $115, $116, $117, $118,
            $119, $120, $121, $122, $123, $124, $125, $126, $127, $128, $129, $130, $131,
            $132, $133, $134, $135, $136, $137, $138, $139, $140, $141, $142, $143, $144,
            $145, $146, $147, $148, $149, $150, $151, $152, $153, $154, $155, $156, $157,
            $158, $159, $160, $161, $162, $163, $164, $165, $166, $167, $168, $169, $170,
            $171, $172, $173, $174, $175, $176, $177, $178, $179, $180, $181, $182, $183,
            $184, $185, $186, $187, $188, $189, $190, $191, $192, $193, $194, $195, $196,
            $197, $198, $199, $200, $201, $202, $203, $204, $205, $206, $207, $208, $209,
            $210, $211, $212, $213, $214, $215
        )
        RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(&input.nid)
        .bind(&input.jid)
        .bind(&input.last_name)
        .bind(&input.first_name)
        .bind(&input.middle_name)
        .bind(&input.suffix)
        .bind(&input.birth_month)
        .bind(&input.birth_day)
        .bind(&input.birth_year)
        .bind(&input.birth_city)
        .bind(&input.birth_state)
        .bind(&input.death_month)
        .bind(&input.death_day)
        .bind(&input.death_year)
        .bind(&input.death_city)
        .bind(&input.death_state)
        .bind(&input.gender)
        .bind(&input.race_or_ethnicity)
        .bind(&input.court_type_1)
        .bind(&input.court_name_1)
        .bind(&input.appointment_title_1)
        .bind(&input.appointing_president_1)
        .bind(&input.party_of_appointing_president_1)
        .bind(&input.reappointing_president_1)
        .bind(&input.party_of_reappointing_president_1)
        .bind(&input.aba_rating_1)
        .bind(&input.seat_id_1)
        .bind(&input.statute_authorizing_new_seat_1)
        .bind(&input.recess_appointment_date_1)
        .bind(&input.nomination_date_1)
        .bind(&input.committee_referral_date_1)
        .bind(&input.hearing_date_1)
        .bind(&input.judiciary_committee_action_1)
        .bind(&input.committee_action_date_1)
        .bind(&input.senate_vote_type_1)
        .bind(&input.ayes_nays_1)
        .bind(&input.confirmation_date_1)
        .bind(&input.commission_date_1)
        .bind(&input.service_as_chief_Judges_begin_1)
        .bind(&input.service_as_chief_Judges_end_1)
        .bind(&input.second_service_as_chief_Judges_begin_1)
        .bind(&input.second_service_as_chief_Judges_end_1)
        .bind(&input.senior_status_date_1)
        .bind(&input.termination_1)
        .bind(&input.termination_date_1)
        .bind(&input.court_type_2)
        .bind(&input.court_name_2)
        .bind(&input.appointment_title_2)
        .bind(&input.appointing_president_2)
        .bind(&input.party_of_appointing_president_2)
        .bind(&input.reappointing_president_2)
        .bind(&input.party_of_reappointing_president_2)
        .bind(&input.aba_rating_2)
        .bind(&input.seat_id_2)
        .bind(&input.statute_authorizing_new_seat_2)
        .bind(&input.recess_appointment_date_2)
        .bind(&input.nomination_date_2)
        .bind(&input.committee_referral_date_2)
        .bind(&input.hearing_date_2)
        .bind(&input.judiciary_committee_action_2)
        .bind(&input.committee_action_date_2)
        .bind(&input.senate_vote_type_2)
        .bind(&input.ayes_nays_2)
        .bind(&input.confirmation_date_2)
        .bind(&input.commission_date_2)
        .bind(&input.service_as_chief_Judges_begin_2)
        .bind(&input.service_as_chief_Judges_end_2)
        .bind(&input.second_service_as_chief_Judges_begin_2)
        .bind(&input.second_service_as_chief_Judges_end_2)
        .bind(&input.senior_status_date_2)
        .bind(&input.termination_2)
        .bind(&input.termination_date_2)
        .bind(&input.court_type_3)
        .bind(&input.court_name_3)
        .bind(&input.appointment_title_3)
        .bind(&input.appointing_president_3)
        .bind(&input.party_of_appointing_president_3)
        .bind(&input.reappointing_president_3)
        .bind(&input.party_of_reappointing_president_3)
        .bind(&input.aba_rating_3)
        .bind(&input.seat_id_3)
        .bind(&input.statute_authorizing_new_seat_3)
        .bind(&input.recess_appointment_date_3)
        .bind(&input.nomination_date_3)
        .bind(&input.committee_referral_date_3)
        .bind(&input.hearing_date_3)
        .bind(&input.judiciary_committee_action_3)
        .bind(&input.committee_action_date_3)
        .bind(&input.senate_vote_type_3)
        .bind(&input.ayes_nays_3)
        .bind(&input.confirmation_date_3)
        .bind(&input.commission_date_3)
        .bind(&input.service_as_chief_Judges_begin_3)
        .bind(&input.service_as_chief_Judges_end_3)
        .bind(&input.second_service_as_chief_Judges_begin_3)
        .bind(&input.second_service_as_chief_Judges_end_3)
        .bind(&input.senior_status_date_3)
        .bind(&input.termination_3)
        .bind(&input.termination_date_3)
        .bind(&input.court_type_4)
        .bind(&input.court_name_4)
        .bind(&input.appointment_title_4)
        .bind(&input.appointing_president_4)
        .bind(&input.party_of_appointing_president_4)
        .bind(&input.reappointing_president_4)
        .bind(&input.party_of_reappointing_president_4)
        .bind(&input.aba_rating_4)
        .bind(&input.seat_id_4)
        .bind(&input.statute_authorizing_new_seat_4)
        .bind(&input.recess_appointment_date_4)
        .bind(&input.nomination_date_4)
        .bind(&input.committee_referral_date_4)
        .bind(&input.hearing_date_4)
        .bind(&input.judiciary_committee_action_4)
        .bind(&input.committee_action_date_4)
        .bind(&input.senate_vote_type_4)
        .bind(&input.ayes_nays_4)
        .bind(&input.confirmation_date_4)
        .bind(&input.commission_date_4)
        .bind(&input.service_as_chief_Judges_begin_4)
        .bind(&input.service_as_chief_Judges_end_4)
        .bind(&input.second_service_as_chief_Judges_begin_4)
        .bind(&input.second_service_as_chief_Judges_end_4)
        .bind(&input.senior_status_date_4)
        .bind(&input.termination_4)
        .bind(&input.termination_date_4)
        .bind(&input.court_type_5)
        .bind(&input.court_name_5)
        .bind(&input.appointment_title_5)
        .bind(&input.appointing_president_5)
        .bind(&input.party_of_appointing_president_5)
        .bind(&input.reappointing_president_5)
        .bind(&input.party_of_reappointing_president_5)
        .bind(&input.aba_rating_5)
        .bind(&input.seat_id_5)
        .bind(&input.statute_authorizing_new_seat_5)
        .bind(&input.recess_appointment_date_5)
        .bind(&input.nomination_date_5)
        .bind(&input.committee_referral_date_5)
        .bind(&input.hearing_date_5)
        .bind(&input.judiciary_committee_action_5)
        .bind(&input.committee_action_date_5)
        .bind(&input.senate_vote_type_5)
        .bind(&input.ayes_nays_5)
        .bind(&input.confirmation_date_5)
        .bind(&input.commission_date_5)
        .bind(&input.service_as_chief_Judges_begin_5)
        .bind(&input.service_as_chief_Judges_end_5)
        .bind(&input.second_service_as_chief_Judges_begin_5)
        .bind(&input.second_service_as_chief_Judges_end_5)
        .bind(&input.senior_status_date_5)
        .bind(&input.termination_5)
        .bind(&input.termination_date_5)
        .bind(&input.court_type_6)
        .bind(&input.court_name_6)
        .bind(&input.appointment_title_6)
        .bind(&input.appointing_president_6)
        .bind(&input.party_of_appointing_president_6)
        .bind(&input.reappointing_president_6)
        .bind(&input.party_of_reappointing_president_6)
        .bind(&input.aba_rating_6)
        .bind(&input.seat_id_6)
        .bind(&input.statute_authorizing_new_seat_6)
        .bind(&input.recess_appointment_date_6)
        .bind(&input.nomination_date_6)
        .bind(&input.committee_referral_date_6)
        .bind(&input.hearing_date_6)
        .bind(&input.judiciary_committee_action_6)
        .bind(&input.committee_action_date_6)
        .bind(&input.senate_vote_type_6)
        .bind(&input.ayes_nays_6)
        .bind(&input.confirmation_date_6)
        .bind(&input.commission_date_6)
        .bind(&input.service_as_chief_Judges_begin_6)
        .bind(&input.service_as_chief_Judges_end_6)
        .bind(&input.second_service_as_chief_Judges_begin_6)
        .bind(&input.second_service_as_chief_Judges_end_6)
        .bind(&input.senior_status_date_6)
        .bind(&input.termination_6)
        .bind(&input.termination_date_6)
        .bind(&input.other_federal_judicial_service_1)
        .bind(&input.other_federal_judicial_service_2)
        .bind(&input.other_federal_judicial_service_3)
        .bind(&input.other_federal_judicial_service_4)
        .bind(&input.school_1)
        .bind(&input.degree_1)
        .bind(&input.degree_year_1)
        .bind(&input.school_2)
        .bind(&input.degree_2)
        .bind(&input.degree_year_2)
        .bind(&input.school_3)
        .bind(&input.degree_3)
        .bind(&input.degree_year_3)
        .bind(&input.school_4)
        .bind(&input.degree_4)
        .bind(&input.degree_year_4)
        .bind(&input.school_5)
        .bind(&input.degree_5)
        .bind(&input.degree_year_5)
        .bind(&input.professional_career)
        .bind(&input.other_nominations_recess_appointments)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => {
            let id: i32 = record.get("id");

            let response_data = Judges {
                id: Some(id),
                nid: input.nid.clone(),
                jid: input.jid.clone(),
                last_name: input.last_name.clone(),
                first_name: input.first_name.clone(),
                middle_name: input.middle_name.clone(),
                suffix: input.suffix.clone(),
                birth_month: input.birth_month.clone(),
                birth_day: input.birth_day.clone(),
                birth_year: input.birth_year.clone(),
                birth_city: input.birth_city.clone(),
                birth_state: input.birth_state.clone(),
                death_month: input.death_month.clone(),
                death_day: input.death_day.clone(),
                death_year: input.death_year.clone(),
                death_city: input.death_city.clone(),
                death_state: input.death_state.clone(),
                gender: input.gender.clone(),
                race_or_ethnicity: input.race_or_ethnicity.clone(),
                court_type_1: input.court_type_1.clone(),
                court_name_1: input.court_name_1.clone(),
                appointment_title_1: input.appointment_title_1.clone(),
                appointing_president_1: input.appointing_president_1.clone(),
                party_of_appointing_president_1: input.party_of_appointing_president_1.clone(),
                reappointing_president_1: input.reappointing_president_1.clone(),
                party_of_reappointing_president_1: input.party_of_reappointing_president_1.clone(),
                aba_rating_1: input.aba_rating_1.clone(),
                seat_id_1: input.seat_id_1.clone(),
                statute_authorizing_new_seat_1: input.statute_authorizing_new_seat_1.clone(),
                recess_appointment_date_1: input.recess_appointment_date_1.clone(),
                nomination_date_1: input.nomination_date_1.clone(),
                committee_referral_date_1: input.committee_referral_date_1.clone(),
                hearing_date_1: input.hearing_date_1.clone(),
                judiciary_committee_action_1: input.judiciary_committee_action_1.clone(),
                committee_action_date_1: input.committee_action_date_1.clone(),
                senate_vote_type_1: input.senate_vote_type_1.clone(),
                ayes_nays_1: input.ayes_nays_1.clone(),
                confirmation_date_1: input.confirmation_date_1.clone(),
                commission_date_1: input.commission_date_1.clone(),
                // service_as_chief_Judges_begin_1: input.service_as_chief_Judges_begin_1.clone(),
                // service_as_chief_Judges_end_1: input.service_as_chief_Judges_end_1.clone(),
                // second_service_as_chief_Judges_begin_1: input.second_service_as_chief_Judges_begin_1.clone(),
                // second_service_as_chief_Judges_end_1: input.second_service_as_chief_Judges_end_1.clone(),
                senior_status_date_1: input.senior_status_date_1.clone(),
                termination_1: input.termination_1.clone(),
                termination_date_1: input.termination_date_1.clone(),
                court_type_2: input.court_type_2.clone(),
                court_name_2: input.court_name_2.clone(),
                appointment_title_2: input.appointment_title_2.clone(),
                appointing_president_2: input.appointing_president_2.clone(),
                party_of_appointing_president_2: input.party_of_appointing_president_2.clone(),
                reappointing_president_2: input.reappointing_president_2.clone(),
                party_of_reappointing_president_2: input.party_of_reappointing_president_2.clone(),
                aba_rating_2: input.aba_rating_2.clone(),
                seat_id_2: input.seat_id_2.clone(),
                statute_authorizing_new_seat_2: input.statute_authorizing_new_seat_2.clone(),
                recess_appointment_date_2: input.recess_appointment_date_2.clone(),
                nomination_date_2: input.nomination_date_2.clone(),
                committee_referral_date_2: input.committee_referral_date_2.clone(),
                hearing_date_2: input.hearing_date_2.clone(),
                judiciary_committee_action_2: input.judiciary_committee_action_2.clone(),
                committee_action_date_2: input.committee_action_date_2.clone(),
                senate_vote_type_2: input.senate_vote_type_2.clone(),
                ayes_nays_2: input.ayes_nays_2.clone(),
                confirmation_date_2: input.confirmation_date_2.clone(),
                commission_date_2: input.commission_date_2.clone(),
                // service_as_chief_Judges_begin_2: input.service_as_chief_Judges_begin_2.clone(),
                // service_as_chief_Judges_end_2: input.service_as_chief_Judges_end_2.clone(),
                // second_service_as_chief_Judges_begin_2: input.second_service_as_chief_Judges_begin_2.clone(),
                // second_service_as_chief_Judges_end_2: input.second_service_as_chief_Judges_end_2.clone(),
                senior_status_date_2: input.senior_status_date_2.clone(),
                termination_2: input.termination_2.clone(),
                termination_date_2: input.termination_date_2.clone(),
                court_type_3: input.court_type_3.clone(),
                court_name_3: input.court_name_3.clone(),
                appointment_title_3: input.appointment_title_3.clone(),
                appointing_president_3: input.appointing_president_3.clone(),
                party_of_appointing_president_3: input.party_of_appointing_president_3.clone(),
                reappointing_president_3: input.reappointing_president_3.clone(),
                party_of_reappointing_president_3: input.party_of_reappointing_president_3.clone(),
                aba_rating_3: input.aba_rating_3.clone(),
                seat_id_3: input.seat_id_3.clone(),
                statute_authorizing_new_seat_3: input.statute_authorizing_new_seat_3.clone(),
                recess_appointment_date_3: input.recess_appointment_date_3.clone(),
                nomination_date_3: input.nomination_date_3.clone(),
                committee_referral_date_3: input.committee_referral_date_3.clone(),
                hearing_date_3: input.hearing_date_3.clone(),
                judiciary_committee_action_3: input.judiciary_committee_action_3.clone(),
                committee_action_date_3: input.committee_action_date_3.clone(),
                senate_vote_type_3: input.senate_vote_type_3.clone(),
                ayes_nays_3: input.ayes_nays_3.clone(),
                confirmation_date_3: input.confirmation_date_3.clone(),
                commission_date_3: input.commission_date_3.clone(),
                // service_as_chief_Judges_begin_3: input.service_as_chief_Judges_begin_3.clone(),
                // service_as_chief_Judges_end_3: input.service_as_chief_Judges_end_3.clone(),
                // second_service_as_chief_Judges_begin_3: input.second_service_as_chief_Judges_begin_3.clone(),
                // second_service_as_chief_Judges_end_3: input.second_service_as_chief_Judges_end_3.clone(),
                senior_status_date_3: input.senior_status_date_3.clone(),
                termination_3: input.termination_3.clone(),
                termination_date_3: input.termination_date_3.clone(),
                court_type_4: input.court_type_4.clone(),
                court_name_4: input.court_name_4.clone(),
                appointment_title_4: input.appointment_title_4.clone(),
                appointing_president_4: input.appointing_president_4.clone(),
                party_of_appointing_president_4: input.party_of_appointing_president_4.clone(),
                reappointing_president_4: input.reappointing_president_4.clone(),
                party_of_reappointing_president_4: input.party_of_reappointing_president_4.clone(),
                aba_rating_4: input.aba_rating_4.clone(),
                seat_id_4: input.seat_id_4.clone(),
                statute_authorizing_new_seat_4: input.statute_authorizing_new_seat_4.clone(),
                recess_appointment_date_4: input.recess_appointment_date_4.clone(),
                nomination_date_4: input.nomination_date_4.clone(),
                committee_referral_date_4: input.committee_referral_date_4.clone(),
                hearing_date_4: input.hearing_date_4.clone(),
                judiciary_committee_action_4: input.judiciary_committee_action_4.clone(),
                committee_action_date_4: input.committee_action_date_4.clone(),
                senate_vote_type_4: input.senate_vote_type_4.clone(),
                ayes_nays_4: input.ayes_nays_4.clone(),
                confirmation_date_4: input.confirmation_date_4.clone(),
                commission_date_4: input.commission_date_4.clone(),
                // service_as_chief_Judges_begin_4: input.service_as_chief_Judges_begin_4.clone(),
                // service_as_chief_Judges_end_4: input.service_as_chief_Judges_end_4.clone(),
                // second_service_as_chief_Judges_begin_4: input.second_service_as_chief_Judges_begin_4.clone(),
                // second_service_as_chief_Judges_end_4: input.second_service_as_chief_Judges_end_4.clone(),
                senior_status_date_4: input.senior_status_date_4.clone(),
                termination_4: input.termination_4.clone(),
                termination_date_4: input.termination_date_4.clone(),
                court_type_5: input.court_type_5.clone(),
                court_name_5: input.court_name_5.clone(),
                appointment_title_5: input.appointment_title_5.clone(),
                appointing_president_5: input.appointing_president_5.clone(),
                party_of_appointing_president_5: input.party_of_appointing_president_5.clone(),
                reappointing_president_5: input.reappointing_president_5.clone(),
                party_of_reappointing_president_5: input.party_of_reappointing_president_5.clone(),
                aba_rating_5: input.aba_rating_5.clone(),
                seat_id_5: input.seat_id_5.clone(),
                statute_authorizing_new_seat_5: input.statute_authorizing_new_seat_5.clone(),
                recess_appointment_date_5: input.recess_appointment_date_5.clone(),
                nomination_date_5: input.nomination_date_5.clone(),
                committee_referral_date_5: input.committee_referral_date_5.clone(),
                hearing_date_5: input.hearing_date_5.clone(),
                judiciary_committee_action_5: input.judiciary_committee_action_5.clone(),
                committee_action_date_5: input.committee_action_date_5.clone(),
                senate_vote_type_5: input.senate_vote_type_5.clone(),
                ayes_nays_5: input.ayes_nays_5.clone(),
                confirmation_date_5: input.confirmation_date_5.clone(),
                commission_date_5: input.commission_date_5.clone(),
                // service_as_chief_Judges_begin_5: input.service_as_chief_Judges_begin_5.clone(),
                // service_as_chief_Judges_end_5: input.service_as_chief_Judges_end_5.clone(),
                // second_service_as_chief_Judges_begin_5: input.second_service_as_chief_Judges_begin_5.clone(),
                // second_service_as_chief_Judges_end_5: input.second_service_as_chief_Judges_end_5.clone(),
                senior_status_date_5: input.senior_status_date_5.clone(),
                termination_5: input.termination_5.clone(),
                termination_date_5: input.termination_date_5.clone(),
                court_type_6: input.court_type_6.clone(),
                court_name_6: input.court_name_6.clone(),
                appointment_title_6: input.appointment_title_6.clone(),
                appointing_president_6: input.appointing_president_6.clone(),
                party_of_appointing_president_6: input.party_of_appointing_president_6.clone(),
                reappointing_president_6: input.reappointing_president_6.clone(),
                party_of_reappointing_president_6: input.party_of_reappointing_president_6.clone(),
                aba_rating_6: input.aba_rating_6.clone(),
                seat_id_6: input.seat_id_6.clone(),
                statute_authorizing_new_seat_6: input.statute_authorizing_new_seat_6.clone(),
                recess_appointment_date_6: input.recess_appointment_date_6.clone(),
                nomination_date_6: input.nomination_date_6.clone(),
                committee_referral_date_6: input.committee_referral_date_6.clone(),
                hearing_date_6: input.hearing_date_6.clone(),
                judiciary_committee_action_6: input.judiciary_committee_action_6.clone(),
                committee_action_date_6: input.committee_action_date_6.clone(),
                senate_vote_type_6: input.senate_vote_type_6.clone(),
                ayes_nays_6: input.ayes_nays_6.clone(),
                confirmation_date_6: input.confirmation_date_6.clone(),
                commission_date_6: input.commission_date_6.clone(),
                // service_as_chief_Judges_begin_6: input.service_as_chief_Judges_begin_6.clone(),
                // service_as_chief_Judges_end_6: input.service_as_chief_Judges_end_6.clone(),
                // second_service_as_chief_Judges_begin_6: input.second_service_as_chief_Judges_begin_6.clone(),
                // second_service_as_chief_Judges_end_6: input.second_service_as_chief_Judges_end_6.clone(),
                senior_status_date_6: input.senior_status_date_6.clone(),
                termination_6: input.termination_6.clone(),
                termination_date_6: input.termination_date_6.clone(),
                other_federal_judicial_service_1: input.other_federal_judicial_service_1.clone(),
                other_federal_judicial_service_2: input.other_federal_judicial_service_2.clone(),
                other_federal_judicial_service_3: input.other_federal_judicial_service_3.clone(),
                other_federal_judicial_service_4: input.other_federal_judicial_service_4.clone(),
                school_1: input.school_1.clone(),
                degree_1: input.degree_1.clone(),
                degree_year_1: input.degree_year_1.clone(),
                school_2: input.school_2.clone(),
                degree_2: input.degree_2.clone(),
                degree_year_2: input.degree_year_2.clone(),
                school_3: input.school_3.clone(),
                degree_3: input.degree_3.clone(),
                degree_year_3: input.degree_year_3.clone(),
                school_4: input.school_4.clone(),
                degree_4: input.degree_4.clone(),
                degree_year_4: input.degree_year_4.clone(),
                school_5: input.school_5.clone(),
                degree_5: input.degree_5.clone(),
                degree_year_5: input.degree_year_5.clone(),
                professional_career: input.professional_career.clone(),
                other_nominations_recess_appointments: input.other_nominations_recess_appointments.clone(),
            };

            (StatusCode::CREATED, Json(response_data)).into_response()
        }
        Err(e) => {
            error!("Failed to create Judges: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_Judges(
    _headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(input): Json<UpdateJudges>,
) -> impl IntoResponse {
    info!("Updating Judges with ID: {}", id);

    let query = r#"
        UPDATE Judges
        SET
            nid = COALESCE($1, nid),
            jid = COALESCE($2, jid),
            last_name = COALESCE($3, last_name),
            first_name = COALESCE($4, first_name),
            middle_name = COALESCE($5, middle_name),
            suffix = COALESCE($6, suffix),
            birth_month = COALESCE($7, birth_month),
            birth_day = COALESCE($8, birth_day),
            birth_year = COALESCE($9, birth_year),
            birth_city = COALESCE($10, birth_city),
            birth_state = COALESCE($11, birth_state),
            death_month = COALESCE($12, death_month),
            death_day = COALESCE($13, death_day),
            death_year = COALESCE($14, death_year),
            death_city = COALESCE($15, death_city),
            death_state = COALESCE($16, death_state),
            gender = COALESCE($17, gender),
            race_or_ethnicity = COALESCE($18, race_or_ethnicity),
            court_type_1 = COALESCE($19, court_type_1),
            court_name_1 = COALESCE($20, court_name_1),
            appointment_title_1 = COALESCE($21, appointment_title_1),
            appointing_president_1 = COALESCE($22, appointing_president_1),
            party_of_appointing_president_1 = COALESCE($23, party_of_appointing_president_1),
            reappointing_president_1 = COALESCE($24, reappointing_president_1),
            party_of_reappointing_president_1 = COALESCE($25, party_of_reappointing_president_1),
            aba_rating_1 = COALESCE($26, aba_rating_1),
            seat_id_1 = COALESCE($27, seat_id_1),
            statute_authorizing_new_seat_1 = COALESCE($28, statute_authorizing_new_seat_1),
            recess_appointment_date_1 = COALESCE($29, recess_appointment_date_1),
            nomination_date_1 = COALESCE($30, nomination_date_1),
            committee_referral_date_1 = COALESCE($31, committee_referral_date_1),
            hearing_date_1 = COALESCE($32, hearing_date_1),
            judiciary_committee_action_1 = COALESCE($33, judiciary_committee_action_1),
            committee_action_date_1 = COALESCE($34, committee_action_date_1),
            senate_vote_type_1 = COALESCE($35, senate_vote_type_1),
            ayes_nays_1 = COALESCE($36, ayes_nays_1),
            confirmation_date_1 = COALESCE($37, confirmation_date_1),
            commission_date_1 = COALESCE($38, commission_date_1),
            service_as_chief_Judges_begin_1 = COALESCE($39, service_as_chief_Judges_begin_1),
            service_as_chief_Judges_end_1 = COALESCE($40, service_as_chief_Judges_end_1),
            second_service_as_chief_Judges_begin_1 = COALESCE($41, second_service_as_chief_Judges_begin_1),
            second_service_as_chief_Judges_end_1 = COALESCE($42, second_service_as_chief_Judges_end_1),
            senior_status_date_1 = COALESCE($43, senior_status_date_1),
            termination_1 = COALESCE($44, termination_1),
            termination_date_1 = COALESCE($45, termination_date_1),
            court_type_2 = COALESCE($46, court_type_2),
            court_name_2 = COALESCE($47, court_name_2),
            appointment_title_2 = COALESCE($48, appointment_title_2),
            appointing_president_2 = COALESCE($49, appointing_president_2),
            party_of_appointing_president_2 = COALESCE($50, party_of_appointing_president_2),
            reappointing_president_2 = COALESCE($51, reappointing_president_2),
            party_of_reappointing_president_2 = COALESCE($52, party_of_reappointing_president_2),
            aba_rating_2 = COALESCE($53, aba_rating_2),
            seat_id_2 = COALESCE($54, seat_id_2),
            statute_authorizing_new_seat_2 = COALESCE($55, statute_authorizing_new_seat_2),
            recess_appointment_date_2 = COALESCE($56, recess_appointment_date_2),
            nomination_date_2 = COALESCE($57, nomination_date_2),
            committee_referral_date_2 = COALESCE($58, committee_referral_date_2),
            hearing_date_2 = COALESCE($59, hearing_date_2),
            judiciary_committee_action_2 = COALESCE($60, judiciary_committee_action_2),
            committee_action_date_2 = COALESCE($61, committee_action_date_2),
            senate_vote_type_2 = COALESCE($62, senate_vote_type_2),
            ayes_nays_2 = COALESCE($63, ayes_nays_2),
            confirmation_date_2 = COALESCE($64, confirmation_date_2),
            commission_date_2 = COALESCE($65, commission_date_2),
            service_as_chief_Judges_begin_2 = COALESCE($66, service_as_chief_Judges_begin_2),
            service_as_chief_Judges_end_2 = COALESCE($67, service_as_chief_Judges_end_2),
            second_service_as_chief_Judges_begin_2 = COALESCE($68, second_service_as_chief_Judges_begin_2),
            second_service_as_chief_Judges_end_2 = COALESCE($69, second_service_as_chief_Judges_end_2),
            senior_status_date_2 = COALESCE($70, senior_status_date_2),
            termination_2 = COALESCE($71, termination_2),
            termination_date_2 = COALESCE($72, termination_date_2),
            court_type_3 = COALESCE($73, court_type_3),
            court_name_3 = COALESCE($74, court_name_3),
            appointment_title_3 = COALESCE($75, appointment_title_3),
            appointing_president_3 = COALESCE($76, appointing_president_3),
            party_of_appointing_president_3 = COALESCE($77, party_of_appointing_president_3),
            reappointing_president_3 = COALESCE($78, reappointing_president_3),
            party_of_reappointing_president_3 = COALESCE($79, party_of_reappointing_president_3),
            aba_rating_3 = COALESCE($80, aba_rating_3),
            seat_id_3 = COALESCE($81, seat_id_3),
            statute_authorizing_new_seat_3 = COALESCE($82, statute_authorizing_new_seat_3),
            recess_appointment_date_3 = COALESCE($83, recess_appointment_date_3),
            nomination_date_3 = COALESCE($84, nomination_date_3),
            committee_referral_date_3 = COALESCE($85, committee_referral_date_3),
            hearing_date_3 = COALESCE($86, hearing_date_3),
            judiciary_committee_action_3 = COALESCE($87, judiciary_committee_action_3),
            committee_action_date_3 = COALESCE($88, committee_action_date_3),
            senate_vote_type_3 = COALESCE($89, senate_vote_type_3),
            ayes_nays_3 = COALESCE($90, ayes_nays_3),
            confirmation_date_3 = COALESCE($91, confirmation_date_3),
            commission_date_3 = COALESCE($92, commission_date_3),
            service_as_chief_Judges_begin_3 = COALESCE($93, service_as_chief_Judges_begin_3),
            service_as_chief_Judges_end_3 = COALESCE($94, service_as_chief_Judges_end_3),
            second_service_as_chief_Judges_begin_3 = COALESCE($95, second_service_as_chief_Judges_begin_3),
            second_service_as_chief_Judges_end_3 = COALESCE($96, second_service_as_chief_Judges_end_3),
            senior_status_date_3 = COALESCE($97, senior_status_date_3),
            termination_3 = COALESCE($98, termination_3),
            termination_date_3 = COALESCE($99, termination_date_3),
            court_type_4 = COALESCE($100, court_type_4),
            court_name_4 = COALESCE($101, court_name_4),
            appointment_title_4 = COALESCE($102, appointment_title_4),
            appointing_president_4 = COALESCE($103, appointing_president_4),
            party_of_appointing_president_4 = COALESCE($104, party_of_appointing_president_4),
            reappointing_president_4 = COALESCE($105, reappointing_president_4),
            party_of_reappointing_president_4 = COALESCE($106, party_of_reappointing_president_4),
            aba_rating_4 = COALESCE($107, aba_rating_4),
            seat_id_4 = COALESCE($108, seat_id_4),
            statute_authorizing_new_seat_4 = COALESCE($109, statute_authorizing_new_seat_4),
            recess_appointment_date_4 = COALESCE($110, recess_appointment_date_4),
            nomination_date_4 = COALESCE($111, nomination_date_4),
            committee_referral_date_4 = COALESCE($112, committee_referral_date_4),
            hearing_date_4 = COALESCE($113, hearing_date_4),
            judiciary_committee_action_4 = COALESCE($114, judiciary_committee_action_4),
            committee_action_date_4 = COALESCE($115, committee_action_date_4),
            senate_vote_type_4 = COALESCE($116, senate_vote_type_4),
            ayes_nays_4 = COALESCE($117, ayes_nays_4),
            confirmation_date_4 = COALESCE($118, confirmation_date_4),
            commission_date_4 = COALESCE($119, commission_date_4),
            service_as_chief_Judges_begin_4 = COALESCE($120, service_as_chief_Judges_begin_4),
            service_as_chief_Judges_end_4 = COALESCE($121, service_as_chief_Judges_end_4),
            second_service_as_chief_Judges_begin_4 = COALESCE($122, second_service_as_chief_Judges_begin_4),
            second_service_as_chief_Judges_end_4 = COALESCE($123, second_service_as_chief_Judges_end_4),
            senior_status_date_4 = COALESCE($124, senior_status_date_4),
            termination_4 = COALESCE($125, termination_4),
            termination_date_4 = COALESCE($126, termination_date_4),
            court_type_5 = COALESCE($127, court_type_5),
            court_name_5 = COALESCE($128, court_name_5),
            appointment_title_5 = COALESCE($129, appointment_title_5),
            appointing_president_5 = COALESCE($130, appointing_president_5),
            party_of_appointing_president_5 = COALESCE($131, party_of_appointing_president_5),
            reappointing_president_5 = COALESCE($132, reappointing_president_5),
            party_of_reappointing_president_5 = COALESCE($133, party_of_reappointing_president_5),
            aba_rating_5 = COALESCE($134, aba_rating_5),
            seat_id_5 = COALESCE($135, seat_id_5),
            statute_authorizing_new_seat_5 = COALESCE($136, statute_authorizing_new_seat_5),
            recess_appointment_date_5 = COALESCE($137, recess_appointment_date_5),
            nomination_date_5 = COALESCE($138, nomination_date_5),
            committee_referral_date_5 = COALESCE($139, committee_referral_date_5),
            hearing_date_5 = COALESCE($140, hearing_date_5),
            judiciary_committee_action_5 = COALESCE($141, judiciary_committee_action_5),
            committee_action_date_5 = COALESCE($142, committee_action_date_5),
            senate_vote_type_5 = COALESCE($143, senate_vote_type_5),
            ayes_nays_5 = COALESCE($144, ayes_nays_5),
            confirmation_date_5 = COALESCE($145, confirmation_date_5),
            commission_date_5 = COALESCE($146, commission_date_5),
            service_as_chief_Judges_begin_5 = COALESCE($147, service_as_chief_Judges_begin_5),
            service_as_chief_Judges_end_5 = COALESCE($148, service_as_chief_Judges_end_5),
            second_service_as_chief_Judges_begin_5 = COALESCE($149, second_service_as_chief_Judges_begin_5),
            second_service_as_chief_Judges_end_5 = COALESCE($150, second_service_as_chief_Judges_end_5),
            senior_status_date_5 = COALESCE($151, senior_status_date_5),
            termination_5 = COALESCE($152, termination_5),
            termination_date_5 = COALESCE($153, termination_date_5),
            court_type_6 = COALESCE($154, court_type_6),
            court_name_6 = COALESCE($155, court_name_6),
            appointment_title_6 = COALESCE($156, appointment_title_6),
            appointing_president_6 = COALESCE($157, appointing_president_6),
            party_of_appointing_president_6 = COALESCE($158, party_of_appointing_president_6),
            reappointing_president_6 = COALESCE($159, reappointing_president_6),
            party_of_reappointing_president_6 = COALESCE($160, party_of_reappointing_president_6),
            aba_rating_6 = COALESCE($161, aba_rating_6),
            seat_id_6 = COALESCE($162, seat_id_6),
            statute_authorizing_new_seat_6 = COALESCE($163, statute_authorizing_new_seat_6),
            recess_appointment_date_6 = COALESCE($164, recess_appointment_date_6),
            nomination_date_6 = COALESCE($165, nomination_date_6),
            committee_referral_date_6 = COALESCE($166, committee_referral_date_6),
            hearing_date_6 = COALESCE($167, hearing_date_6),
            judiciary_committee_action_6 = COALESCE($168, judiciary_committee_action_6),
            committee_action_date_6 = COALESCE($169, committee_action_date_6),
            senate_vote_type_6 = COALESCE($170, senate_vote_type_6),
            ayes_nays_6 = COALESCE($171, ayes_nays_6),
            confirmation_date_6 = COALESCE($172, confirmation_date_6),
            commission_date_6 = COALESCE($173, commission_date_6),
            service_as_chief_Judges_begin_6 = COALESCE($174, service_as_chief_Judges_begin_6),
            service_as_chief_Judges_end_6 = COALESCE($175, service_as_chief_Judges_end_6),
            second_service_as_chief_Judges_begin_6 = COALESCE($176, second_service_as_chief_Judges_begin_6),
            second_service_as_chief_Judges_end_6 = COALESCE($177, second_service_as_chief_Judges_end_6),
            senior_status_date_6 = COALESCE($178, senior_status_date_6),
            termination_6 = COALESCE($179, termination_6),
            termination_date_6 = COALESCE($180, termination_date_6),
            other_federal_judicial_service_1 = COALESCE($181, other_federal_judicial_service_1),
            other_federal_judicial_service_2 = COALESCE($182, other_federal_judicial_service_2),
            other_federal_judicial_service_3 = COALESCE($183, other_federal_judicial_service_3),
            other_federal_judicial_service_4 = COALESCE($184, other_federal_judicial_service_4),
            school_1 = COALESCE($185, school_1),
            degree_1 = COALESCE($186, degree_1),
            degree_year_1 = COALESCE($187, degree_year_1),
            school_2 = COALESCE($188, school_2),
            degree_2 = COALESCE($189, degree_2),
            degree_year_2 = COALESCE($190, degree_year_2),
            school_3 = COALESCE($191, school_3),
            degree_3 = COALESCE($192, degree_3),
            degree_year_3 = COALESCE($193, degree_year_3),
            school_4 = COALESCE($194, school_4),
            degree_4 = COALESCE($195, degree_4),
            degree_year_4 = COALESCE($196, degree_year_4),
            school_5 = COALESCE($197, school_5),
            degree_5 = COALESCE($198, degree_5),
            degree_year_5 = COALESCE($199, degree_year_5),
            professional_career = COALESCE($200, professional_career),
            other_nominations_recess_appointments = COALESCE($201, other_nominations_recess_appointments)
        WHERE id = $202
    "#;

    match sqlx::query(query)
        .bind(input.nid)
        .bind(input.jid)
        .bind(input.last_name)
        .bind(input.first_name)
        .bind(input.middle_name)
        .bind(input.suffix)
        .bind(input.birth_month)
        .bind(input.birth_day)
        .bind(input.birth_year)
        .bind(input.birth_city)
        .bind(input.birth_state)
        .bind(input.death_month)
        .bind(input.death_day)
        .bind(input.death_year)
        .bind(input.death_city)
        .bind(input.death_state)
        .bind(input.gender)
        .bind(input.race_or_ethnicity)
        .bind(input.court_type_1)
        .bind(input.court_name_1)
        .bind(input.appointment_title_1)
        .bind(input.appointing_president_1)
        .bind(input.party_of_appointing_president_1)
        .bind(input.reappointing_president_1)
        .bind(input.party_of_reappointing_president_1)
        .bind(input.aba_rating_1)
        .bind(input.seat_id_1)
        .bind(input.statute_authorizing_new_seat_1)
        .bind(input.recess_appointment_date_1)
        .bind(input.nomination_date_1)
        .bind(input.committee_referral_date_1)
        .bind(input.hearing_date_1)
        .bind(input.judiciary_committee_action_1)
        .bind(input.committee_action_date_1)
        .bind(input.senate_vote_type_1)
        .bind(input.ayes_nays_1)
        .bind(input.confirmation_date_1)
        .bind(input.commission_date_1)
        .bind(input.service_as_chief_Judges_begin_1)
        .bind(input.service_as_chief_Judges_end_1)
        .bind(input.second_service_as_chief_Judges_begin_1)
        .bind(input.second_service_as_chief_Judges_end_1)
        .bind(input.senior_status_date_1)
        .bind(input.termination_1)
        .bind(input.termination_date_1)
        .bind(input.court_type_2)
        .bind(input.court_name_2)
        .bind(input.appointment_title_2)
        .bind(input.appointing_president_2)
        .bind(input.party_of_appointing_president_2)
        .bind(input.reappointing_president_2)
        .bind(input.party_of_reappointing_president_2)
        .bind(input.aba_rating_2)
        .bind(input.seat_id_2)
        .bind(input.statute_authorizing_new_seat_2)
        .bind(input.recess_appointment_date_2)
        .bind(input.nomination_date_2)
        .bind(input.committee_referral_date_2)
        .bind(input.hearing_date_2)
        .bind(input.judiciary_committee_action_2)
        .bind(input.committee_action_date_2)
        .bind(input.senate_vote_type_2)
        .bind(input.ayes_nays_2)
        .bind(input.confirmation_date_2)
        .bind(input.commission_date_2)
        .bind(input.service_as_chief_Judges_begin_2)
        .bind(input.service_as_chief_Judges_end_2)
        .bind(input.second_service_as_chief_Judges_begin_2)
        .bind(input.second_service_as_chief_Judges_end_2)
        .bind(input.senior_status_date_2)
        .bind(input.termination_2)
        .bind(input.termination_date_2)
        .bind(input.court_type_3)
        .bind(input.court_name_3)
        .bind(input.appointment_title_3)
        .bind(input.appointing_president_3)
        .bind(input.party_of_appointing_president_3)
        .bind(input.reappointing_president_3)
        .bind(input.party_of_reappointing_president_3)
        .bind(input.aba_rating_3)
        .bind(input.seat_id_3)
        .bind(input.statute_authorizing_new_seat_3)
        .bind(input.recess_appointment_date_3)
        .bind(input.nomination_date_3)
        .bind(input.committee_referral_date_3)
        .bind(input.hearing_date_3)
        .bind(input.judiciary_committee_action_3)
        .bind(input.committee_action_date_3)
        .bind(input.senate_vote_type_3)
        .bind(input.ayes_nays_3)
        .bind(input.confirmation_date_3)
        .bind(input.commission_date_3)
        .bind(input.service_as_chief_Judges_begin_3)
        .bind(input.service_as_chief_Judges_end_3)
        .bind(input.second_service_as_chief_Judges_begin_3)
        .bind(input.second_service_as_chief_Judges_end_3)
        .bind(input.senior_status_date_3)
        .bind(input.termination_3)
        .bind(input.termination_date_3)
        .bind(input.court_type_4)
        .bind(input.court_name_4)
        .bind(input.appointment_title_4)
        .bind(input.appointing_president_4)
        .bind(input.party_of_appointing_president_4)
        .bind(input.reappointing_president_4)
        .bind(input.party_of_reappointing_president_4)
        .bind(input.aba_rating_4)
        .bind(input.seat_id_4)
        .bind(input.statute_authorizing_new_seat_4)
        .bind(input.recess_appointment_date_4)
        .bind(input.nomination_date_4)
        .bind(input.committee_referral_date_4)
        .bind(input.hearing_date_4)
        .bind(input.judiciary_committee_action_4)
        .bind(input.committee_action_date_4)
        .bind(input.senate_vote_type_4)
        .bind(input.ayes_nays_4)
        .bind(input.confirmation_date_4)
        .bind(input.commission_date_4)
        .bind(input.service_as_chief_Judges_begin_4)
        .bind(input.service_as_chief_Judges_end_4)
        .bind(input.second_service_as_chief_Judges_begin_4)
        .bind(input.second_service_as_chief_Judges_end_4)
        .bind(input.senior_status_date_4)
        .bind(input.termination_4)
        .bind(input.termination_date_4)
        .bind(input.court_type_5)
        .bind(input.court_name_5)
        .bind(input.appointment_title_5)
        .bind(input.appointing_president_5)
        .bind(input.party_of_appointing_president_5)
        .bind(input.reappointing_president_5)
        .bind(input.party_of_reappointing_president_5)
        .bind(input.aba_rating_5)
        .bind(input.seat_id_5)
        .bind(input.statute_authorizing_new_seat_5)
        .bind(input.recess_appointment_date_5)
        .bind(input.nomination_date_5)
        .bind(input.committee_referral_date_5)
        .bind(input.hearing_date_5)
        .bind(input.judiciary_committee_action_5)
        .bind(input.committee_action_date_5)
        .bind(input.senate_vote_type_5)
        .bind(input.ayes_nays_5)
        .bind(input.confirmation_date_5)
        .bind(input.commission_date_5)
        .bind(input.service_as_chief_Judges_begin_5)
        .bind(input.service_as_chief_Judges_end_5)
        .bind(input.second_service_as_chief_Judges_begin_5)
        .bind(input.second_service_as_chief_Judges_end_5)
        .bind(input.senior_status_date_5)
        .bind(input.termination_5)
        .bind(input.termination_date_5)
        .bind(input.court_type_6)
        .bind(input.court_name_6)
        .bind(input.appointment_title_6)
        .bind(input.appointing_president_6)
        .bind(input.party_of_appointing_president_6)
        .bind(input.reappointing_president_6)
        .bind(input.party_of_reappointing_president_6)
        .bind(input.aba_rating_6)
        .bind(input.seat_id_6)
        .bind(input.statute_authorizing_new_seat_6)
        .bind(input.recess_appointment_date_6)
        .bind(input.nomination_date_6)
        .bind(input.committee_referral_date_6)
        .bind(input.hearing_date_6)
        .bind(input.judiciary_committee_action_6)
        .bind(input.committee_action_date_6)
        .bind(input.senate_vote_type_6)
        .bind(input.ayes_nays_6)
        .bind(input.confirmation_date_6)
        .bind(input.commission_date_6)
        .bind(input.service_as_chief_Judges_begin_6)
        .bind(input.service_as_chief_Judges_end_6)
        .bind(input.second_service_as_chief_Judges_begin_6)
        .bind(input.second_service_as_chief_Judges_end_6)
        .bind(input.senior_status_date_6)
        .bind(input.termination_6)
        .bind(input.termination_date_6)
        .bind(input.other_federal_judicial_service_1)
        .bind(input.other_federal_judicial_service_2)
        .bind(input.other_federal_judicial_service_3)
        .bind(input.other_federal_judicial_service_4)
        .bind(input.school_1)
        .bind(input.degree_1)
        .bind(input.degree_year_1)
        .bind(input.school_2)
        .bind(input.degree_2)
        .bind(input.degree_year_2)
        .bind(input.school_3)
        .bind(input.degree_3)
        .bind(input.degree_year_3)
        .bind(input.school_4)
        .bind(input.degree_4)
        .bind(input.degree_year_4)
        .bind(input.school_5)
        .bind(input.degree_5)
        .bind(input.degree_year_5)
        .bind(input.professional_career)
        .bind(input.other_nominations_recess_appointments)
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("No Judges found with ID: {}", id);
                (StatusCode::NOT_FOUND, "No Judges found").into_response()
            } else {
                info!("Successfully updated Judges with ID: {}", id);
                (StatusCode::OK, "Judges updated successfully").into_response()
            }
        }
        Err(e) => {
            error!("Failed to update Judges with ID: {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update Judges").into_response()
        }
    }
}

pub async fn get_Judges(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = "SELECT * FROM Judges WHERE id = $1";

    match sqlx::query_as::<_, Judges>(query)
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(Judges) => (StatusCode::OK, Json(Judges)).into_response(),
        Err(e) => {
            error!("Failed to fetch Judges for id {}: {:?}", id, e);
            (StatusCode::NOT_FOUND, Json("Judges not found")).into_response()
        }
    }
}

pub async fn delete_Judges(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = "DELETE FROM Judges WHERE id = $1";

    match sqlx::query(query).bind(id).execute(&pool).await {
        Ok(result) => {
            let affected = result.rows_affected();
            if affected == 0 {
                error!("Judges not found with ID: {}", id);
                (
                    StatusCode::NOT_FOUND,
                    Json(DeleteResponse {
                        message: format!("Judges not found with ID: {}", id),
                    }),
                )
                    .into_response()
            } else {
                info!(
                    "Successfully deleted {} Judges record(s) with ID: {}",
                    affected, id
                );
                (
                    StatusCode::NO_CONTENT,
                    Json(DeleteResponse {
                        message: format!("Successfully deleted Judges with ID: {}", id),
                    }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            error!("Failed to delete Judges with ID: {}: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteResponse {
                    message: format!("Failed to delete Judges with ID: {}", id),
                }),
            )
                .into_response()
        }
    }
}

pub async fn list_all_Judges(
    _headers: HeaderMap, // Prefix with an underscore if not used
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let query = "SELECT * FROM Judges LIMIT 0, 100";

    match sqlx::query_as::<_, Judges>(query).fetch_all(&pool).await {
        Ok(Judges_list) => {
            let total_records = Judges_list.len();
            info!("Listing all Judges: total records = {}", total_records);

            (StatusCode::OK, Json(Judges_list)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch Judges list: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to fetch Judges list"),
            )
                .into_response()
        }
    }
}

-- Add down migration script here
CREATE TABLE IF NOT EXISTS Judges (
                        id SERIAL PRIMARY KEY,
                        nid TEXT,
                        jid TEXT,
                        last_name VARCHAR(255),
                        first_name VARCHAR(255),
                        middle_name VARCHAR(255),
                        suffix VARCHAR(50),
                        birth_month INT,
                        birth_day INT,
                        birth_year INT,
                        birth_city VARCHAR(255),
                        birth_state VARCHAR(255),
                        death_month INT,
                        death_day INT,
                        death_year INT,
                        death_city VARCHAR(255),
                        death_state VARCHAR(255),
                        gender VARCHAR(50),
                        race_or_ethnicity VARCHAR(255),
                        court_type_1 VARCHAR(255),
                        court_name_1 VARCHAR(255),
                        appointment_title_1 VARCHAR(255),
                        appointing_president_1 VARCHAR(255),
                        party_of_appointing_president_1 VARCHAR(255),
                        reappointing_president_1 VARCHAR(255),
                        party_of_reappointing_president_1 VARCHAR(255),
                        aba_rating_1 VARCHAR(255),
                        seat_id_1 TEXT,
                        statute_authorizing_new_seat_1 VARCHAR(255),
                        recess_appointment_date_1 TIMESTAMPTZ,
                        nomination_date_1 TIMESTAMPTZ,
                        committee_referral_date_1 TIMESTAMPTZ,
                        hearing_date_1 TIMESTAMPTZ,
                        judiciary_committee_action_1 VARCHAR(255),
                        committee_action_date_1 TIMESTAMPTZ,
                        senate_vote_type_1 VARCHAR(255),
                        ayes_nays_1 VARCHAR(255),
                        confirmation_date_1 TIMESTAMPTZ,
                        commission_date_1 TIMESTAMPTZ,
                        service_as_chief_judge_begin_1 TEXT,
                        service_as_chief_judge_end_1 TEXT,
                        second_service_as_chief_judge_begin_1 TEXT,
                        second_service_as_chief_judge_end_1 TEXT,
                        senior_status_date_1 TIMESTAMPTZ,
                        termination_1 VARCHAR(255),
                        termination_date_1 TIMESTAMPTZ,
                        court_type_2 VARCHAR(255),
                        court_name_2 VARCHAR(255),
                        appointment_title_2 VARCHAR(255),
                        appointing_president_2 VARCHAR(255),
                        party_of_appointing_president_2 VARCHAR(255),
                        reappointing_president_2 VARCHAR(255),
                        party_of_reappointing_president_2 VARCHAR(255),
                        aba_rating_2 VARCHAR(255),
                        seat_id_2 TEXT,
                        statute_authorizing_new_seat_2 VARCHAR(255),
                        recess_appointment_date_2 TIMESTAMPTZ,
                        nomination_date_2 TIMESTAMPTZ,
                        committee_referral_date_2 TIMESTAMPTZ,
                        hearing_date_2 TIMESTAMPTZ,
                        judiciary_committee_action_2 VARCHAR(255),
                        committee_action_date_2 TIMESTAMPTZ,
                        senate_vote_type_2 VARCHAR(255),
                        ayes_nays_2 VARCHAR(255),
                        confirmation_date_2 TIMESTAMPTZ,
                        commission_date_2 TIMESTAMPTZ,
                        service_as_chief_judge_begin_2 INT,
                        service_as_chief_judge_end_2 INT,
                        second_service_as_chief_judge_begin_2 INT,
                        second_service_as_chief_judge_end_2 INT,
                        senior_status_date_2 TIMESTAMPTZ,
                        termination_2 VARCHAR(255),
                        termination_date_2 TIMESTAMPTZ,
                        court_type_3 VARCHAR(255),
                        court_name_3 VARCHAR(255),
                        appointment_title_3 VARCHAR(255),
                        appointing_president_3 VARCHAR(255),
                        party_of_appointing_president_3 VARCHAR(255),
                        reappointing_president_3 VARCHAR(255),
                        party_of_reappointing_president_3 VARCHAR(255),
                        aba_rating_3 VARCHAR(255),
                        seat_id_3 TEXT,
                        statute_authorizing_new_seat_3 VARCHAR(255),
                        recess_appointment_date_3 TIMESTAMPTZ,
                        nomination_date_3 TIMESTAMPTZ,
                        committee_referral_date_3 TIMESTAMPTZ,
                        hearing_date_3 TIMESTAMPTZ,
                        judiciary_committee_action_3 VARCHAR(255),
                        committee_action_date_3 TIMESTAMPTZ,
                        senate_vote_type_3 VARCHAR(255),
                        ayes_nays_3 VARCHAR(255),
                        confirmation_date_3 TIMESTAMPTZ,
                        commission_date_3 TEXT,
                        service_as_chief_judge_begin_3 INT,
                        service_as_chief_judge_end_3 INT,
                        second_service_as_chief_judge_begin_3 INT,
                        second_service_as_chief_judge_end_3 INT,
                        senior_status_date_3 TIMESTAMPTZ,
                        termination_3 VARCHAR(255),
                        termination_date_3 TIMESTAMPTZ,
                        court_type_4 VARCHAR(255),
                        court_name_4 VARCHAR(255),
                        appointment_title_4 VARCHAR(255),
                        appointing_president_4 VARCHAR(255),
                        party_of_appointing_president_4 VARCHAR(255),
                        reappointing_president_4 VARCHAR(255),
                        party_of_reappointing_president_4 VARCHAR(255),
                        aba_rating_4 VARCHAR(255),
                        seat_id_4 TEXT,
                        statute_authorizing_new_seat_4 VARCHAR(255),
                        recess_appointment_date_4 TIMESTAMPTZ,
                        nomination_date_4 TIMESTAMPTZ,
                        committee_referral_date_4 TIMESTAMPTZ,
                        hearing_date_4 TIMESTAMPTZ,
                        judiciary_committee_action_4 VARCHAR(255),
                        committee_action_date_4 TIMESTAMPTZ,
                        senate_vote_type_4 VARCHAR(255),
                        ayes_nays_4 VARCHAR(255),
                        confirmation_date_4 TIMESTAMPTZ,
                        commission_date_4 TIMESTAMPTZ,
                        service_as_chief_judge_begin_4 INT,
                        service_as_chief_judge_end_4 INT,
                        second_service_as_chief_judge_begin_4 TIMESTAMPTZ,
                        second_service_as_chief_judge_end_4 TIMESTAMPTZ,
                        senior_status_date_4 TIMESTAMPTZ,
                        termination_4 VARCHAR(255),
                        termination_date_4 TIMESTAMPTZ,
                        court_type_5 VARCHAR(255),
                        court_name_5 VARCHAR(255),
                        appointment_title_5 VARCHAR(255),
                        appointing_president_5 VARCHAR(255),
                        party_of_appointing_president_5 VARCHAR(255),
                        reappointing_president_5 VARCHAR(255),
                        party_of_reappointing_president_5 VARCHAR(255),
                        aba_rating_5 VARCHAR(255),
                        seat_id_5 TEXT,
                        statute_authorizing_new_seat_5 VARCHAR(255),
                        recess_appointment_date_5 TIMESTAMPTZ,
                        nomination_date_5 TIMESTAMPTZ,
                        committee_referral_date_5 TIMESTAMPTZ,
                        hearing_date_5 TIMESTAMPTZ,
                        judiciary_committee_action_5 VARCHAR(255),
                        committee_action_date_5 TIMESTAMPTZ,
                        senate_vote_type_5 VARCHAR(255),
                        ayes_nays_5 VARCHAR(255),
                        confirmation_date_5 TIMESTAMPTZ,
                        commission_date_5 TIMESTAMPTZ,
                        service_as_chief_judge_begin_5 INT,
                        service_as_chief_judge_end_5 INT,
                        second_service_as_chief_judge_begin_5 TIMESTAMPTZ,
                        second_service_as_chief_judge_end_5 TIMESTAMPTZ,
                        senior_status_date_5 TIMESTAMPTZ,
                        termination_5 VARCHAR(255),
                        termination_date_5 TIMESTAMPTZ,
                        court_type_6 VARCHAR(255),
                        court_name_6 VARCHAR(255),
                        appointment_title_6 VARCHAR(255),
                        appointing_president_6 VARCHAR(255),
                        party_of_appointing_president_6 VARCHAR(255),
                        reappointing_president_6 VARCHAR(255),
                        party_of_reappointing_president_6 VARCHAR(255),
                        aba_rating_6 VARCHAR(255),
                        seat_id_6 TEXT,
                        statute_authorizing_new_seat_6 VARCHAR(255),
                        recess_appointment_date_6 TIMESTAMPTZ,
                        nomination_date_6 TIMESTAMPTZ,
                        committee_referral_date_6 TIMESTAMPTZ,
                        hearing_date_6 TIMESTAMPTZ,
                        judiciary_committee_action_6 VARCHAR(255),
                        committee_action_date_6 TIMESTAMPTZ,
                        senate_vote_type_6 VARCHAR(255),
                        ayes_nays_6 VARCHAR(255),
                        confirmation_date_6 TIMESTAMPTZ,
                        commission_date_6 TIMESTAMPTZ,
                        service_as_chief_judge_begin_6 TIMESTAMPTZ,
                        service_as_chief_judge_end_6 TIMESTAMPTZ,
                        second_service_as_chief_judge_begin_6 TIMESTAMPTZ,
                        second_service_as_chief_judge_end_6 TIMESTAMPTZ,
                        senior_status_date_6 TIMESTAMPTZ,
                        termination_6 VARCHAR(255),
                        termination_date_6 TIMESTAMPTZ,
                        other_federal_judicial_service_1 TEXT,
                        other_federal_judicial_service_2 TEXT,
                        other_federal_judicial_service_3 TEXT,
                        other_federal_judicial_service_4 TEXT,
                        school_1 VARCHAR(255),
                        degree_1 VARCHAR(255),
                        degree_year_1 TEXT,
                        school_2 VARCHAR(255),
                        degree_2 VARCHAR(255),
                        degree_year_2 TEXT,
                        school_3 VARCHAR(255),
                        degree_3 VARCHAR(255),
                        degree_year_3 TEXT,
                        school_4 VARCHAR(255),
                        degree_4 VARCHAR(255),
                        degree_year_4 TEXT,
                        school_5 VARCHAR(255),
                        degree_5 VARCHAR(255),
                        degree_year_5 TEXT,
                        professional_career TEXT,
                        other_nominations_recess_appointments TEXT
);

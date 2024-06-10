```erDiagram
    case {
        INT case_id PK
        VARCHAR case_number
        DATE case_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    attorney_advocate {
        INT advocate_id PK
        INT case_id FK
        INT attorney_id FK
        VARCHAR role
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    attorney {
        INT attorney_id PK
        VARCHAR first_name
        VARCHAR last_name
        VARCHAR middle_name
        INT firm_id FK
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    firm {
        INT firm_id PK
        VARCHAR name
        TEXT address
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    adr {
        INT adr_id PK
        INT case_id FK
        INT adr_type_id FK
        DATE adr_date
        VARCHAR result
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    adr_type {
        INT adr_type_id PK
        VARCHAR name
        TEXT description
    }

    hearings_events {
        INT hearing_event_id PK
        INT case_id FK
        INT hearing_type_id FK
        DATE hearing_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    hearing_type {
        INT hearing_type_id PK
        VARCHAR name
        TEXT description
    }

    motions_filings {
        INT motion_filing_id PK
        INT case_id FK
        INT filing_type_id FK
        DATE motion_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    orders {
        INT order_id PK
        INT case_id FK
        DATE order_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    participants {
        INT participant_id PK
        INT case_id FK
        INT participant_type_id FK
        INT party_id FK
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    party {
        INT party_id PK
        VARCHAR name
        TEXT address
        VARCHAR phone
        VARCHAR email
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    post_trial {
        INT post_trial_id PK
        INT case_id FK
        INT post_trial_type_id FK
        DATE post_trial_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    pretrial_intake {
        INT pretrial_intake_id PK
        INT case_id FK
        INT pretrial_intake_type_id FK
        DATE pretrial_intake_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    probate {
        INT probate_id PK
        INT case_id FK
        INT probate_type_id FK
        DATE probate_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    sanctions {
        INT sanction_id PK
        INT case_id FK
        INT sanction_type_id FK
        DATE sanction_date
        DECIMAL amount
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    status {
        INT status_id PK
        INT case_id FK
        INT status_type_id FK
        DATE status_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    gateway {
        INT gateway_id PK
        INT case_id FK
        INT gateway_type_id FK
        DATE gateway_date
        TEXT description
        TIMESTAMP created_at
        TIMESTAMP updated_at
    }

    gateway_type {
        INT gateway_type_id PK
        VARCHAR name
        TEXT description
    }

    pleading_type {
        INT pleading_type_id PK
        VARCHAR name
        TEXT description
    }

    post_trial_type {
        INT post_trial_type_id PK
        VARCHAR name
        TEXT description
    }

    pretrial_intake_type {
        INT pretrial_intake_type_id PK
        VARCHAR name
        TEXT description
    }

    probate_type {
        INT probate_type_id PK
        VARCHAR name
        TEXT description
    }

    status_type {
        INT status_type_id PK
        VARCHAR name
        TEXT description
    }

    case ||--o{ attorney_advocate : ""
    case ||--o{ adr : ""
    case ||--o{ hearings_events : ""
    case ||--o{ motions_filings : ""
    case ||--o{ orders : ""
    case ||--o{ participants : ""
    case ||--o{ post_trial : ""
    case ||--o{ pretrial_intake : ""
    case ||--o{ probate : ""
    case ||--o{ sanctions : ""
    case ||--o{ status : ""
    case ||--o{ gateway : ""

    attorney_advocate }o--|| attorney : ""
    attorney_advocate }o--|| case : ""

    attorney }o--|| firm : ""

    adr }o--|| adr_type : ""
    adr }o--|| case : ""

    hearings_events }o--|| hearing_type : ""
    hearings_events }o--|| case : ""

    motions_filings }o--|| case : ""

    orders }o--|| case : ""

    participants }o--|| case : ""
    participants }o--|| participant_type : ""
    participants }o--|| party : ""

    post_trial }o--|| post_trial_type : ""
    post_trial }o--|| case : ""

    pretrial_intake }o--|| pretrial_intake_type : ""
    pretrial_intake }o--|| case : ""

    probate }o--|| probate_type : ""
    probate }o--|| case : ""

    sanctions }o--|| sanction_type : ""
    sanctions }o--|| case : ""

    status }o--|| status_type : ""
    status }o--|| case : ""

    gateway }o--|| gateway_type : ""
    gateway }o--|| case : ""
```
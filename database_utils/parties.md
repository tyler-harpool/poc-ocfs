```mermaid
erDiagram
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

    participants }o--|| party : ""

```
```erDiagram
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

    attorney_advocate }o--|| attorney : ""
    attorney }o--|| firm : ""
```
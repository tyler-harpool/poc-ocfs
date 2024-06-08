-- Add up migration script here
CREATE TABLE IF NOT EXISTS cases (
    id SERIAL PRIMARY KEY,
    case_number VARCHAR(255) NOT NULL,
    client_name VARCHAR(255) NOT NULL,
    case_type VARCHAR(255) NOT NULL,
    case_status VARCHAR(50) NOT NULL,
    date_opened DATE,
    date_closed DATE,
    assigned_attorney VARCHAR(255) NOT NULL,
    notes TEXT
);
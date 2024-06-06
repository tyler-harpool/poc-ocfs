-- Add up migration script here
-- Create attorneyAdvocate Table
CREATE TABLE IF NOT EXISTS attorneyAdvocate (
    id SERIAL PRIMARY KEY,
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    data_element TEXT,
    definition TEXT,
    values JSONB,
    currently_collected TEXT,
    if_no_is_this_needed TEXT,
    if_yes_where TEXT,
    comments TEXT
);

-- Insert Data into attorneyAdvocate
INSERT INTO attorneyAdvocate (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Identifier', 'Unique identifier for the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Role', 'Role of the attorney in the case', '["defense", "prosecution", "advocate", "guardian"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Name', 'Name of the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Address', 'Address of the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Contact', 'Contact information of the attorney', NULL, NULL, NULL, NULL, NULL);

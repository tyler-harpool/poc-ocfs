-- Add up migration script here
-- Create pretrialIntake Table
CREATE TABLE IF NOT EXISTS pretrialIntake (
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

-- Insert Data into pretrialIntake
INSERT INTO pretrialIntake (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Type', 'The type of intake conducted', '["booking", "screening", "assessment", "interview"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Status', 'The status of the intake process', '["completed", "pending", "incomplete", "referred"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Date', 'The date the intake was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Party', 'The party associated with the intake', '["defendant", "suspect", "witness", "victim"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Result', 'The result of the intake process', '["processed", "released", "detained", "transferred"]'::jsonb, NULL, NULL, NULL, NULL);

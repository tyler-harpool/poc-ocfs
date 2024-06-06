-- Add up migration script here
-- Create charges Table
CREATE TABLE IF NOT EXISTS charges (
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

-- Insert Data into charges
INSERT INTO charges (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Type', 'The type of charge filed in the case', '["felony", "misdemeanor", "infraction", "citation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Status', 'The status of the charge', '["filed", "pending", "dismissed", "convicted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Date', 'The date the charge was filed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Party', 'The party associated with the charge', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Result', 'The result of the charge', '["guilty", "not guilty", "nolo contendere", "acquitted"]'::jsonb, NULL, NULL, NULL, NULL);

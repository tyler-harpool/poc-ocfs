-- Add up migration script here

-- Create dependencyPermanency Table
CREATE TABLE IF NOT EXISTS dependencyPermanency (
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

-- Insert Data into dependencyPermanency
INSERT INTO dependencyPermanency (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Plan', 'The type of permanency plan in place', '["reunification", "adoption", "guardianship", "long-term care"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Status', 'The status of the permanency plan', '["in progress", "completed", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Date', 'The date the permanency plan was established', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Party', 'The party responsible for the permanency plan', '["caseworker", "guardian", "parent", "court"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Result', 'The result of the permanency plan', '["achieved", "not achieved", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL);

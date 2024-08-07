-- Add migration script here
-- Create CaseData Table
CREATE TABLE IF NOT EXISTS CaseData (
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

-- Insert Data into CaseData
INSERT INTO CaseData (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES ('Civil data', 'Family data', 'Probate data', 'Dependency data', 'Juvenile data', 'Criminal data', 'Traffic data', 'Element data', 'Definition data', '{}'::jsonb, 'Yes', 'No', 'Some place', 'Some comments');

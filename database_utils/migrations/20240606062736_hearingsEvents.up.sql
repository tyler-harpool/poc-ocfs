-- Create hearingsEvents Table
CREATE TABLE IF NOT EXISTS hearingsEvents (
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

-- Insert Data into hearingsEvents
INSERT INTO hearingsEvents (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Hearing Type', 'The type of hearing scheduled or held in the case', '["initial hearing", "preliminary hearing", "motion hearing", "trial"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Event Type', 'The type of event scheduled or held in the case', '["conference", "mediation", "settlement", "review"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Hearing Date', 'The date the hearing was scheduled or held', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Event Date', 'The date the event was scheduled or held', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Hearing Result', 'The result of the hearing', '["held", "continued", "postponed", "cancelled"]'::jsonb, NULL, NULL, NULL, NULL);

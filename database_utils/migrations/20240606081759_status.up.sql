-- Add up migration script here
CREATE TABLE IF NOT EXISTS status (
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

INSERT INTO status (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
 VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Case Status', 'The current status of the case', '["open", "closed", "pending", "resolved"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Hearing Status', 'The status of the hearing related to the case', '["scheduled", "postponed", "cancelled", "held"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Filing Status', 'The status of the filings in the case', '["filed", "not filed", "rejected", "accepted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Disposition Status', 'The final disposition status of the case', '["convicted", "acquitted", "dismissed", "plea"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Status', 'The status of the judgment in the case', '["entered", "pending", "appealed", "satisfied"]'::jsonb, NULL, NULL, NULL, NULL);

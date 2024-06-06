-- Add up migration script here
-- Create sanctions Table
CREATE TABLE IF NOT EXISTS sanctions (
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

-- Insert Data into sanctions
INSERT INTO sanctions (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Sanction Type', 'The type of sanction imposed', '["fine", "community service", "probation", "jail"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Sanction Status', 'The status of the sanction', '["imposed", "pending", "completed", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Sanction Date', 'The date the sanction was imposed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Sanction Party', 'The party who received the sanction', '["defendant", "plaintiff", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Sanction Result', 'The result of the sanction', '["satisfied", "unsatisfied", "modified", "vacated"]'::jsonb, NULL, NULL, NULL, NULL);

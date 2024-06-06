-- Add up migration script here
-- Create pleadings Table
CREATE TABLE IF NOT EXISTS pleadings (
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

-- Insert Data into pleadings
INSERT INTO pleadings (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Pleading Type', 'The type of pleading filed in the case', '["complaint", "answer", "motion", "brief"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Pleading Status', 'The status of the pleading', '["filed", "pending", "granted", "denied"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Pleading Date', 'The date the pleading was filed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Pleading Party', 'The party who filed the pleading', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Pleading Result', 'The result of the pleading', '["approved", "rejected", "modified", "withdrawn"]'::jsonb, NULL, NULL, NULL, NULL);

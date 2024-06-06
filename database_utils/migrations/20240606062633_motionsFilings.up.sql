-- Add up migration script here
-- Create motionsFilings Table
CREATE TABLE IF NOT EXISTS motionsFilings (
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

-- Insert Data into motionsFilings
INSERT INTO motionsFilings (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Motion Type', 'The type of motion filed in the case', '["motion to dismiss", "motion for summary judgment", "motion to compel", "motion in limine"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Filing Status', 'The status of the filing', '["filed", "pending", "granted", "denied"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Filing Date', 'The date the filing was submitted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Filing Party', 'The party who submitted the filing', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Filing Result', 'The result of the filing', '["approved", "rejected", "modified", "withdrawn"]'::jsonb, NULL, NULL, NULL, NULL);

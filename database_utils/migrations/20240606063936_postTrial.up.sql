-- Add up migration script here
-- Create postTrial Table
CREATE TABLE IF NOT EXISTS postTrial (
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

-- Insert Data into postTrial
INSERT INTO postTrial (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Post-trial Activity', 'The type of activity conducted post-trial', '["appeal", "motion for new trial", "sentencing", "probation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Activity Status', 'The status of the post-trial activity', '["pending", "completed", "denied", "granted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Activity Date', 'The date the post-trial activity was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Activity Party', 'The party responsible for the post-trial activity', '["defendant", "plaintiff", "court", "attorney"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Activity Result', 'The result of the post-trial activity', '["granted", "denied", "modified", "vacated"]'::jsonb, NULL, NULL, NULL, NULL);

-- Add up migration script here
-- Create diversion Table
CREATE TABLE IF NOT EXISTS diversion (
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

-- Insert Data into diversion
INSERT INTO diversion (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Diversion Type', 'The type of diversion program', '["pre-charge", "post-charge", "pre-adjudication", "post-adjudication"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Diversion Status', 'The status of the diversion', '["active", "completed", "terminated", "referred"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Diversion Date', 'The date the diversion was initiated', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Diversion Party', 'The party participating in the diversion', '["defendant", "juvenile", "participant"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Diversion Result', 'The outcome of the diversion program', '["successful", "unsuccessful", "ongoing", "transferred"]'::jsonb, NULL, NULL, NULL, NULL);

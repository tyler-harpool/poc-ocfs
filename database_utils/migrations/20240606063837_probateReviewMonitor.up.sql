-- Add up migration script here


-- Create probateReviewMonitor Table
CREATE TABLE IF NOT EXISTS probateReviewMonitor (
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

-- Insert Data into probateReviewMonitor
INSERT INTO probateReviewMonitor (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Review Type', 'The type of probate review or monitoring', '["annual review", "status check", "compliance review", "final review"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Review Status', 'The status of the probate review or monitoring', '["scheduled", "completed", "pending", "cancelled"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Review Date', 'The date the probate review or monitoring was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Review Party', 'The party responsible for the probate review or monitoring', '["court", "guardian", "trustee", "beneficiary"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Review Result', 'The result of the probate review or monitoring', '["approved", "denied", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL);

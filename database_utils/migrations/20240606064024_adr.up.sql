-- Add up migration script here
-- Create adr Table
CREATE TABLE IF NOT EXISTS adr (
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

-- Insert Data into adr
INSERT INTO adr (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'ADR Type', 'The type of alternative dispute resolution used', '["mediation", "arbitration", "settlement conference", "neutral evaluation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'ADR Status', 'The status of the ADR process', '["initiated", "in progress", "completed", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'ADR Date', 'The date the ADR process was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'ADR Party', 'The party involved in the ADR process', '["plaintiff", "defendant", "third party", "mediator"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'ADR Result', 'The result of the ADR process', '["settled", "not settled", "partially settled", "pending"]'::jsonb, NULL, NULL, NULL, NULL);

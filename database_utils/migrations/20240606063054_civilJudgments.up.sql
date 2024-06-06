-- Add up migration script here
-- Create civilJudgments Table
CREATE TABLE IF NOT EXISTS civilJudgments (
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

-- Insert Data into civilJudgments
INSERT INTO civilJudgments (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Type', 'The type of civil judgment issued', '["monetary", "non-monetary", "consent", "default"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Status', 'The status of the civil judgment', '["issued", "pending", "satisfied", "vacated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Date', 'The date the civil judgment was issued', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Party', 'The party against whom the judgment was issued', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Result', 'The result of the civil judgment', '["paid", "unpaid", "partially paid", "appealed"]'::jsonb, NULL, NULL, NULL, NULL);

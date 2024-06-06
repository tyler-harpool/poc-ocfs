-- Create CaseData Table
CREATE TABLE IF NOT EXISTS CaseData (
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

-- Insert Data into CaseData
INSERT INTO CaseData (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES ('Civil data', 'Family data', 'Probate data', 'Dependency data', 'Juvenile data', 'Criminal data', 'Traffic data', 'Element data', 'Definition data', '{}'::jsonb, 'Yes', 'No', 'Some place', 'Some comments');

-- Create participants Table
CREATE TABLE IF NOT EXISTS participants (
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

-- Insert Data into participants
INSERT INTO participants (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Participant Identifier', 'Unique identifier for the participant', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Participant Role', 'Role of the participant in the case', '["plaintiff", "defendant", "witness", "advocate"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Participant Name', 'Name of the participant', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Participant Address', 'Address of the participant', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Participant Contact', 'Contact information of the participant', NULL, NULL, NULL, NULL, NULL);

-- Create attorneyAdvocate Table
CREATE TABLE IF NOT EXISTS attorneyAdvocate (
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

-- Insert Data into attorneyAdvocate
INSERT INTO attorneyAdvocate (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Identifier', 'Unique identifier for the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Role', 'Role of the attorney in the case', '["defense", "prosecution", "advocate", "guardian"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Name', 'Name of the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Address', 'Address of the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Attorney Contact', 'Contact information of the attorney', NULL, NULL, NULL, NULL, NULL);

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

-- Create status Table
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

-- Insert Data into status
INSERT INTO status (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Case Status', 'The current status of the case', '["open", "closed", "pending", "resolved"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Hearing Status', 'The status of the hearing related to the case', '["scheduled", "postponed", "cancelled", "held"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Filing Status', 'The status of the filings in the case', '["filed", "not filed", "rejected", "accepted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Disposition Status', 'The final disposition status of the case', '["convicted", "acquitted", "dismissed", "plea"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Judgment Status', 'The status of the judgment in the case', '["entered", "pending", "appealed", "satisfied"]'::jsonb, NULL, NULL, NULL, NULL);

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

-- Create orders Table
CREATE TABLE IF NOT EXISTS orders (
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

-- Insert Data into orders
INSERT INTO orders (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Order Type', 'The type of order issued in the case', '["protective order", "restraining order", "judgment", "injunction"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Order Status', 'The status of the order', '["issued", "pending", "vacated", "modified"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Order Date', 'The date the order was issued', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Order Party', 'The party associated with the order', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Order Result', 'The result of the order', '["enforced", "dismissed", "appealed", "satisfied"]'::jsonb, NULL, NULL, NULL, NULL);

-- Create charges Table
CREATE TABLE IF NOT EXISTS charges (
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

-- Insert Data into charges
INSERT INTO charges (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Type', 'The type of charge filed in the case', '["felony", "misdemeanor", "infraction", "citation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Status', 'The status of the charge', '["filed", "pending", "dismissed", "convicted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Date', 'The date the charge was filed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Party', 'The party associated with the charge', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Charge Result', 'The result of the charge', '["guilty", "not guilty", "nolo contendere", "acquitted"]'::jsonb, NULL, NULL, NULL, NULL);

-- Create pretrialIntake Table
CREATE TABLE IF NOT EXISTS pretrialIntake (
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

-- Insert Data into pretrialIntake
INSERT INTO pretrialIntake (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Type', 'The type of intake conducted', '["booking", "screening", "assessment", "interview"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Status', 'The status of the intake process', '["completed", "pending", "incomplete", "referred"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Date', 'The date the intake was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Party', 'The party associated with the intake', '["defendant", "suspect", "witness", "victim"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Intake Result', 'The result of the intake process', '["processed", "released", "detained", "transferred"]'::jsonb, NULL, NULL, NULL, NULL);

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

-- Create dependencyPermanency Table
CREATE TABLE IF NOT EXISTS dependencyPermanency (
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

-- Insert Data into dependencyPermanency
INSERT INTO dependencyPermanency (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Plan', 'The type of permanency plan in place', '["reunification", "adoption", "guardianship", "long-term care"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Status', 'The status of the permanency plan', '["in progress", "completed", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Date', 'The date the permanency plan was established', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Party', 'The party responsible for the permanency plan', '["caseworker", "guardian", "parent", "court"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 'Permanency Result', 'The result of the permanency plan', '["achieved", "not achieved", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL);

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

-- Create Table
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


-- Insert Data with JSONB Values
INSERT INTO CaseData (civ, fam, prob, dep, juv, crim, traf, data_element, definition, values, currently_collected, if_no_is_this_needed, if_yes_where, comments)
VALUES ('Civil data', 'Family data', 'Probate data', 'Dependency data', 'Juvenile data', 'Criminal data', 'Traffic data', 'Element data', 'Definition data', '{}', 'Yes', 'No', 'Some place', 'Some comments');

CREATE TABLE participants (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);
INSERT INTO participants (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Participant Identifier', 'Unique identifier for the participant', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Participant Role', 'Role of the participant in the case', '["plaintiff", "defendant", "witness", "advocate"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Participant Name', 'Name of the participant', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Participant Address', 'Address of the participant', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Participant Contact', 'Contact information of the participant', NULL, NULL, NULL, NULL, NULL);
CREATE TABLE attorneyAdvocate (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);

INSERT INTO attorneyAdvocate (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Attorney Identifier', 'Unique identifier for the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Attorney Role', 'Role of the attorney in the case', '["defense", "prosecution", "advocate", "guardian"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Attorney Name', 'Name of the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Attorney Address', 'Address of the attorney', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Attorney Contact', 'Contact information of the attorney', NULL, NULL, NULL, NULL, NULL);
CREATE TABLE pleadings (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO pleadings (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Pleading Type', 'The type of pleading filed in the case', '["complaint", "answer", "motion", "brief"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Pleading Status', 'The status of the pleading', '["filed", "pending", "granted", "denied"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Pleading Date', 'The date the pleading was filed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Pleading Party', 'The party who filed the pleading', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Pleading Result', 'The result of the pleading', '["approved", "rejected", "modified", "withdrawn"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE status (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO status (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Case Status', 'The current status of the case', '["open", "closed", "pending", "resolved"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Hearing Status', 'The status of the hearing related to the case', '["scheduled", "postponed", "cancelled", "held"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Filing Status', 'The status of the filings in the case', '["filed", "not filed", "rejected", "accepted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Disposition Status', 'The final disposition status of the case', '["convicted", "acquitted", "dismissed", "plea"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Judgment Status', 'The status of the judgment in the case', '["entered", "pending", "appealed", "satisfied"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE motionsFilings (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);



INSERT INTO motionsFilings (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Motion Type', 'The type of motion filed in the case', '["motion to dismiss", "motion for summary judgment", "motion to compel", "motion in limine"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Filing Status', 'The status of the filing', '["filed", "pending", "granted", "denied"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Filing Date', 'The date the filing was submitted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Filing Party', 'The party who submitted the filing', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Filing Result', 'The result of the filing', '["approved", "rejected", "modified", "withdrawn"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE hearingsEvents (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO hearingsEvents (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Hearing Type', 'The type of hearing scheduled or held in the case', '["initial hearing", "preliminary hearing", "motion hearing", "trial"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Event Type', 'The type of event scheduled or held in the case', '["conference", "mediation", "settlement", "review"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Hearing Date', 'The date the hearing was scheduled or held', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Event Date', 'The date the event was scheduled or held', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Hearing Result', 'The result of the hearing', '["held", "continued", "postponed", "cancelled"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE orders (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO orders (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Order Type', 'The type of order issued in the case', '["protective order", "restraining order", "judgment", "injunction"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Order Status', 'The status of the order', '["issued", "pending", "vacated", "modified"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Order Date', 'The date the order was issued', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Order Party', 'The party associated with the order', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Order Result', 'The result of the order', '["enforced", "dismissed", "appealed", "satisfied"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE charges (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);

INSERT INTO charges (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Charge Type', 'The type of charge filed in the case', '["felony", "misdemeanor", "infraction", "citation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Charge Status', 'The status of the charge', '["filed", "pending", "dismissed", "convicted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Charge Date', 'The date the charge was filed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Charge Party', 'The party associated with the charge', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Charge Result', 'The result of the charge', '["guilty", "not guilty", "nolo contendere", "acquitted"]'::jsonb, NULL, NULL, NULL, NULL);

CREATE TABLE pretrialIntake (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);

INSERT INTO pretrialIntake (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Intake Type', 'The type of intake conducted', '["booking", "screening", "assessment", "interview"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Intake Status', 'The status of the intake process', '["completed", "pending", "incomplete", "referred"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Intake Date', 'The date the intake was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Intake Party', 'The party associated with the intake', '["defendant", "suspect", "witness", "victim"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Intake Result', 'The result of the intake process', '["processed", "released", "detained", "transferred"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE civilJudgments (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);
INSERT INTO civilJudgments (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Judgment Type', 'The type of civil judgment issued', '["monetary", "non-monetary", "consent", "default"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Judgment Status', 'The status of the civil judgment', '["issued", "pending", "satisfied", "vacated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Judgment Date', 'The date the civil judgment was issued', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Judgment Party', 'The party against whom the judgment was issued', '["plaintiff", "defendant", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Judgment Result', 'The result of the civil judgment', '["paid", "unpaid", "partially paid", "appealed"]'::jsonb, NULL, NULL, NULL, NULL);


CREATE TABLE diversion (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);
INSERT INTO diversion (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Diversion Type', 'The type of diversion program', '["pre-charge", "post-charge", "pre-adjudication", "post-adjudication"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Diversion Status', 'The status of the diversion', '["active", "completed", "terminated", "referred"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Diversion Date', 'The date the diversion was initiated', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Diversion Party', 'The party participating in the diversion', '["defendant", "juvenile", "participant"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Diversion Result', 'The outcome of the diversion program', '["successful", "unsuccessful", "ongoing", "transferred"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE sanctions (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);

INSERT INTO sanctions (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Sanction Type', 'The type of sanction imposed', '["fine", "community service", "probation", "jail"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Sanction Status', 'The status of the sanction', '["imposed", "pending", "completed", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Sanction Date', 'The date the sanction was imposed', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Sanction Party', 'The party who received the sanction', '["defendant", "plaintiff", "third party"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Sanction Result', 'The result of the sanction', '["satisfied", "unsatisfied", "modified", "vacated"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE dependencyPermanency (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO dependencyPermanency (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Permanency Plan', 'The type of permanency plan in place', '["reunification", "adoption", "guardianship", "long-term care"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Permanency Status', 'The status of the permanency plan', '["in progress", "completed", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Permanency Date', 'The date the permanency plan was established', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Permanency Party', 'The party responsible for the permanency plan', '["caseworker", "guardian", "parent", "court"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Permanency Result', 'The result of the permanency plan', '["achieved", "not achieved", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE probateReviewMonitor (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO probateReviewMonitor (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Review Type', 'The type of probate review or monitoring', '["annual review", "status check", "compliance review", "final review"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Review Status', 'The status of the probate review or monitoring', '["scheduled", "completed", "pending", "cancelled"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Review Date', 'The date the probate review or monitoring was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Review Party', 'The party responsible for the probate review or monitoring', '["court", "guardian", "trustee", "beneficiary"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Review Result', 'The result of the probate review or monitoring', '["approved", "denied", "modified", "terminated"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE postTrial (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);


INSERT INTO postTrial (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'Post-trial Activity', 'The type of activity conducted post-trial', '["appeal", "motion for new trial", "sentencing", "probation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'Activity Status', 'The status of the post-trial activity', '["pending", "completed", "denied", "granted"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'Activity Date', 'The date the post-trial activity was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'Activity Party', 'The party responsible for the post-trial activity', '["defendant", "plaintiff", "court", "attorney"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'Activity Result', 'The result of the post-trial activity', '["granted", "denied", "modified", "vacated"]'::jsonb, NULL, NULL, NULL, NULL);
CREATE TABLE adr (
    civ TEXT,
    fam TEXT,
    prob TEXT,
    dep TEXT,
    juv TEXT,
    crim TEXT,
    traf TEXT,
    id FLOAT,
    dataElement TEXT,
    definition TEXT,
    values JSONB,
    currentlyCollected TEXT,
    ifNoIsThisNeeded TEXT,
    ifYesWhere TEXT,
    comments TEXT
);
INSERT INTO adr (
    civ, fam, prob, dep, juv, crim, traf, id, dataElement, definition, values, currentlyCollected, ifNoIsThisNeeded, ifYesWhere, comments
) VALUES
('x', 'x', 'x', 'x', 'x', 'x', 'x', 1, 'ADR Type', 'The type of alternative dispute resolution used', '["mediation", "arbitration", "settlement conference", "neutral evaluation"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 2, 'ADR Status', 'The status of the ADR process', '["initiated", "in progress", "completed", "terminated"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 3, 'ADR Date', 'The date the ADR process was conducted', NULL, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 4, 'ADR Party', 'The party involved in the ADR process', '["plaintiff", "defendant", "third party", "mediator"]'::jsonb, NULL, NULL, NULL, NULL),
('x', 'x', 'x', 'x', 'x', 'x', 'x', 5, 'ADR Result', 'The result of the ADR process', '["settled", "not settled", "partially settled", "pending"]'::jsonb, NULL, NULL, NULL, NULL);

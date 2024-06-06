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

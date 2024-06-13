CREATE TABLE ticket_custom_fields (
    id SERIAL PRIMARY KEY,
    field_name TEXT NOT NULL,
    field_type TEXT NOT NULL, 
    field_size INTEGER NOT NULL,
    select_values TEXT[],
    order_index INTEGER NOT NULL
);

CREATE TABLE ticket_custom_field_data (
    id SERIAL PRIMARY KEY,
    ticket_id INTEGER REFERENCES tickets(ticket_id) ON DELETE CASCADE,
    custom_field_id INTEGER REFERENCES ticket_custom_fields(id) ON DELETE CASCADE,
    field_value TEXT
);
CREATE TABLE ticket_custom_fields (
    id SERIAL PRIMARY KEY,
    field_name TEXT NOT NULL,
    field_type TEXT NOT NULL, 
    field_size INTEGER NOT NULL,
    is_select BOOLEAN NOT NULL,
    select_values TEXT[]
);

CREATE TABLE ticket_custom_field_data (
    id SERIAL PRIMARY KEY,
    ticket_id INTEGER REFERENCES tickets(ticket_id) ON DELETE CASCADE,
    custom_field_id INTEGER REFERENCES ticket_custom_fields(id) ON DELETE CASCADE,
    field_value TEXT
);
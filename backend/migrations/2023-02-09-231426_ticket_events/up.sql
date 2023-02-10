CREATE TABLE IF NOT EXISTS ticket_events (
    event_id UUID PRIMARY KEY,
    ticket_id INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    event_data TEXT NOT NULL,
    user_id UUID,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_ticket_event_ticket
        FOREIGN KEY (ticket_id)
        REFERENCES tickets (ticket_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_ticket_event_user
        FOREIGN KEY (user_id)
        REFERENCES users (user_id)
        ON DELETE SET NULL
);
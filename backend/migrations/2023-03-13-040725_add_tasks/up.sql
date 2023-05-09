CREATE TABLE tasklists (
    tasklist_id UUID PRIMARY KEY,
    ticket_id INTEGER NOT NULL,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_tasklist_ticket
        FOREIGN KEY (ticket_id)
        REFERENCES tickets (ticket_id)
        ON DELETE CASCADE
);

CREATE TABLE tasks (
    task_id UUID PRIMARY KEY,
    tasklist_id UUID NOT NULL,
    label TEXT NOT NULL,
    is_done BOOLEAN NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_tasklist
        FOREIGN KEY (tasklist_id)
        REFERENCES tasklists (tasklist_id)
        ON DELETE CASCADE
);

CREATE TABLE tasklist_templates (
    tasklist_id UUID PRIMARY KEY,
    template_id UUID NOT NULL,
    label TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE tasklist_templates_tasks (
    task_id UUID PRIMARY KEY,
    tasklist_id UUID NOT NULL,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_tasklist_templates
        FOREIGN KEY (tasklist_id)
        REFERENCES tasklist_templates (tasklist_id)
        ON DELETE CASCADE
);
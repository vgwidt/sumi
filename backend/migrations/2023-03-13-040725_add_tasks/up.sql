CREATE TABLE task_groups (
    group_id UUID PRIMARY KEY,
    ticket_id INTEGER NOT NULL,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_task_group_ticket
        FOREIGN KEY (ticket_id)
        REFERENCES tickets (ticket_id)
        ON DELETE CASCADE
);

CREATE TABLE tasks (
    task_id UUID PRIMARY KEY,
    group_id UUID NOT NULL,
    label TEXT NOT NULL,
    is_done BOOLEAN NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_task_group
        FOREIGN KEY (group_id)
        REFERENCES task_groups (group_id)
        ON DELETE CASCADE
);

CREATE TABLE task_templates (
    template_id UUID PRIMARY KEY,
    label TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE task_template_groups (
    group_id UUID PRIMARY KEY,
    template_id UUID NOT NULL,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_task_template_group_template
        FOREIGN KEY (template_id)
        REFERENCES task_templates (template_id)
        ON DELETE CASCADE
);

CREATE TABLE task_template_tasks (
    task_id UUID PRIMARY KEY,
    group_id UUID NOT NULL,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL,
    CONSTRAINT fk_task_template_task_group
        FOREIGN KEY (group_id)
        REFERENCES task_template_groups (group_id)
        ON DELETE CASCADE
);
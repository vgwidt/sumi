// @generated automatically by Diesel CLI.

diesel::table! {
    comments (comment_id) {
        comment_id -> Uuid,
        document_id -> Uuid,
        author -> Nullable<Uuid>,
        text -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    contacts (contact_id) {
        contact_id -> Uuid,
        display_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    document_revisions (revision_id) {
        revision_id -> Uuid,
        document_id -> Uuid,
        content -> Text,
        updated_by -> Nullable<Uuid>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    documents (document_id) {
        document_id -> Uuid,
        parent_id -> Nullable<Uuid>,
        url -> Text,
        title -> Text,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Nullable<Uuid>,
        updated_by -> Nullable<Uuid>,
        archived -> Bool,
    }
}

diesel::table! {
    notes (note_id) {
        note_id -> Uuid,
        ticket -> Int4,
        owner -> Nullable<Uuid>,
        text -> Text,
        time -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ticket_custom_field_data (id) {
        id -> Int4,
        ticket_id -> Nullable<Int4>,
        custom_field_id -> Nullable<Int4>,
        field_value -> Nullable<Text>,
    }
}

diesel::table! {
    ticket_custom_fields (id) {
        id -> Int4,
        field_name -> Text,
        field_type -> Text,
        field_size -> Int4,
        is_select -> Bool,
        select_values -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    ticket_events (event_id) {
        event_id -> Uuid,
        ticket_id -> Int4,
        event_type -> Text,
        event_data -> Text,
        user_id -> Nullable<Uuid>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    ticket_revisions (revision_id) {
        revision_id -> Uuid,
        ticket_id -> Int4,
        description -> Text,
        updated_by -> Nullable<Uuid>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tickets (ticket_id) {
        ticket_id -> Int4,
        assignee -> Nullable<Uuid>,
        contact -> Nullable<Uuid>,
        title -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        due_date -> Nullable<Timestamp>,
        priority -> Text,
        status -> Text,
        resolution -> Nullable<Uuid>,
        created_by -> Nullable<Uuid>,
        updated_by -> Nullable<Uuid>,
        revision -> Timestamp,
        revision_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    user_preferences (user_id) {
        user_id -> Uuid,
        theme -> Nullable<Text>,
        locale -> Nullable<Text>,
        timezone -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Text,
        display_name -> Text,
        email -> Text,
        created_at -> Timestamp,
        access -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(comments -> documents (document_id));
diesel::joinable!(comments -> users (author));
diesel::joinable!(document_revisions -> documents (document_id));
diesel::joinable!(document_revisions -> users (updated_by));
diesel::joinable!(notes -> users (owner));
diesel::joinable!(ticket_custom_field_data -> ticket_custom_fields (custom_field_id));
diesel::joinable!(ticket_custom_field_data -> tickets (ticket_id));
diesel::joinable!(ticket_events -> tickets (ticket_id));
diesel::joinable!(ticket_events -> users (user_id));
diesel::joinable!(ticket_revisions -> tickets (ticket_id));
diesel::joinable!(ticket_revisions -> users (updated_by));
diesel::joinable!(tickets -> contacts (contact));
diesel::joinable!(tickets -> users (assignee));
diesel::joinable!(user_preferences -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    contacts,
    document_revisions,
    documents,
    notes,
    ticket_custom_field_data,
    ticket_custom_fields,
    ticket_events,
    ticket_revisions,
    tickets,
    user_preferences,
    users,
);

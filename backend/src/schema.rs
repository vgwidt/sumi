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
    }
}

diesel::table! {
    user_preferences (user_id) {
        user_id -> Uuid,
        theme -> Nullable<Text>,
        locale -> Nullable<Text>,
        timezone -> Nullable<Text>,
        custom_views -> Jsonb,
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
diesel::joinable!(notes -> users (owner));
diesel::joinable!(tickets -> contacts (contact));
diesel::joinable!(tickets -> users (assignee));
diesel::joinable!(user_preferences -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    contacts,
    documents,
    notes,
    tickets,
    user_preferences,
    users,
);

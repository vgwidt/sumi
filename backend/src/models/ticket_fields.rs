use crate::schema::{ticket_custom_fields, ticket_custom_field_data};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TicketCustomField {
    pub id: i32,
    pub field_name: String,
    pub field_type: String,
    pub field_size: i32,
    pub is_select: bool,
    pub select_values: Option<Vec<Option<String>>>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TicketCustomFieldData {
    pub id: i32,
    pub ticket_id: i32,
    pub custom_field_id: i32,
    pub field_value: String,
}

#[derive(Deserialize, Debug, Insertable)]
#[diesel(table_name = ticket_custom_fields)]
pub struct NewTicketCustomField {
    pub field_name: String,
    pub field_type: String,
    pub field_size: i32,
    pub is_select: bool,
    pub select_values: Option<Vec<Option<String>>>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = ticket_custom_field_data)]
pub struct NewTicketCustomFieldData {
    pub ticket_id: i32,
    pub custom_field_id: i32,
    pub field_value: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = ticket_custom_fields)]
pub struct TicketCustomFieldPayload {
    pub field_name: String,
    pub field_type: String,
    pub field_size: i32,
    pub is_select: bool,
    pub select_values: Option<Vec<Option<String>>>,
}
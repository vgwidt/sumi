use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserRepresentation;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NoteInfo {
    pub note_id: Uuid,
    pub ticket: i32,
    pub owner: Option<UserRepresentation>,
    pub text: String,
    pub time: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl NoteInfo {
    pub fn display_name(&self) -> String {
        //backend will return 0's for owner if it is not set
        if self.owner.is_some() && self.owner.clone().unwrap().user_id != Uuid::nil() {
            self.owner.clone().unwrap().display_name
        } else {
            "Unknown".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NoteCreateInfo {
    pub ticket: i32,
    pub owner: Option<Uuid>,
    pub text: String,
    pub time: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteListInfo {
    pub notes: Vec<NoteInfo>,
}

impl IntoIterator for NoteListInfo {
    type Item = NoteInfo;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}
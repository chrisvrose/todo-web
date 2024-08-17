use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Todo {
    pub id: i32,
    pub owner: Option<i32>,
    pub title: String,
    pub state: TodoState,
    pub description: Option<String>,
    pub project_grouping: Option<i32>,
    pub date_added: DateTime<Utc>,
    pub date_updated: DateTime<Utc>
}

impl Todo {

}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub enum TodoState {
    #[default]
    ToBeDone,
    InProgress,
    Done,
}

impl From<String> for TodoState{
    fn from(value: String) -> Self {
        match value.as_str(){
            "DONE"=>TodoState::Done,
            "INP"=>TodoState::InProgress,
            _=>{
                TodoState::ToBeDone
            }
        }
    }
}

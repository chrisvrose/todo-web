use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgTypeInfo, Postgres, Type, TypeInfo};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Todo {
    pub id: Option<i32>,
    pub owner: Option<i32>,
    pub title: String,
    pub state: TodoState,
    pub description: Option<String>,
    pub project_grouping: Option<i32>,
    pub date_added: Option<DateTime<Utc>>,
    pub date_updated: Option<DateTime<Utc>>
}

impl Todo {

}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default, PartialEq, Eq, sqlx::Type)]
pub enum TodoState {
    #[default]
    TBD,
    INP,
    DONE,
}

impl From<String> for TodoState{
    fn from(value: String) -> Self {
        match value.as_str(){
            "DONE"=>TodoState::DONE,
            "INP"=>TodoState::INP,
            _=>{
                TodoState::TBD
            }
        }
    }
}
impl Into<String> for TodoState{
    fn into(self) -> String {
        format!("{:?}", self)
    }
}

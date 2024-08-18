use serde::{Deserialize, Serialize};

use super::entity::{Todo, TodoState};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Debug)]
pub struct TodoDto {
    pub owner: Option<i32>,
    pub title: String,
    pub state: TodoState,
    pub description: Option<String>,
    pub project_grouping: Option<i32>,
}

impl Into<Todo> for TodoDto {
    fn into(self) -> Todo {
        let TodoDto {owner, title, state, description, project_grouping} = self;
        Todo {
            id: None,
            owner,
            title,
            state,
            description,
            project_grouping,
            date_added: None,
            date_updated: None,
        }
    }
}

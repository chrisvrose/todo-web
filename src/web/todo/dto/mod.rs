use serde::{Deserialize, Serialize};

use super::entity::{Todo, TodoState};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, Debug)]
pub struct TodoDto {
    pub title: String,
    pub state: TodoState,
    pub description: Option<String>,
    pub project_grouping: Option<i32>,
}

impl Into<Todo> for TodoDto {
    /// create a todo from dto. Note that remaining columns are set to none/default as applicable
    fn into(self) -> Todo {
        let TodoDto {title, state, description, project_grouping} = self;
        Todo {
            id: None,
            owner: None,
            title,
            state,
            description,
            project_grouping,
            date_added: None,
            date_updated: None,
        }
    }
}

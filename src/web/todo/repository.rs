use std::sync::Arc;

use sqlx::PgPool;

use crate::util::db::DbResult;

use super::entity::Todo;

pub trait TodoRepository {
    async fn get_list(&self) -> DbResult<Vec<Todo>>;
    async fn create(&self, todo: Todo) -> DbResult<Todo>;
}

pub struct PostgresTodoRepository {
    todo_list: Arc<PgPool>,
}

impl PostgresTodoRepository {
    pub fn new(pg_pool: Arc<PgPool>) -> PostgresTodoRepository {
        PostgresTodoRepository {
            todo_list: Arc::from(pg_pool),
        }
    }
}

impl TodoRepository for PostgresTodoRepository {
    async fn get_list(&self) -> DbResult<Vec<Todo>> {
        let row_vector = sqlx::query_as!(Todo, "select * from todo");
        let y = row_vector.fetch_all(&*self.todo_list).await?;
        Ok(y)
    }

    async fn create(&self, todo: Todo) -> DbResult<Todo> {
        let todo_state_str: String = todo.state.into();
        let inserted_todo = sqlx::query_as!(
            Todo,
            "INSERT INTO todo (owner,title,state,description,project_grouping) VALUES ( $1,$2,$3,$4, $5) returning *",
            todo.owner, todo.title, todo_state_str ,todo.description, todo.project_grouping
        ).fetch_one(&*self.todo_list).await?;
        Ok(inserted_todo)
    }
}

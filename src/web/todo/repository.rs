use std::sync::Arc;

use sqlx::PgPool;

use crate::util::db::DbResult;

use super::entity::Todo;

pub trait TodoRepository {
    async fn get_list(&self) -> DbResult<Vec<Todo>>;
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
}

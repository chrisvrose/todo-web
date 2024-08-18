use actix_web::{
    web::{Data, Form, Json},
    Responder,
};
use log::{debug, info, warn};

use crate::web::todo::{
    entity::Todo,
    repository::{PostgresTodoRepository, TodoRepository},
};

use super::dto::TodoDto;

#[actix_web::get("/todo")]
pub async fn get_list(repository: Data<PostgresTodoRepository>) -> impl Responder {
    debug!("Getting list of todos");
    let list = repository.get_list().await.expect("Failed to get msgs");
    Json(list)
}

#[actix_web::post("/todo")]
pub async fn create(
    repository: Data<PostgresTodoRepository>,
    input: actix_web::web::Form<TodoDto>,
) -> impl Responder {
    let todo_entity:Todo = input.into_inner().into();
    debug!("content converted to - {:?}",&todo_entity);
    let inserted_todo = repository.create(todo_entity).await.inspect_err(|err|warn!("Failed insert {}",err)).unwrap();
    Json(inserted_todo)
}

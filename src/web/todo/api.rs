use actix_web::{web::{Data, Json}, Responder};
use log::{debug, info, warn};

use crate::web::todo::{entity::Todo, repository::{PostgresTodoRepository, TodoRepository}};

#[actix_web::get("get")]
pub async fn get_list(repository: Data<PostgresTodoRepository>) -> impl Responder {
    debug!("Getting list of todos");
    let list = repository.get_list().await.expect("Failed to get msgs");
    Json(list)
}

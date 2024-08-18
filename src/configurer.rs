use actix_files::Files;
use actix_web::web::ServiceConfig;

use crate::web;

/// configurer api routes
pub fn web_configurer(config: &mut ServiceConfig) {
    config
    // api routes
    .service(web::todo::api::get_list)
    .service(web::todo::api::create)
    // fallback static
    .service(Files::new("/", "static").index_file("index.html"));
}

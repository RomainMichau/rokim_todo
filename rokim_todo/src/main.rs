use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::todo_controller::{create_todos, delete_todo, get_todos, mark_todo_as_done, mark_todo_as_undone, update_todo};

mod db_client;
mod openapi;
mod todo_controller;

struct AppState {
    db_client: db_client::DbClient,
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let state = web::Data::new(AppState {
        db_client: db_client::DbClient::new().await.unwrap(),
    });
    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .app_data(state.clone())
        .service(create_todos)
        .service(get_todos)
        .service(delete_todo)
        .service(update_todo)
        .service(mark_todo_as_done)
        .service(mark_todo_as_undone)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi::ApiDoc::openapi())),
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
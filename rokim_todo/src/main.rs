use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::dev::ServiceRequest;
use actix_web::middleware::Logger;
use clap::Parser;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openid_middleware::{auth_endpoint, AuthenticateMiddlewareFactory};
use crate::todo_controller::{create_todos, delete_todo, get_todos, mark_todo_as_done, mark_todo_as_undone, update_todo};

mod db_client;
mod openapi;
mod todo_controller;
mod openid;
mod openid_middleware;

struct AppState {
    db_client: db_client::DbClient,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 's', long)]
    db_password: String,
    #[arg(short = 'u', long)]
    db_user: String,
    #[arg(short = 'r', long)]
    db_host: String,
    #[arg(short = 'p', long, default_value_t = 5432)]
    db_port: u16,
    #[arg(short = 'd', long)]
    db_name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let shoud_auth = |req: &ServiceRequest| {
        !req.path().starts_with("/auth")
    };
    let open_id = Arc::new(openid::OpenID::init("rokim_todo".to_string(),
                                                "R3w4cChf8fwmSAB6weXBuAiPdpEA2ytL".to_string(),
                                                "http://localhost:8080/auth".to_string(),
                                                "https://login.romainmic.com/realms/romainmic".to_string()).await.unwrap());


    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let state = web::Data::new(AppState {
        db_client: db_client::DbClient::new(cli.db_host, cli.db_port, cli.db_name, cli.db_user, cli.db_password).await.unwrap(),
    });
    HttpServer::new(move || App::new()
        .wrap(Logger::default())
        .wrap(AuthenticateMiddlewareFactory::new(open_id.clone(), shoud_auth))
        .app_data(state.clone())
        .app_data(web::Data::new(open_id.clone()))
        .service(create_todos)
        .service(get_todos)
        .service(delete_todo)
        .service(update_todo)
        .service(mark_todo_as_done)
        .service(mark_todo_as_undone)
        .service(auth_endpoint)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi::ApiDoc::openapi())),
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
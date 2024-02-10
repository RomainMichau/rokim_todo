use actix_web::dev::ServiceRequest;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::todo_controller::{
    create_todos, delete_todo, get_todos, mark_todo_as_done, mark_todo_as_undone, update_todo,
};

mod db_client;
mod openapi;
mod todo_controller;

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
    #[arg(short, long)]
    open_id_client_id: String,
    #[arg(short, long)]
    open_id_client_secret: String,
    #[arg(short, long)]
    issuer_url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let shoud_auth = |req: &ServiceRequest| !req.path().starts_with("/auth");
    // let open_id = Arc::new(openid::OpenID::init("rokim_todo".to_string(),
    //                                             "R3w4cChf8fwmSAB6weXBuAiPdpEA2ytL".to_string(),
    //                                             "http://localhost:8080/auth".to_string(),
    //                                             "https://login.romainmic.com/realms/romainmic".to_string()).await.unwrap());

    let openid = actix_web_openidconnect::ActixWebOpenId::init(
        cli.open_id_client_id,
        cli.open_id_client_secret,
        "http://localhost:8080/auth".to_string(),
        cli.issuer_url,
        shoud_auth,
        None,
        vec!["openid".to_string()],
    )
    .await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let state = web::Data::new(AppState {
        db_client: db_client::DbClient::new(
            cli.db_host,
            cli.db_port,
            cli.db_name,
            cli.db_user,
            cli.db_password,
        )
        .await
        .unwrap(),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(openid.get_middleware())
            .wrap(Logger::default())
            .configure(openid.configure_open_id())
            .app_data(state.clone())
            .service(create_todos)
            .service(get_todos)
            .service(delete_todo)
            .service(update_todo)
            .service(mark_todo_as_done)
            .service(mark_todo_as_undone)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

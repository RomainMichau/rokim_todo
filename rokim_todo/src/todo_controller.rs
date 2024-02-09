use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use utoipa::ToSchema;

use crate::{db_client, AppState};

#[derive(serde::Serialize, ToSchema)]
struct TodoResponse {
    todo: db_client::Todo,
}

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    title: String,
    category: String,
    description: Option<String>,
}

#[utoipa::path(
responses(
(status = 201, description = "Todo was created", body = Todo),
),
)]
#[post("/api/v1/todos")]
async fn create_todos(
    state: web::Data<AppState>,
    todos: web::Json<CreateTodoRequest>,
) -> actix_web::Result<impl Responder> {
    let todo = state
        .db_client
        .create_todo(&todos.description, &todos.category, &todos.title)
        .await
        .unwrap();
    Ok(web::Json(todo))
}

#[utoipa::path(
responses(
(status = 200, body = Vec<Todo>),
),
)]
#[get("/api/v1/todos")]
async fn get_todos(state: web::Data<AppState>) -> actix_web::Result<impl Responder> {
    let todos = state.db_client.get_todos().await.unwrap();
    Ok(HttpResponse::Created().json(todos))
}

#[utoipa::path(
responses(
(status = 204, description = "Todo was deleted"),
),
)]
#[delete("/api/v1/todos/{id}")]
async fn delete_todo(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    state.db_client.delete_todo(id.into_inner()).await.unwrap();
    Ok(HttpResponse::NoContent())
}

#[utoipa::path(
responses(
(status = 200, description = "Todo was updated", body = Todo),
),
)]
#[put("/api/v1/todos/{id}")]
async fn update_todo(
    state: web::Data<AppState>,
    id: web::Path<i64>,
    todo: web::Json<CreateTodoRequest>,
) -> actix_web::Result<impl Responder> {
    let todo = state
        .db_client
        .update_todo(
            id.into_inner(),
            &todo.description,
            &todo.title,
            &todo.category,
        )
        .await
        .unwrap();
    Ok(web::Json(todo))
}

#[utoipa::path(
responses(
(status = 200, description = "Todo was marked as done", body = Todo),
),
)]
#[post("/api/v1/todos/{id}/done")]
async fn mark_todo_as_done(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let todo = state
        .db_client
        .mark_todo_as_done(id.into_inner())
        .await
        .unwrap();
    Ok(web::Json(todo))
}

#[utoipa::path(
responses(
(status = 200, description = "Todo was marked as to do", body = Todo),
),
)]
#[post("/api/v1/todos/{id}/to_do")]
async fn mark_todo_as_undone(
    state: web::Data<AppState>,
    id: web::Path<i64>,
) -> actix_web::Result<impl Responder> {
    let todo = state
        .db_client
        .mark_todo_as_undone(id.into_inner())
        .await
        .unwrap();
    Ok(web::Json(todo))
}

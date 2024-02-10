use crate::todo_controller;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        todo_controller::create_todos,
        todo_controller::get_todos,
        todo_controller::delete_todo,
        todo_controller::update_todo,
        todo_controller::mark_todo_as_done,
        todo_controller::mark_todo_as_undone,
    ),
    components(schemas(todo_controller::CreateTodoRequest, crate::db_client::Todo))
)]
pub(crate) struct ApiDoc;

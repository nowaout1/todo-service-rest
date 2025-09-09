use axum::{
    Router,
    extract::{Path, Query},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct TodoDto {
    id: String,
    title: String,
    description: String,
    is_done: bool,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct CreateTodoResponse {
    todo: TodoDto,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct UpdateTodoRequest {
    title: Option<String>,
    descriotion: Option<String>,
    is_done: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct UpdateTodoResponse {
    todo: TodoDto,
}

pub fn create_todo_router() -> Router {
    Router::new()
        .route("/api/v1/todo/{id}", get(get_todo))
        .route("/api/v1/create_todo", post(create_todo))
        .route("/api/v1/udate_todo/{id}", put(update_todo))
        .route("/api/v1/delete_todo/{id}", delete(delete_todo))
}

async fn get_todo(Path(id): Path<String>) -> impl IntoResponse {
    ()
}

async fn create_todo(
    Query(CreateTodoRequest { title, description }): Query<CreateTodoRequest>,
) -> impl IntoResponse {
    ()
}

async fn update_todo(
    Path(id): Path<String>,
    Query(UpdateTodoRequest {
        title,
        descriotion,
        is_done,
    }): Query<UpdateTodoRequest>,
) -> impl IntoResponse {
    ()
}

async fn delete_todo(Path(id): Path<String>) -> impl IntoResponse {
    ()
}

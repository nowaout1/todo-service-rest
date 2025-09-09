use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, ()>;

#[async_trait]
pub trait TodoRepository {
    async fn fetch_todo_by_id(&self, req: FetchTodoByIdRequest) -> Result<FetchTodoByIdResponse>;
    async fn fetch_todos_by_range(
        &self,
        req: FetchTodosByRangeRequest,
    ) -> Result<FetchTodosByRangeResponse>;
    async fn create_todo(&self, req: CreateTodoRequest) -> Result<CreateTodoResponse>;
    async fn update_todo(&self, req: UpdateTodoRequest) -> Result<UpdateTodoResponse>;
    async fn delete_todo(&self, req: DeleteTodoRequest) -> Result<DeleteTodoResponse>;
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct FetchTodoByIdRequest {
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct FetchTodoByIdResponse {
    todo: Option<Todo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct FetchTodosByRangeRequest {
    identifiers: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct FetchTodosByRangeResponse {
    todos: Vec<Todo>,
    count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct CreateTodoRequest {
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct CreateTodoResponse {
    todo: Todo,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct UpdateTodoRequest {
    id: Uuid,
    title: Option<String>,
    description: Option<String>,
    is_done: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct UpdateTodoResponse {
    todo: Option<Todo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct DeleteTodoRequest {
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct DeleteTodoResponse {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default, Hash)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
    is_done: bool,
    updated_at: DateTime<Utc>,
}

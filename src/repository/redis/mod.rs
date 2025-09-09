use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::repository::{
    self, CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse,
    FetchTodoByIdRequest, FetchTodoByIdResponse, FetchTodosByRangeRequest,
    FetchTodosByRangeResponse, TodoRepository, UpdateTodoRequest, UpdateTodoResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TodoRedis {}

#[async_trait]
impl TodoRepository for TodoRedis {
    async fn fetch_todo_by_id(
        &self,
        req: FetchTodoByIdRequest,
    ) -> repository::Result<FetchTodoByIdResponse> {
        todo!()
    }

    async fn fetch_todos_by_range(
        &self,
        req: FetchTodosByRangeRequest,
    ) -> repository::Result<FetchTodosByRangeResponse> {
        todo!()
    }

    async fn create_todo(&self, req: CreateTodoRequest) -> repository::Result<CreateTodoResponse> {
        todo!()
    }

    async fn update_todo(&self, req: UpdateTodoRequest) -> repository::Result<UpdateTodoResponse> {
        todo!()
    }

    async fn delete_todo(&self, req: DeleteTodoRequest) -> repository::Result<DeleteTodoResponse> {
        todo!()
    }
}

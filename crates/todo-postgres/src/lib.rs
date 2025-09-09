use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::db::{
    self, CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse,
    FetchTodoByIdRequest, FetchTodoByIdResponse, FetchTodosByRangeRequest,
    FetchTodosByRangeResponse, TodoRepository, UpdateTodoRequest, UpdateTodoResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TodoPostgres {}

#[async_trait]
impl TodoRepository for TodoPostgres {
    async fn fetch_todo_by_id(
        &self,
        req: FetchTodoByIdRequest,
    ) -> db::Result<FetchTodoByIdResponse> {
        todo!()
    }

    async fn fetch_todos_by_range(
        &self,
        req: FetchTodosByRangeRequest,
    ) -> db::Result<FetchTodosByRangeResponse> {
        todo!()
    }

    async fn create_todo(&self, req: CreateTodoRequest) -> db::Result<CreateTodoResponse> {
        todo!()
    }

    async fn update_todo(&self, req: UpdateTodoRequest) -> db::Result<UpdateTodoResponse> {
        todo!()
    }

    async fn delete_todo(&self, req: DeleteTodoRequest) -> db::Result<DeleteTodoResponse> {
        todo!()
    }
}

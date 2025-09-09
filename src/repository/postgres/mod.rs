use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::repository::{
    self, CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest, DeleteTodoResponse,
    FetchTodoByIdRequest, FetchTodoByIdResponse, FetchTodosByRangeRequest,
    FetchTodosByRangeResponse, Todo, TodoRepository, UpdateTodoRequest, UpdateTodoResponse,
};

pub mod error;

const DEFAULT_TODOS_MIN_FETCH: usize = 4;
const DEFAULT_TODOS_MAX_FETCH: usize = 64;

#[derive(Debug, Clone)]
pub struct TodoPostgres {
    pool: PgPool,
}

impl TodoPostgres {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl TodoRepository for TodoPostgres {
    async fn fetch_todo_by_id(
        &self,
        req: FetchTodoByIdRequest,
    ) -> repository::Result<FetchTodoByIdResponse> {
        let FetchTodoByIdRequest { id } = req;

        let todo = sqlx::query_as!(
            Todo,
            r#"
                SELECT id, title, description, is_done, updated_at
                FROM todos
                WHERE id = $1
                LIMIT 1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(FetchTodoByIdResponse { todo })
    }

    async fn fetch_todos_by_range(
        &self,
        req: FetchTodosByRangeRequest,
    ) -> repository::Result<FetchTodosByRangeResponse> {
        let FetchTodosByRangeRequest { offset, limit } = req;

        let limit = limit.clamp(DEFAULT_TODOS_MIN_FETCH, DEFAULT_TODOS_MAX_FETCH);

        let todos = sqlx::query_as!(
            Todo,
            r#"
                SELECT id, title, description, is_done, updated_at
                FROM todos
                LIMIT $1
                OFFSET $2
            "#,
            limit as i32,
            offset as i32
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(FetchTodosByRangeResponse {
            count: todos.len(),
            todos,
        })
    }

    async fn create_todo(&self, req: CreateTodoRequest) -> repository::Result<CreateTodoResponse> {
        let CreateTodoRequest { title, description } = req;

        let id = Uuid::now_v7();

        let todo = sqlx::query_as!(
            Todo,
            r#"
                    INSERT INTO todos (id, title, description)
                    VALUES ($1, $2, $3)
                    RETURNING id, title, description, is_done, updated_at
                "#,
            id,
            title,
            description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateTodoResponse { todo })
    }

    async fn update_todo(&self, req: UpdateTodoRequest) -> repository::Result<UpdateTodoResponse> {
        let UpdateTodoRequest {
            id,
            title,
            description,
            is_done,
        } = req;

        let todo = sqlx::query_as!(
            Todo,
            r#"
                UPDATE todos
                SET
                    title = COALESCE($1, title),
                    description = COALESCE($2, description),
                    is_done = COALESCE($3, is_done)
                WHERE id = $4
                RETURNING id, title, description, is_done, updated_at
            "#,
            title,
            description,
            is_done.unwrap_or(false),
            id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(UpdateTodoResponse { todo })
    }

    async fn delete_todo(&self, req: DeleteTodoRequest) -> repository::Result<DeleteTodoResponse> {
        let DeleteTodoRequest { id } = req;

        let was_found = sqlx::query!(
            r#"
                DELETE FROM todos
                WHERE id = $1
                RETURNING TRUE as was_found
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();

        Ok(DeleteTodoResponse { was_found })
    }
}

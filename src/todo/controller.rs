use poem::{ web::{ Data, Query }, error::NotFound };
use poem_openapi::{ payload::{ PlainText, Json }, OpenApi, param::Path };
use sqlx::PgPool;

use super::{
    types::{ TodoCreateResponse, TodoManyResponse, TodoOneResponse, TodoDeleteResponse },
    db::{ add_todo, get_todo_many, get_todo_by_id, complete_todo_by_id, delete_todo_by_id },
};

pub struct TodosApi;

#[OpenApi]
impl TodosApi {
    #[oai(path = "/", method = "post")]
    async fn create(
        &self,
        pool: Data<&PgPool>,
        description: PlainText<String>
    ) -> TodoCreateResponse {
        let id = add_todo(pool.0, description.0).await.unwrap();
        Ok(Json(id))
    }

    #[oai(path = "/", method = "get")]
    async fn get(
        &self,
        pool: Data<&PgPool>,
        limit: Query<Option<i64>>,
        offset: Query<Option<i64>>
    ) -> TodoManyResponse {
        let recs = get_todo_many(pool.0, limit.0, offset.0).await;

        let res = match recs {
            Ok(val) => val,
            Err(err) => {
                return Err(NotFound(err));
            }
        };

        Ok(Json(res))
    }

    #[oai(path = "/:id", method = "get")]
    async fn get_by_id(&self, pool: Data<&PgPool>, id: Path<i64>) -> TodoOneResponse {
        let rec = get_todo_by_id(pool.0, id.0).await;

        let res = match rec {
            Ok(val) => val,
            Err(err) => {
                return Err(NotFound(err));
            }
        };

        Ok(Json(res))
    }

    #[oai(path = "/:id", method = "post")]
    async fn complete(&self, pool: Data<&PgPool>, id: Path<i64>) -> TodoOneResponse {
        let rec = complete_todo_by_id(pool.0, id.0).await;

        let res = match rec {
            Ok(val) => val,
            Err(err) => {
                return Err(NotFound(err));
            }
        };

        Ok(Json(res))
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete(&self, pool: Data<&PgPool>, id: Path<i64>) -> TodoDeleteResponse {
        let res = match delete_todo_by_id(pool.0, id.0).await {
            Ok(_) => true,
            Err(err) => {
                return Err(NotFound(err));
            }
        };

        Ok(Json(res))
    }
}

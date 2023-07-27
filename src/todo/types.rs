use poem::Result;
use poem_openapi::{ Object, payload::Json };

#[derive(Object)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

pub type TodoCreateResponse = Result<Json<i64>>;
pub type TodoManyResponse = Result<Json<Vec<Todo>>>;
pub type TodoOneResponse = Result<Json<Todo>>;
pub type TodoDeleteResponse = Result<Json<bool>>;

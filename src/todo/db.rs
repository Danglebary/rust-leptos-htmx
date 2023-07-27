use sqlx::{ Postgres, PgPool, Pool };

use crate::config::Config;
use super::types::Todo;

pub async fn connect(config: Config) -> Pool<Postgres> {
    PgPool::connect(&config.database_url).await.expect("could not connect to DB")
}

pub async fn add_todo(pool: &PgPool, description: String) -> Result<i64, sqlx::error::Error> {
    // Insert the task, then obtain the ID of this row
    let query = sqlx::query!(
        r#"
            INSERT INTO todos ( description )
            VALUES ( $1 )
            RETURNING id
        "#,
        description
    );

    let rec = query.fetch_one(pool).await?;

    Ok(rec.id)
}

pub async fn get_todo_by_id(pool: &PgPool, id: i64) -> Result<Todo, sqlx::error::Error> {
    let query = sqlx::query_as!(
        Todo,
        r#"
            SELECT * 
            FROM todos 
            WHERE id = ( $1 )
        "#,
        id
    );

    let rec = query.fetch_one(pool).await?;

    Ok(rec)
}

pub async fn get_todo_many(
    pool: &PgPool,
    limit: Option<i64>,
    offset: Option<i64>
) -> Result<Vec<Todo>, sqlx::error::Error> {
    let _limit = limit.unwrap_or(100);
    let _offset = offset.unwrap_or(0);

    let query = sqlx::query_as!(
        Todo,
        r#"
            SELECT * 
            FROM todos 
            ORDER BY id
            LIMIT ( $1 ) 
            OFFSET ( $2 )
        "#,
        _limit,
        _offset
    );

    let recs = query.fetch_all(pool).await?;
    Ok(recs)
}

pub async fn complete_todo_by_id(pool: &PgPool, id: i64) -> Result<Todo, sqlx::error::Error> {
    let query = sqlx::query_as!(
        Todo,
        r#"
            UPDATE todos
            SET done = TRUE
            WHERE id = ( $1 )
            RETURNING *
        "#,
        id
    );

    let rec = query.fetch_one(pool).await?;
    Ok(rec)
}

pub async fn delete_todo_by_id(pool: &PgPool, id: i64) -> Result<bool, sqlx::error::Error> {
    let query = sqlx::query!(
        r#"
            DELETE FROM todos
            WHERE id = ( $1 )
        "#,
        id
    );

    query.execute(pool).await?;
    Ok(true)
}

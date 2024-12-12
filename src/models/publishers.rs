    use tokio_postgres::{Client, Error};

use serde_json::Value;
use tokio_postgres::types::ToSql;


pub async fn fetch_publishers_latest(client: &Client) -> Result<Vec<Value>, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT id, title, slug, image FROM publishers WHERE status = 'PUBLISHED' LIMIT 20",
            &[],
        )
        .await?;

    let publishers: Vec<Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<_, i64>(0),
                "title": row.get::<_, String>(1),
                "slug": row.get::<_, String>(2),
                "image": row.get::<_, String>(3),
            })
        })
        .collect();

    Ok(publishers)
}

pub async fn fetch_publishers_featured(client: &Client) -> Result<Vec<Value>, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT id, title, slug, image FROM publishers WHERE status = 'PUBLISHED' AND featured = 'f' LIMIT 5",
            &[],
        )
        .await?;

    let publishers: Vec<Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<_, i64>(0),
                "title": row.get::<_, String>(1),
                "slug": row.get::<_, String>(2),
                "image": row.get::<_, String>(3),
            })
        })
        .collect();

    Ok(publishers)
}

pub async fn fetch_publishers_by_category(
    client: &Client,
    category_id: i32,
) -> Result<Vec<Value>, Error> {
    let stmt = client.prepare(
        "SELECT id, title, slug, image, category_id 
         FROM publishers 
         WHERE category_id = $1 AND status = 'PUBLISHED' 
         LIMIT 20"
    ).await?;

    let rows = client.query(&stmt, &[(&category_id as &(dyn ToSql + Sync))]).await?;

    let publishers: Vec<Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<_, i64>(0),
                "title": row.get::<_, String>(1),
                "slug": row.get::<_, String>(2),
                "image": row.get::<_, String>(3),
                "category_id": row.get::<_, i32>(4),
            })
        })
        .collect();

    Ok(publishers)
}

pub async fn fetch_publishers_by_author(
    client: &Client,
    author_id: i32,
) -> Result<Vec<Value>, Error> {
    let stmt = client.prepare(
        "SELECT id, title, slug, image, category_id, posted_by 
         FROM publishers 
         WHERE posted_by = $1 AND status = 'PUBLISHED' 
         LIMIT 20"
    ).await?;

    let rows = client.query(&stmt, &[(&author_id as &(dyn ToSql + Sync))]).await?;

    let publishers: Vec<Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<_, i64>(0),
                "title": row.get::<_, String>(1),
                "slug": row.get::<_, String>(2),
                "image": row.get::<_, String>(3),
                "category_id": row.get::<_, i32>(4),
                "posted_by": row.get::<_, i32>(5),

            })
        })
        .collect();

    Ok(publishers)
}

pub async fn fetch_publishers_by_search(
    client: &Client,
    search_query: &str,
) -> Result<Vec<Value>, Error> {
    // Prepare SQL query for search with ILIKE
    let stmt = client.prepare(
        "SELECT id, title, slug, image, category_id, posted_by 
         FROM publishers 
         WHERE title ILIKE $1 AND status = 'PUBLISHED' 
         LIMIT 20"
    ).await?;

    let rows = client.query(&stmt, &[&format!("%{}%", search_query)]).await?;

    // Map the rows into a vector of JSON objects
    let publishers: Vec<Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<_, i64>(0),
                "title": row.get::<_, String>(1),
                "slug": row.get::<_, String>(2),
                "image": row.get::<_, String>(3),
                "category_id": row.get::<_, i32>(4),
                "posted_by": row.get::<_, i32>(5),
            })
        })
        .collect();

    Ok(publishers)
}








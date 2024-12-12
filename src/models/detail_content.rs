use tokio_postgres::Client;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DetailContent {
    pub id: i64,
    pub posted_by: i32,
    pub category_id: Option<i32>,
    pub title: String,
    pub summary: Option<String>,
    pub body: String,
    pub image: Option<String>,
    pub image_caption: Option<String>,
    pub preview_video: Option<String>,
    pub thumbnail_portrait: Option<String>,
    pub thumbnail_landscape: Option<String>,
    pub thumbnail_caption: Option<String>,
    pub slug: String,
    pub status: String,
    pub featured: Option<bool>,
    pub reported_by: Option<i32>,
    pub edited_by: Option<i32>,
    pub content_type: String,
    pub editor_choice: Option<bool>,
    pub image_cropped: Option<String>,
    pub image_medium: Option<String>,
    pub image_small: Option<String>,
    pub pinned: Option<bool>,
    pub subdomain: Option<String>,
}

impl DetailContent {
    pub async fn get_detail_by_id(client: &Client, content_id: i64) -> Result<Option<DetailContent>, Box<dyn std::error::Error>> {
        let query = "
            SELECT id, posted_by, category_id, title, summary, body, image, image_caption, preview_video,
                   thumbnail_portrait, thumbnail_landscape, thumbnail_caption, slug, status, featured, reported_by, 
                   edited_by, content_type, editor_choice, image_cropped, image_medium, image_small, pinned, subdomain
            FROM publishers WHERE id = $1
        ";

        let rows = client.query(query, &[&(content_id as i64)]).await?;

        if let Some(row) = rows.first() {
            Ok(Some(DetailContent {
                id: row.get(0),
                posted_by: row.get(1),
                category_id: row.get(2),
                title: row.get(3),
                summary: row.get(4),
                body: row.get(5),
                image: row.get(6),
                image_caption: row.get(7),
                preview_video: row.get(8),
                thumbnail_portrait: row.get(9),
                thumbnail_landscape: row.get(10),
                thumbnail_caption: row.get(11),
                slug: row.get(12),
                status: row.get(13),
                featured: row.get(14),
                reported_by: row.get(15),
                edited_by: row.get(16),
                content_type: row.get(17),
                editor_choice: row.get(18),
                image_cropped: row.get(19),
                image_medium: row.get(20),
                image_small: row.get(21),
                pinned: row.get(22),
                subdomain: row.get(23),
            }))
        } else {
            Ok(None)  // If no rows are found
        }
    }
}

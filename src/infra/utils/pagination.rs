use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    #[serde(flatten)]
    pub pagination: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i32,
    pub items_per_page: i32,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub page: Option<i64>,
}

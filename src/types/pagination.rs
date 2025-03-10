use degen_sql::pagination::PaginationData;
use serde::Serialize;

#[derive(Serialize, utoipa::ToSchema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn from_pagination_data(
        pagination: &PaginationData,
        mut items: Vec<T>,
        total_count: i64,
    ) -> Self {
        let page = pagination.page.unwrap_or(1);
        let page_size = pagination.page_size.unwrap_or(10).min(100);
        let total_pages = (total_count + page_size - 1) / page_size; // Ceiling division

        // Ensure we don't return more items than the requested page_size
        if items.len() > page_size as usize {
            items.truncate(page_size as usize);
        }

        Self {
            items,
            total_count,
            page,
            page_size,
            total_pages,
        }
    }
}

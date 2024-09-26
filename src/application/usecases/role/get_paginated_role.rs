use std::sync::Arc;

use crate::{
    domain::{entities::role::Role, repositories::role_repo::RoleRepository},
    infra::{
        errors::app_error::AppError,
        utils::pagination::{PaginatedResponse, PaginationMeta},
    },
};

#[derive(Clone)]
pub struct GetPaginatedRole<R> {
    role_repo: Arc<R>,
}

impl<R> GetPaginatedRole<R>
where
    R: RoleRepository,
{
    pub fn new(role_repo: Arc<R>) -> Self {
        Self { role_repo }
    }

    pub async fn execute(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<PaginatedResponse<Role>, AppError> {
        let (roles, total_items) = self.role_repo.paginate(page, limit).await?;

        let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

        let pagination = PaginationMeta {
            total_items,
            total_pages,
            current_page: page as i32,
            items_per_page: limit as i32,
        };

        Ok(PaginatedResponse {
            items: roles,
            pagination,
        })
    }
}

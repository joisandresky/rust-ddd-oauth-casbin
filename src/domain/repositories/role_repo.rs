use crate::{domain::entities::role::Role, infra::errors::app_error::AppError};

#[async_trait::async_trait]
pub trait RoleRepository {
    async fn paginate(&self, page: i64, limit: i64) -> Result<(Vec<Role>, i64), AppError>;
    async fn find_all(&self) -> Result<Vec<Role>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Role, AppError>;
    async fn find_default(&self) -> Result<Role, AppError>;
    async fn find_by_name(&self, role_name: &str) -> Result<Role, AppError>;
    async fn create(&self, entity: Role) -> Result<Role, AppError>;
    async fn tx_create(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: Role,
    ) -> Result<Role, AppError>;
    async fn update(&self, id: &str, entity: Role) -> Result<(), AppError>;
    async fn delete(&self, id: &str) -> Result<(), AppError>;

    async fn get_roles_by_user_id(&self, user_id: &str) -> Result<Vec<Role>, AppError>;
}

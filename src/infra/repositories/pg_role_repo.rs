use crate::{
    domain::{
        entities::role::{Role, RoleCount},
        repositories::role_repo::RoleRepository,
    },
    infra::errors::app_error::AppError,
};

#[derive(Debug, Clone)]
pub struct PgRoleRepository {
    pub db_pool: sqlx::PgPool,
}

impl PgRoleRepository {
    pub fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl RoleRepository for PgRoleRepository {
    async fn paginate(&self, page: i64, limit: i64) -> Result<(Vec<Role>, i64), AppError> {
        let offset = (page - 1) * limit;

        let roles = sqlx::query_as!(
            Role,
            "SELECT * FROM roles WHERE deleted_at IS NULL LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;

        let count = sqlx::query_as!(
            RoleCount,
            "SELECT COUNT(*) AS total_items FROM roles WHERE deleted_at IS NULL"
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok((roles, count.total_items.unwrap_or(0)))
    }

    async fn find_all(&self) -> Result<Vec<Role>, AppError> {
        let roles = sqlx::query_as!(Role, "SELECT * FROM roles WHERE deleted_at IS NULL")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(roles)
    }

    async fn find_by_id(&self, id: &str) -> Result<Role, AppError> {
        let role = sqlx::query_as!(
            Role,
            "SELECT * FROM roles WHERE id = $1 AND deleted_at IS NULL",
            id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(role)
    }

    async fn find_default(&self) -> Result<Role, AppError> {
        let role = sqlx::query_as!(
            Role,
            "SELECT * FROM roles WHERE is_default = true AND deleted_at IS NULL"
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(role)
    }

    async fn find_by_name(&self, role_name: &str) -> Result<Role, AppError> {
        let role = sqlx::query_as!(
            Role,
            "SELECT * FROM roles WHERE name = $1 AND deleted_at IS NULL",
            role_name
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(role)
    }

    async fn create(&self, entity: Role) -> Result<Role, AppError> {
        let role = sqlx::query_as!(
            Role,
            "INSERT INTO roles (id, name, is_default) VALUES ($1, $2, $3) RETURNING *",
            entity.id,
            entity.name,
            entity.is_default
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(role)
    }

    async fn tx_create(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: Role,
    ) -> Result<Role, AppError> {
        let role = sqlx::query_as!(
            Role,
            "INSERT INTO roles (id, name, is_default) VALUES ($1, $2, $3) RETURNING *",
            entity.id,
            entity.name,
            entity.is_default
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(role)
    }

    async fn update(&self, id: &str, entity: Role) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE roles SET name = $1, is_default = $2 WHERE id = $3",
            entity.name,
            entity.is_default,
            id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), AppError> {
        let now = chrono::Utc::now();
        sqlx::query!("UPDATE roles SET deleted_at = $2 WHERE id = $1", id, now)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    async fn get_roles_by_user_id(&self, user_id: &str) -> Result<Vec<Role>, AppError> {
        let roles = sqlx::query_as!(
            Role,
            "SELECT roles.* FROM roles INNER JOIN user_roles ON roles.id = user_roles.role_id WHERE user_roles.user_id = $1 AND roles.deleted_at IS NULL",
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(roles)
    }
}

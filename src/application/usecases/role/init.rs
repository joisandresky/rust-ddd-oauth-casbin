use std::sync::Arc;

use crate::infra::{rbac::Rbac, repositories::pg_role_repo::PgRoleRepository};

use super::{
    create_role::CreateRole, delete_role_by_id::DeleteRoleById, get_all_role::GetAllRole,
    get_paginated_role::GetPaginatedRole, get_role_by_id::GetRoleById,
    update_role_by_id::UpdateRoleById,
};

#[derive(Clone)]
pub struct RoleUsecase {
    pub get_paginated_role: Arc<GetPaginatedRole<PgRoleRepository>>,
    pub get_all_role: Arc<GetAllRole<PgRoleRepository>>,
    pub get_role_by_id: Arc<GetRoleById<PgRoleRepository>>,
    pub create_role: Arc<CreateRole<PgRoleRepository>>,
    pub update_role_by_id: Arc<UpdateRoleById<PgRoleRepository>>,
    pub delete_role_by_id: Arc<DeleteRoleById<PgRoleRepository>>,
}

impl RoleUsecase {
    pub fn new(role_repo: Arc<PgRoleRepository>, rbac: Arc<Rbac>) -> Self {
        let get_paginated_role = Arc::new(GetPaginatedRole::new(role_repo.clone()));
        let get_all_role = Arc::new(GetAllRole::new(role_repo.clone()));
        let get_role_by_id = Arc::new(GetRoleById::new(role_repo.clone(), rbac.clone()));
        let create_role = Arc::new(CreateRole::new(role_repo.clone(), rbac.clone()));
        let update_role = Arc::new(UpdateRoleById::new(role_repo.clone(), rbac.clone()));
        let delete_role_by_id = Arc::new(DeleteRoleById::new(role_repo.clone(), rbac.clone()));

        Self {
            get_paginated_role,
            get_all_role,
            get_role_by_id,
            create_role,
            update_role_by_id: update_role,
            delete_role_by_id,
        }
    }
}

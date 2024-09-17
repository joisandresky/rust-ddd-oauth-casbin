use std::sync::Arc;

use crate::infra::{
    config::AppConfig,
    rbac::Rbac,
    repositories::{
        pg_oauth_provider::PgOauthProviderRepository, pg_role_repo::PgRoleRepository,
        pg_user_repo::PgUserRepository, pg_user_session::PgUserSessionRepository,
        redis_repo_impl::RedisRepositoryImpl,
    },
    utils::{google_jwt::GoogleJwtMaker, jwt_maker::JwtMaker},
};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use sqlx::PgPool;

use super::{
    services::{oauth_svc::OauthService, redis_svc::RedisService},
    usecases::{
        auth::{
            email_login::EmailLogin, email_register::EmailRegister,
            get_google_auth_url::GetGoogleAuthUrl, oauth2_login::Oauth2Login,
            oauth2_logout::Oauth2Logout, seed_super_admin::SeedSuperAdmin,
        },
        role::{
            create_role::CreateRole, delete_role_by_id::DeleteRoleById, get_all_role::GetAllRole,
            get_role_by_id::GetRoleById, update_role_by_id::UpdateRoleById,
        },
    },
};

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<AppConfig>,
    pub db_pool: PgPool,
    pub jwt_maker: Arc<JwtMaker>,
    pub google_jwt_maker: Arc<GoogleJwtMaker>,
    pub rbac: Arc<Rbac>,
    pub svc: Arc<Service>,
    pub uc: Arc<Usecase>,
}

/* Usecases list */
#[derive(Clone)]
pub struct Usecase {
    pub role: Arc<RoleUsecase>,
    pub auth: Arc<AuthUsecase>,
}

#[derive(Clone)]
pub struct RoleUsecase {
    pub get_all_role: Arc<GetAllRole<PgRoleRepository>>,
    pub get_role_by_id: Arc<GetRoleById<PgRoleRepository>>,
    pub create_role: Arc<CreateRole<PgRoleRepository>>,
    pub update_role_by_id: Arc<UpdateRoleById<PgRoleRepository>>,
    pub delete_role_by_id: Arc<DeleteRoleById<PgRoleRepository>>,
}
/* End Usecases list */

/* Auth Usecases*/
#[derive(Clone)]
pub struct AuthUsecase {
    pub get_google_auth_url: Arc<GetGoogleAuthUrl>,
    pub oauth2_login: Arc<
        Oauth2Login<
            PgUserRepository,
            PgRoleRepository,
            PgUserSessionRepository,
            PgOauthProviderRepository,
        >,
    >,
    pub oauth2_logout: Arc<
        Oauth2Logout<
            PgUserRepository,
            PgRoleRepository,
            PgUserSessionRepository,
            PgOauthProviderRepository,
        >,
    >,
    pub email_register: Arc<EmailRegister<PgUserRepository, PgRoleRepository>>,
    pub email_login: Arc<
        EmailLogin<
            PgUserRepository,
            PgRoleRepository,
            PgUserSessionRepository,
            PgOauthProviderRepository,
        >,
    >,
    pub seed_super_admin: Arc<SeedSuperAdmin<PgUserRepository, PgRoleRepository>>,
}
/* End Auth Usecases */

#[derive(Clone)]
pub struct Service {
    pub oauth: Arc<
        OauthService<
            PgUserRepository,
            PgRoleRepository,
            PgUserSessionRepository,
            PgOauthProviderRepository,
        >,
    >,
    pub redis: Arc<RedisService<RedisRepositoryImpl>>,
}

impl AppState {
    pub fn new(
        cfg: Arc<AppConfig>,
        db_pool: PgPool,
        redis_pool: Pool<RedisConnectionManager>,
        rbac: Arc<Rbac>,
    ) -> Self {
        // utils or tooling
        let jwt_maker = Arc::new(JwtMaker::new(cfg.jwt_secret.clone()));
        let google_jwt_maker = Arc::new(GoogleJwtMaker::new(cfg.clone()));
        let redis_repo = Arc::new(RedisRepositoryImpl::new(redis_pool.clone()));

        // repos list
        let role_repo = Arc::new(PgRoleRepository::new(db_pool.clone()));
        let user_repo = Arc::new(PgUserRepository::new(db_pool.clone()));
        let user_session_repo = Arc::new(PgUserSessionRepository::new(db_pool.clone()));
        let oauth_provider_repo = Arc::new(PgOauthProviderRepository::new(db_pool.clone()));

        // services list
        let redis_svc = Arc::new(RedisService::new(redis_repo.clone()));
        let oauth_svc = Arc::new(OauthService::new(
            cfg.clone(),
            user_repo.clone(),
            role_repo.clone(),
            user_session_repo.clone(),
            oauth_provider_repo.clone(),
        ));

        // usecases list
        // Role UC
        let get_all_role = Arc::new(GetAllRole::new(role_repo.clone()));
        let get_role_by_id = Arc::new(GetRoleById::new(role_repo.clone(), rbac.clone()));
        let create_role = Arc::new(CreateRole::new(role_repo.clone(), rbac.clone()));
        let update_role = Arc::new(UpdateRoleById::new(role_repo.clone(), rbac.clone()));
        let delete_role_by_id = Arc::new(DeleteRoleById::new(role_repo.clone(), rbac.clone()));

        // Auth UC
        let get_google_auth_url = Arc::new(GetGoogleAuthUrl::new(
            cfg.google_client_id.clone(),
            cfg.google_redirect_url.clone(),
        ));
        let oauth2_login = Arc::new(Oauth2Login::new(oauth_svc.clone()));
        let oauth2_logout = Arc::new(Oauth2Logout::new(
            user_session_repo.clone(),
            oauth_svc.clone(),
            redis_svc.clone(),
        ));
        let email_register = Arc::new(EmailRegister::new(user_repo.clone(), role_repo.clone()));
        let email_login = Arc::new(EmailLogin::new(
            user_repo.clone(),
            jwt_maker.clone(),
            oauth_svc.clone(),
        ));
        let seed_super_admin = Arc::new(SeedSuperAdmin::new(
            user_repo.clone(),
            role_repo.clone(),
            redis_svc.clone(),
            rbac.clone(),
        ));

        // service registration
        let svc = Arc::new(Service {
            oauth: oauth_svc,
            redis: redis_svc,
        });

        // usecase registration
        let uc = Arc::new(Usecase {
            role: Arc::new(RoleUsecase {
                get_all_role,
                get_role_by_id,
                create_role,
                update_role_by_id: update_role,
                delete_role_by_id,
            }),
            auth: Arc::new(AuthUsecase {
                get_google_auth_url,
                oauth2_login,
                oauth2_logout,
                email_register,
                email_login,
                seed_super_admin,
            }),
        });

        Self {
            cfg,
            db_pool,
            jwt_maker,
            google_jwt_maker,
            rbac,
            svc,
            uc,
        }
    }
}

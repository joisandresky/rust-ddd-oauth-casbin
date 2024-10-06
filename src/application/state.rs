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
    usecases::{auth::init::AuthUsecase, role::init::RoleUsecase},
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

/* End Usecases list */

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

        // service registration
        let svc = Arc::new(Service {
            oauth: oauth_svc,
            redis: redis_svc,
        });

        // usecase registration
        let uc = Arc::new(Usecase {
            role: Arc::new(RoleUsecase::new(role_repo.clone(), rbac.clone())),
            auth: Arc::new(AuthUsecase::new(
                cfg.clone(),
                svc.oauth.clone(),
                rbac.clone(),
                user_repo.clone(),
                role_repo.clone(),
                user_session_repo.clone(),
                jwt_maker.clone(),
                svc.redis.clone(),
            )),
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

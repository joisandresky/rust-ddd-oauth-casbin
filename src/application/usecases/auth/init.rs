use std::sync::Arc;

use crate::{
    application::services::{oauth_svc::OauthService, redis_svc::RedisService},
    infra::{
        config::AppConfig,
        rbac::Rbac,
        repositories::{
            pg_oauth_provider::PgOauthProviderRepository, pg_role_repo::PgRoleRepository,
            pg_user_repo::PgUserRepository, pg_user_session::PgUserSessionRepository,
            redis_repo_impl::RedisRepositoryImpl,
        },
        utils::jwt_maker::JwtMaker,
    },
};

use super::{
    email_login::EmailLogin, email_register::EmailRegister, get_google_auth_url::GetGoogleAuthUrl,
    oauth2_login::Oauth2Login, oauth2_logout::Oauth2Logout, refresh_oauth_token::RefreshOauthToken,
    seed_super_admin::SeedSuperAdmin,
};

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
    pub refresh_oauth_token: Arc<
        RefreshOauthToken<
            PgUserRepository,
            PgRoleRepository,
            PgUserSessionRepository,
            PgOauthProviderRepository,
        >,
    >,
}

impl AuthUsecase {
    pub fn new(
        cfg: Arc<AppConfig>,
        oauth_svc: Arc<
            OauthService<
                PgUserRepository,
                PgRoleRepository,
                PgUserSessionRepository,
                PgOauthProviderRepository,
            >,
        >,
        rbac: Arc<Rbac>,
        user_repo: Arc<PgUserRepository>,
        role_repo: Arc<PgRoleRepository>,
        user_session_repo: Arc<PgUserSessionRepository>,
        jwt_maker: Arc<JwtMaker>,
        redis_svc: Arc<RedisService<RedisRepositoryImpl>>,
    ) -> Self {
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
            rbac.clone(),
        ));
        let refresh_oauth_token = Arc::new(RefreshOauthToken::new(
            jwt_maker.clone(),
            user_session_repo.clone(),
            oauth_svc.clone(),
        ));

        Self {
            get_google_auth_url,
            oauth2_login,
            oauth2_logout,
            email_register,
            email_login,
            seed_super_admin,
            refresh_oauth_token,
        }
    }
}

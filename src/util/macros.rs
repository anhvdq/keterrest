#[macro_export]
macro_rules! permission_required {
    ($($perm:expr),+ $(,)?) => {{
        use $crate::util::api_response::ServiceError;
        use axum::{
            extract::Request,
            middleware::{from_fn, Next},
            response::IntoResponse,
            Extension,
        };
        async fn check(Extension(auth_info): Extension<AuthInfo>, req: Request, next: Next) -> Result<impl IntoResponse, ApiError> {
            $(
                if (!auth_info.permissions.contains(&$perm.into())) {
                    return Err(ServiceError::MissingRequiredPermission(String::from($perm.as_str())).into());
                }
            )+
            Ok(next.run(req).await)
        }

        from_fn(check)
    }};
}

use axum::{
    extract::{Extension, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use common::{
    contextx::AppContext,
    jwt::{self},
};

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Extension(mut ctx): Extension<AppContext>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let claim = match jwt::decode_token(ctx.locale, auth.token()) {
        Ok(r) => r,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    ctx.uid = claim.uid;
    ctx.tid = claim.tid;
    req.extensions_mut().insert(ctx).unwrap();

    Ok(next.run(req).await)
}

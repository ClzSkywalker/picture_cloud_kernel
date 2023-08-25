use axum::{
    extract::{
        rejection::{FormRejection, JsonRejection},
        FromRequest,
    },
    response::{IntoResponse, Response},
    Form, Json,
};

use hyper::{Request, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

// todo 把用户id等信息也记录进日志
#[async_trait::async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let uri = req.uri().clone();
        let path = uri.path();
        let query = uri.query().unwrap_or_default();

        let header = req.headers().clone();
        let version = match header.get("verison") {
            Some(r) => r.to_str().unwrap_or_default(),
            None => "",
        };

        let Form(value) = match Form::<T>::from_request(req, state).await {
            Ok(r) => r,
            Err(e) => {
                return Err(ServerError::AxumFormRejection(
                    e,
                    version.to_string(),
                    path.to_string(),
                    query.to_string(),
                ))
            }
        };
        match value.validate() {
            Ok(_) => {}
            Err(e) => {
                return Err(ServerError::ValidationError(
                    e,
                    path.to_string(),
                    query.to_string(),
                ))
            }
        };
        Ok(ValidatedForm(value))
    }
}

pub struct ValidatedJson<J>(pub J);

#[async_trait::async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = ServerError;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let uri = req.uri().clone();
        let path = uri.path().to_string();
        let query = uri.query().unwrap_or_default();

        let header = req.headers().clone();
        let version = match header.get("version") {
            Some(r) => r.to_str().unwrap_or_default(),
            None => "",
        };

        let Json(value) = match Json::<T>::from_request(req, state).await {
            Ok(r) => r,
            Err(e) => {
                return Err(ServerError::AxumJsonRejection(
                    e,
                    version.to_string(),
                    path.to_string(),
                    query.to_string(),
                ));
            }
        };
        match value.validate() {
            Ok(_) => {}
            Err(e) => {
                return Err(ServerError::ValidationError(
                    e,
                    path.to_string(),
                    query.to_string(),
                ))
            }
        };
        Ok(Self(value))
    }
}

pub enum ServerError {
    ValidationError(validator::ValidationErrors, String, String),
    AxumJsonRejection(JsonRejection, String, String, String),
    AxumFormRejection(FormRejection, String, String, String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(item, path, query) => {
                tracing::error!("err:{},path:{},query:{}", item.to_string(), path, query);
                bad_request_resp()
            }
            ServerError::AxumJsonRejection(item, version, path, query) => {
                tracing::error!(
                    "err:{},verison:{},path:{},query:{}",
                    item.to_string(),
                    version,
                    path,
                    query
                );
                bad_request_resp()
            }
            ServerError::AxumFormRejection(item, version, path, query) => {
                tracing::error!(
                    "err:{},verison:{},path:{},query:{}",
                    item.to_string(),
                    version,
                    path,
                    query
                );
                bad_request_resp()
            }
        }
        .into_response()
    }
}

fn bad_request_resp() -> (StatusCode, String) {
    let m = r#"{"code":"400","msg":"Parameter check failure"}"#;
    let message: Value = serde_json::from_str(m).unwrap();
    (StatusCode::BAD_REQUEST, message.to_string())
}

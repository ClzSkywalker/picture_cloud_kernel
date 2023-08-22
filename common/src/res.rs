use std::fmt::Debug;

use axum::{
    body::{self, Full},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use super::{
    errorx::Errorx,
    i18n::{I18nKey, Locale},
};
use serde::Serialize;

const EMPTY_MSG: String = String::new();

#[derive(Debug, Serialize, Default)]
pub struct Responsex<T>
where
    T: Serialize,
{
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: Serialize> From<Errorx> for Responsex<T> {
    fn from(val: Errorx) -> Self {
        Self {
            code: val.id.id(),
            msg: val.msg,
            data: None,
        }
    }
}

/// 填入到extensions中的数据
#[derive(Debug)]
pub struct ResJsonString(pub String);

impl<T> IntoResponse for Responsex<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(r) => r,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("response error"),
                    )
                    .body(body::boxed(Full::from(e.to_string())))
                    .unwrap();
            }
        };
        let res_json_string = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T: Serialize> Responsex<T> {
    pub fn ok(locale: Locale) -> Self {
        Self {
            code: I18nKey::Ok.id(),
            msg: I18nKey::Ok.trans(locale),
            data: None,
        }
    }

    #[allow(dead_code)]
    pub fn ok_with_msg(msg: String) -> Self {
        Self {
            code: I18nKey::Ok.id(),
            msg: msg,
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            code: I18nKey::Ok.id(),
            msg: EMPTY_MSG,
            data: Some(data),
        }
    }

    pub fn err_with_req(locale: Locale) -> Self {
        Self {
            code: I18nKey::DbCreate.id(),
            msg: I18nKey::DbCreate.trans(locale),
            data: None,
        }
    }

    // #[allow(dead_code)]
    // pub fn err_with_data(code: i32, data: T) -> Self {
    //     Self {
    //         code,
    //         msg: EMPTY_MSG,
    //         data: Some(data),
    //     }
    // }
}

pub fn err_to_resp<T>(e: anyhow::Error, locale: Locale) -> Responsex<T>
where
    T: Serialize,
{
    match e.downcast_ref::<Errorx>() {
        Some(my_error) => my_error.clone().into(),
        None => Responsex::err_with_req(locale),
    }
}

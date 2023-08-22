use chrono::{Duration, Local};
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::{errorx::Errorx, i18n::Locale};

pub const TOKEN_TYPE: &str = "Bearer";
pub const EXP: usize = 7 * 86400;
const SUB: &str = "1045683477@qq.com";
const COMPANY: &str = "1045683477@qq.com";
const KID: &str = "todo list kernel";
const KEY: &str = "may-the-force-be-with-you";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
    pub tid: String,
    pub uid: String,
}

/**
 * @Author         : ClzSkywalker
 * @Date           : 2023-05-19
 * @Description    : 创建token
 * @param           {String} uid
 * @param           {String} tid
 * @return          {*}
 */
pub fn generate_token(uid: String, tid: String) -> Result<String> {
    let mut claims = Claims {
        sub: SUB.to_string(),
        company: COMPANY.to_string(),
        exp: 0,
        uid,
        tid,
    };
    let exp = Local::now()
        .checked_add_signed(Duration::seconds(EXP.try_into().unwrap()))
        .unwrap()
        .timestamp();
    if exp > usize::MAX as i64 {
        claims.exp = usize::MAX;
    } else {
        claims.exp = exp as usize;
    }

    let header = Header {
        kid: Some(KID.to_string()),
        alg: Algorithm::HS512,
        ..Default::default()
    };

    match encode(&header, &claims, &EncodingKey::from_secret(KEY.as_bytes())) {
        Ok(t) => Ok(t),
        Err(e) => Err(anyhow!(e)),
    }
}

/**
 * @Author         : ClzSkywalker
 * @Date           : 2023-05-19
 * @Description    : 解析token
 * @param           {*} token
 * @return          {*}
 */
pub fn decode_token(local: Locale, token: &str) -> Result<Claims> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(KEY.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(r) => Ok(r.claims),
        Err(e) => {
            if *e.kind() == jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                anyhow::bail!(Errorx::new(local, crate::i18n::I18nKey::TokenExpire))
            }
            tracing::error!("e:{},token:{}", e, token);
            anyhow::bail!(Errorx::new(local, crate::i18n::I18nKey::TokenInvalid))
        }
    }
}

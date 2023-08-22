use rusty_ulid::Ulid;

pub fn generate_ulid() -> String {
    Ulid::generate().to_string()
}

pub fn bcrypt_hash(value: String) -> anyhow::Result<String> {
    match bcrypt::hash(value.clone(), bcrypt::DEFAULT_COST) {
        Ok(r) => Ok(r),
        Err(e) => {
            tracing::error!("e:{},value:{}", e, value);
            anyhow::bail!(e)
        }
    }
}

pub fn bcrypt_verify(value: String, hash_pwd: String) -> anyhow::Result<bool> {
    match bcrypt::verify(value.clone(), &hash_pwd) {
        Ok(r) => Ok(r),
        Err(e) => {
            tracing::error!("e:{},value:{}", e, value);
            anyhow::bail!(e)
        }
    }
}

use crate::types::User;
use eyre::Result;
// jwt
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn authenticate_user(_user: &User) -> bool {
    true
}

pub fn create_token(user: &User) -> Result<String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", &user.name);
    Ok(claims.sign_with_key(&key)?)
}

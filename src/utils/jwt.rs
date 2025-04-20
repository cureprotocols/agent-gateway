use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

/// JWT claims expected from incoming tokens
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub scopes: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

/// Decodes and validates a JWT token using the shared secret.
///
/// # Arguments
/// * `token` - The JWT string received from the request
/// * `secret` - The JWT secret stored in AppState
///
/// # Returns
/// * `Ok(TokenClaims)` if valid and not expired
/// * `Err(String)` if invalid or expired
pub fn decode_token(token: &str, secret: &str) -> Result<TokenClaims, String> {
    let key = DecodingKey::from_secret(secret.as_bytes());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    match decode::<TokenClaims>(token, &key, &validation) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(format!("Token validation failed: {}", e)),
    }
}

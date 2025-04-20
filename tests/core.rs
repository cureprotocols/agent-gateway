use agent_token_gateway::utils::{jwt, scopes};
use agent_token_gateway::utils::jwt::TokenClaims;
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_mock_claims(scopes: Vec<&str>, offset_seconds: i64) -> TokenClaims {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    TokenClaims {
        sub: "agent-123".into(),
        scopes: scopes.into_iter().map(|s| s.to_string()).collect(),
        iat: (now - 10) as usize,
        exp: (now + offset_seconds) as usize,
    }
}

#[test]
fn test_scope_enforcement_allows_valid_action() {
    let scopes = vec!["agent:read".to_string(), "agent:write".to_string()];
    let result = scopes::enforce_scope(&scopes, "agent:write");
    assert!(result.is_ok());
}

#[test]
fn test_scope_enforcement_denies_invalid_action() {
    let scopes = vec!["agent:read".to_string()];
    let result = scopes::enforce_scope(&scopes, "agent:write");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Missing required scope: agent:write");
}

#[test]
fn test_valid_token_claims_expiration() {
    let claims = generate_mock_claims(vec!["agent:read"], 60); // expires in 60s
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    assert!(claims.exp > now);
}

#[test]
fn test_expired_token_claims() {
    let claims = generate_mock_claims(vec!["agent:read"], -60); // expired 60s ago
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    assert!(claims.exp < now);
}

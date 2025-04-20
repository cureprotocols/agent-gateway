/// Validates whether the requested action exists within the allowed scopes.
///
/// # Arguments
/// * `scopes` - A list of scopes from the JWT (e.g. `["agent:read", "agent:write"]`)
/// * `action` - The specific action the agent is trying to perform
///
/// # Returns
/// * `Ok(())` if allowed
/// * `Err(String)` with a reason if denied
pub fn enforce_scope(scopes: &Vec<String>, action: &str) -> Result<(), String> {
    if scopes.contains(&action.to_string()) {
        Ok(())
    } else {
        Err(format!("Missing required scope: {}", action))
    }
}

// validate user username
// - check for illegal char
use regex::Regex;


/// Validates username with hard-coded regex
/// - Minimum length of 4 chars
/// - Must only include a-z, A-Z or ., _, -
pub fn is_valid_username(username: &String) -> (bool, &str) {
    // construct regex
    let raw_regex = Regex::new(r"^[a-zA-Z._-]{4,}");
    let regex = match raw_regex {
        Ok(o) => o,
        Err(_) => return (false, "Failed to construct regex")
    };
    
    if !regex.is_match(username) {
        return (false, "Username does not match regex")
    }
    
    (true, "")
}
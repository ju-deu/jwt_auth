// validate user's password
// - check for legal chars
// - check for sql injections


/// Validates Password with hard-coded regex:
/// - Minimum length of 8 chars
/// - Must include at lease 1 of [@$!%*?&~#+/_]
/// - Must be 1 uppercase and 1 lowercase letter
/// - Must be at lease 1 number
pub fn is_valid_password(password: &String) -> (bool, &str) {
    // construct regex
    let raw_regex = regex::Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[@$!%*?&~#+/_])[A-Za-z\d@$!%*?/&~#+_]{8,}$");
    let regex = match raw_regex {
        Ok(o) => o,
        _ => return (false, "Failed to construct regex")
    };
    
    if !regex.is_match(password) {
        return (false, "Password does not match regex")    
    };
    
    (true, "")
}
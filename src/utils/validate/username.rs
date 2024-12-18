use std::error::Error;
// validate user username
// - check for illegal char
use regex::Regex;


/// Validates username: 
/// - Between 4 and 20 chars
/// - Must only include a-z, A-Z or ., _, -
pub fn is_valid_username(username: &String) -> Result<(bool, &str), Box<dyn Error>> {
    // check username length
    if username.len() < 4 || username.len() > 20 {
        return Ok((false, "Length not in bounds: (4..=20)"))
    }
    
    // use regex to search for non a-z, A-z, ., _, - chars
    let allowed_chars = Regex::new(r"^[A-Za-z\d._-]+$")?.is_match(username);
    if !allowed_chars {
        return Ok((false, "Includes illegal chars"))
    }
    
    Ok((true, ""))
}
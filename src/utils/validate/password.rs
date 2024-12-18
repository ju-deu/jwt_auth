// validate user's password
// - check for legal chars
// - check for sql injections

use std::error::Error;
use regex::Regex;

/// Validates Password: (usage of multiple Regular Expressions as the regex crate doesn't support look around)
/// - Between 8 and 30 chars
/// - Must include at least 1 of [@$!%*?&~#+/_]
/// - Must include at least 1 uppercase and 1 lowercase letter
/// - Must include at least 1 number/digit
pub fn is_valid_password(password: &String) -> Result<(bool, &str), Box< dyn Error>> {
    // check for password length
    if password.len() < 8 || password.len() > 30 {
        return Ok((false, "Length not in bounds: (8..=30)"))
    }
    
    // match regex
    let allowed_chars = Regex::new(r"^[A-Za-z\d@$!%*?&~#+/_]+$")?.is_match(password);
    let lowercase = Regex::new(r"[a-z]")?.is_match(password);
    let uppercase = Regex::new(r"[A-Z]")?.is_match(password);
    let digit = Regex::new(r"\d")?.is_match(password);
    let special_chars = Regex::new(r"[@$!%*?&~#+/_]")?.is_match(password);
    
    // check cases
    if !allowed_chars {
        return Ok((false, "Includes illegal chars"))
    }
    if !lowercase {
        return Ok((false, "Does not include a-z letter(s)"))
    }
    if !uppercase {
        return Ok((false, "Does not include A-Z letter(s)"))
    }
    if !digit {
        return Ok((false, "Does not include digit 0-9"))
    }
    if !special_chars {
        return Ok((false, "Does not include special char [@$!%*?&~#+/_]"))
    }
    
    Ok((true, ""))
}
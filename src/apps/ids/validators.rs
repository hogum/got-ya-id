//! Custom input validators
use regex::Regex;
use validator::ValidationError;

/// Validates location names
/// - Ensures the name input is composed of only alphaneumeric characters
pub fn validate_location_name(name: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref NAME_PATTERN: Regex = Regex::new(r"^[a-zA-Z0-9 -`_]+$").unwrap();
    }
    if !NAME_PATTERN.is_match(name) {
        return Err(ValidationError::new(
            "Invalid characters in name. Name should contain Alphanumerics or the characters: -_",
        ));
    }
    Ok(())
}

/// Static validation regex instances
pub mod regexes {
    use super::Regex;
    lazy_static! {
        pub static ref ALPHA_REGEX: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
    }
}

/// Validates alphabetic regex
/// - Ensures the name input is composed of only alphaneumeric characters
pub fn validate_alpha_regex(name: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
    }
    if !PATTERN.is_match(name) {
        return Err(ValidationError::new(
            "Invalid characters in name. Name should only contain letters",
        ));
    }
    Ok(())
}

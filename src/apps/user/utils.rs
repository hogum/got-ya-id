use lazy_static;
use regex::Regex;
use tera::{self, Context, Tera};
use validator::ValidationError;

use super::models::{NewUser, User};

use crate::apps::core::response;

lazy_static! {
    /// Lazily Compiled Templates
    pub static ref TEMPLATE: Tera = {
        let mut tera = tera::compile_templates!("src/templates/**/*");
        tera.autoescape_on(vec![".sql"]);
        tera
    };
}
/// Validates name
/// - Ensures the name input is composed of alphabet characters
///  only
///
///  # Returns
///
///  ## ValidationError
/// If the validation fails
pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref NAME_PATTERN: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
    }
    if !NAME_PATTERN.is_match(name) {
        return Err(ValidationError::new("Name should only contain letters"));
    }
    Ok(())
}

/// Validates Email
/// - Ensures the email input follows a valid email
/// address format
///
///
///  # Returns
///
///  ## ValidationError
/// If the validation fails
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref EMAIL_PATTERN: Regex =
            Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
    }
    if !EMAIL_PATTERN.is_match(email) {
        return Err(ValidationError::new("Oops! Email format not invented"));
    }
    Ok(())
}
/// Validates Passwords
/// - Ensures the password inputs match a required regex pattern
///
///
///  # Returns
///
///  ## ValidationError
/// If the validation fails
pub fn validate_pass(pass: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref PASSWORD: Regex = Regex::new(r"^.{6,25}$").unwrap();
    }
    if !PASSWORD.is_match(pass) {
        return Err(ValidationError::new(
            "Password should contain:\n At least 6 characters",
        ));
    }
    Ok(())
}

/// Returns the context holding the template variables
///
/// # Returns
/// - tera::Context
pub fn get_context<'a>(data: &NewUser, path: &'a str) -> Context {
    let mut context = Context::new();

    context.add("username", &data.username);
    context.add("link", &path);
    context
}

/// Template holding context for password reset
/// Receives a User ref
pub fn get_reset_context<'a>(data: &User, path: &'a str) -> Context {
    let mut context = Context::new();

    context.add("username", &data.username);
    context.add("link", &path);
    context
}

/// NaiveDateTime Serialize Deserialize implementation
pub mod naive_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.%f %:z";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Creates a Err Json Response
/// using the given arguments
///
pub fn err_response<T>(status: String, msg: T) -> response::JsonErrResponse<T> {
    response::JsonErrResponse::new(status, msg)
}

/// Builds a complete URI from the arguments given
///
/// # Arguments
/// ## host: str
///     - The host part of the URL
///
/// ## path: str
///     - Path of the request
///
/// ## id: str
///     - Parameter to append to complete the url path
pub fn get_url<'a>(host: &'a str, path: &'a str, id: &'a str) -> String {
    //
    format!(
        r#"https://{host}/{path}/{id}"#,
        host = host,
        path = path,
        id = id
    )
    .replace("\"", "")
    // HeaderValue can't be formatted to str
}

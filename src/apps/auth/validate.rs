//! User authentication

use std::{env, error, process};

use crate::apps::user::models::NewUser;
use crate::config::config;

use chrono::{prelude::*, Duration};
use jsonwebtoken as jwt;
use jwt::{decode, encode, Header, Validation};
use serde_derive::{Deserialize, Serialize};

/// JWT Auth Identity
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

/// Encodes a JWT token with user details {email, username}
pub fn encode_jwt_token(user: NewUser) -> Result<String, Box<dyn error::Error>> {
    let payload = Claims {
        company: user.email.into_owned(),
        sub: "REG".to_owned(),
        exp: (Utc::now() + Duration::hours(36)).timestamp() as usize,
    };

    // ENV Configuration
    let conf = config::get_env_config().unwrap_or_else(|err| {
        eprintln!("Error: Missing required ENV Variable\n{:#?}", err);
        process::exit(78);
    });
    let key = &conf.secret_key;

    let mut header = Header::default();
    header.kid = Some("secretssec".to_owned());

    match encode(&header, &payload, key.as_ref()) {
        Ok(t) => Ok(t),
        Err(e) => Result::Err(Box::new(e)),
    }
}

/// Decodes an encoded authorization token
///
/// # Returns
/// ---------
/// Token Claims
///
/// struct Claims {
///    pub sub: String,
///    pub company: String,
///    pub exp: usize,
/// }
///
/// # Panics
/// - If the token decoding fails
///
pub fn decode_auth_token(token: &String) -> Result<Claims, Box<dyn error::Error>> {
    let key = env::var("secret_key").unwrap_or_else(|_er| {
        eprintln!("Error: Missing required ENV Variable `secret_key`\n");
        process::exit(0);
    });

    let decoded_token = match decode::<Claims>(&token, key.as_ref(), &Validation::default()) {
        Ok(c) => c,
        Err(e) => return Result::Err(Box::new(e)),
    };
    Ok(decoded_token.claims)
}

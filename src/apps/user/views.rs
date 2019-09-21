//! Handles views for User items
//!

use crate::apps::user::models::User;
use actix_web::{http, web, HttpResponse};
use validator::Validate;

/// Registers a new user
///
/// # method
///
///
/// # Returns
/// JSON of received User data
pub fn register_user(data: web::Json<User>) -> HttpResponse {
    println!("Data: {:#?}", data);
    if let Err(err) = data.validate() {
        return HttpResponse::build(http::StatusCode::BAD_REQUEST).json(err);
        // Filter json where message is not null
    };
    HttpResponse::build(http::StatusCode::CREATED).json(data.0)
}

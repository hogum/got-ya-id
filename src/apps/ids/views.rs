use actix_web::{Error, HttpResponse, Result, web, http::StatusCode};
use actix_web::error::ErrorInternalServerError;

use super::models::{ NewIdentification};
use crate::{hashmap, core::response::{err, respond}};

use validator::Validate;

/// Receives a json NewIdentification data struct which is
/// used to POST a new Identification
///
/// # url
/// ``
/// # method
/// `POST`
pub async fn create_new_identification(new_idt: web::Json<NewIdentification<'_>>) -> Result<HttpResponse, Error> {
   if let Err(e) = new_idt.0.validate() {
       return Ok(err("400", e.to_string()));
   }
   new_idt
       .save()
       .map_err(|e| ErrorInternalServerError(e))
       .map( move|idt| {
            let res = hashmap!["status" => "201",
            "message" => "Success. Indentification created"];
            respond(res, Some(idt), None).unwrap()
       })

    }
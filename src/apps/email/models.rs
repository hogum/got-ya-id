use diesel::{self, prelude::*};

use validator::Validate;
use validator_derive::Validate;

use serde::{Deserialize, Serialize};

use crate::{
    apps::user::{models::User, utils::from_timestamp},
    diesel_cfg::schema::emails,
};

use chrono::NaiveDateTime;

use std::borrow::Cow;

#[derive(Queryable, Associations, Serialize, Deserialize, AsChangeset, Identifiable)]
#[belongs_to(User)]
//#[table_name = "emails"]
/// Represents the Queryable Email data
pub struct Email {
    id: i32,
    user_id: i32,
    pub email: String,
    /// Selected for default use in identifying the user
    active: bool,
    /// No longer associated. Deleted.
    removed: bool,

    #[serde(deserialize_with = "from_timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(deserialize_with = "from_timestamp")]
    updated_at: NaiveDateTime,
}

/// Holds new email data
#[derive(Insertable, Deserialize, Validate)]
#[table_name = "emails"]
#[serde(deny_unknown_fields)]
pub struct NewEmail<'a> {
    #[serde(skip_deserializing)]
    pub user_id: i32,

    #[validate(email(message = "Email format not invented yet"))]
    pub email: Cow<'a, str>,
    #[serde(skip_deserializing)]
    pub active: bool,

    #[serde(skip_deserializing)]
    pub removed: bool,
}
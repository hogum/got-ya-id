//! Models for User Profiles

use crate::apps::user::models::User;
use crate::diesel::RunQueryDsl;
use crate::diesel_cfg::{config::connect_to_db, schema, schema::profiles};

use diesel::{self, prelude::*};
use serde::{Deserialize, Serialize};

use std::{borrow::Cow, error};

/// Holds the User Profile Record
#[derive(Queryable, Identifiable, Associations, Deserialize, Default, Serialize, Debug)]
#[belongs_to(User)]
pub struct Profile<'a> {
    id: i32,
    user_id: i32,
    phone: Option<String>,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    institution: Option<String>,
    avatar: Option<String>,
    found_ids: Option<Cow<'a, i32>>,
}

impl<'a> Profile<'a> {
    /// Finds a given profile by its Primary Key
    pub fn find_by_key(pk: i32) -> Result<Vec<Profile<'a>>, Box<dyn error::Error>> {
        use schema::profiles::dsl::*;
        let query = profiles.find(pk).load(&connect_to_db())?;
        if query.is_empty() {
            return Err(format!("User of ID {} non-existent", pk).into());
        }
        Ok(query)
    }
}

/// Holds a new User Profile Record
#[derive(Insertable, Deserialize, Default, Serialize, Debug)]
#[table_name = "profiles"]
#[serde(deny_unknown_fields)]
pub struct NewProfile<'a> {
    user_id: i32,
    institution: Option<Cow<'a, str>>,
}

impl<'a> NewProfile<'a> {
    /// Creates a new Profile associated with the user of the given ID.
    ///
    /// # Arguments
    /// - user_id: u32
    ///     ID of the user to associate with this profile
    /// - profile: Option<u32>
    ///     If Some returns the created user Profile object.
    ///     None(default): Nothing is returned
    pub fn new<'b>(user_id: i32, profile: Option<u32>) -> Result<Option<Profile<'b>>, String> {
        let new_profile = NewProfile {
            user_id,
            ..Default::default()
        };
        let res = diesel::insert_into(profiles::table)
            .values(new_profile)
            .get_result::<Profile>(&connect_to_db())
            .expect("Error creating user profile");

        if profile.is_some() {
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}
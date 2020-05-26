table! {
    /// Representation of the `avatars` table.
    ///
    /// (Automatically generated by Diesel.)
    avatars (id) {
        /// The `id` column of the `avatars` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `avatars` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `url` column of the `avatars` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        url -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `claimed_identifications` table.
    ///
    /// (Automatically generated by Diesel.)
    claimed_identifications (id) {
        /// The `id` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `name` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `course` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        course -> Varchar,
        /// The `entry_year` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        entry_year -> Nullable<Date>,
        /// The `graduation_year` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        graduation_year -> Nullable<Date>,
        /// The `institution` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        institution -> Varchar,
        /// The `campus_location` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        campus_location -> Varchar,
        /// The `created_at` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `claimed_identifications` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
    /// Representation of the `email` table.
    ///
    /// (Automatically generated by Diesel.)
    email (id) {
        /// The `id` column of the `email` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `email` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `email` column of the `email` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `active` column of the `email` table.
        ///
        /// Its SQL type is `Nullable<Bool>`.
        ///
        /// (Automatically generated by Diesel.)
        active -> Nullable<Bool>,
        /// The `removed` column of the `email` table.
        ///
        /// Its SQL type is `Nullable<Bool>`.
        ///
        /// (Automatically generated by Diesel.)
        removed -> Nullable<Bool>,
        /// The `created_at` column of the `email` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `email` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
    /// Representation of the `emails` table.
    ///
    /// (Automatically generated by Diesel.)
    emails (id) {
        /// The `id` column of the `emails` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `emails` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `email` column of the `emails` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `active` column of the `emails` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        active -> Bool,
        /// The `removed` column of the `emails` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        removed -> Bool,
        /// The `created_at` column of the `emails` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `emails` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
        use diesel_geometry::sql_types::Point;
        use diesel::sql_types::*;
    /// Representation of the `identifications` table.
    ///
    /// (Automatically generated by Diesel.)
    identifications (id) {
        /// The `id` column of the `identifications` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `course` column of the `identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        course -> Varchar,
        /// The `valid_from` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        valid_from -> Nullable<Date>,
        /// The `valid_till` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Date>`.
        ///
        /// (Automatically generated by Diesel.)
        valid_till -> Nullable<Date>,
        /// The `institution` column of the `identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        institution -> Varchar,
        /// The `campus` column of the `identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        campus -> Varchar,
        /// The `location_name` column of the `identifications` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        location_name -> Varchar,
        /// The `location_point` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Point>`.
        ///
        /// (Automatically generated by Diesel.)
        location_point -> Nullable<Point>,
        /// The `picture` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        picture -> Nullable<Varchar>,
        /// The `posted_by` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        posted_by -> Nullable<Int4>,
        /// The `is_found` column of the `identifications` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_found -> Bool,
        /// The `created_at` column of the `identifications` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `identifications` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `about` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        about -> Nullable<Varchar>,
        /// The `owner` column of the `identifications` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        owner -> Nullable<Int4>,
    }
}

table! {
    /// Representation of the `matched_identifications` table.
    ///
    /// (Automatically generated by Diesel.)
    matched_identifications (id) {
        /// The `id` column of the `matched_identifications` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `claim_id` column of the `matched_identifications` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        claim_id -> Int4,
        /// The `identification_id` column of the `matched_identifications` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        identification_id -> Int4,
        /// The `created_at` column of the `matched_identifications` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    /// Representation of the `oath_users` table.
    ///
    /// (Automatically generated by Diesel.)
    oath_users (id) {
        /// The `id` column of the `oath_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `email` column of the `oath_users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `name` column of the `oath_users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `first_name` column of the `oath_users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        first_name -> Nullable<Varchar>,
        /// The `family_name` column of the `oath_users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        family_name -> Nullable<Varchar>,
        /// The `is_verified` column of the `oath_users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_verified -> Bool,
        /// The `picture` column of the `oath_users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        picture -> Nullable<Varchar>,
        /// The `locale` column of the `oath_users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        locale -> Nullable<Varchar>,
        /// The `acc_id` column of the `oath_users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        acc_id -> Varchar,
        /// The `is_active` column of the `oath_users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_active -> Bool,
        /// The `provider` column of the `oath_users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        provider -> Varchar,
        /// The `provider_verified` column of the `oath_users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        provider_verified -> Bool,
        /// The `created_at` column of the `oath_users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `oath_users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
    }
}

table! {
    /// Representation of the `profiles` table.
    ///
    /// (Automatically generated by Diesel.)
    profiles (id) {
        /// The `id` column of the `profiles` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `profiles` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `phone` column of the `profiles` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        phone -> Nullable<Varchar>,
        /// The `name` column of the `profiles` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Nullable<Varchar>,
        /// The `institution` column of the `profiles` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        institution -> Nullable<Varchar>,
        /// The `about` column of the `profiles` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        about -> Nullable<Text>,
        /// The `found_ids` column of the `profiles` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        found_ids -> Nullable<Int4>,
    }
}

table! {
    /// Representation of the `refresh_tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    refresh_tokens (id) {
        /// The `id` column of the `refresh_tokens` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int8,
        /// The `body` column of the `refresh_tokens` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        body -> Varchar,
        /// The `valid` column of the `refresh_tokens` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        valid -> Bool,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `username` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        username -> Varchar,
        /// The `password` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        password -> Nullable<Varchar>,
        /// The `created_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `updated_at` column of the `users` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `is_active` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_active -> Bool,
        /// The `is_verified` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        is_verified -> Bool,
        /// The `social_id` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        social_id -> Nullable<Varchar>,
        /// The `social_account_verified` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        social_account_verified -> Bool,
        /// The `access_level` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        access_level -> Int4,
    }
}

joinable!(avatars -> users (user_id));
joinable!(claimed_identifications -> users (user_id));
joinable!(email -> users (user_id));
joinable!(emails -> users (user_id));
joinable!(identifications -> users (owner));
joinable!(matched_identifications -> claimed_identifications (claim_id));
joinable!(matched_identifications -> identifications (identification_id));
joinable!(profiles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    avatars,
    claimed_identifications,
    email,
    emails,
    identifications,
    matched_identifications,
    oath_users,
    profiles,
    refresh_tokens,
    users,
);

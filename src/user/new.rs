// - generate a new user (NewUser)
// - add to bucket of new users
// - write model to db
// - send user with verification code

use crate::user::model::{Code, NewUser};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::{Arc};
use uuid::Uuid;
use crate::utils::appstate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Body {
    username: String,
    email: String,
    password: String,
}


pub async fn new(
    State(appstate): State<Arc<AppState>>,
    Json(payload): Json<Body>               // json payload
) -> (StatusCode, &'static str) {
    // salt and hash password
    let password_salt = SaltString::generate(&mut OsRng);
    let hashed_password = match Argon2::default().hash_password(payload.password.as_ref(), &password_salt) {
        Ok(o) => o.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password")
    };

    // generate user
    let uuid = Uuid::new_v4();
    let code = Code::new(&uuid);

    let new_user = NewUser::from(
        uuid,
        payload.username,
        payload.email,
        hashed_password,
        code.code
    );

    println!("{:?}", new_user);

    // add user to UserBucket
    let mut user_bucket = match appstate.user_bucket.lock() {
        Ok(o) => o,
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get UserBucket")
    };

    // error handling might unnecessary here as it only returns an error if the uuid is already in the bucket
    match user_bucket.insert(&uuid, &code) {
        Some(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert into bucket"),
        _ => {}
    }
    
    
    // write model to database
    

    (StatusCode::OK, "")
}
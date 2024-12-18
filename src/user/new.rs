// - generate a new user (NewUser)
// - add to bucket of new users
// - write user to database
// - send user with verification code


use crate::{
    user::model::{Code, NewUser},
    utils::{
        appstate::AppState,
        validate::{
            password::is_valid_password,
            username::is_valid_username
        }
    }
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    Json
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Body {
    username: String,
    email: String,
    password: String,
}

#[axum::debug_handler]
pub async fn new(
    State(appstate): State<Arc<AppState>>,
    Json(payload): Json<Body>
) -> (StatusCode, String) {
    // validate username and password
    match is_valid_username(&payload.username) {
        Ok(o) => { 
            match o {
                (true, _) => {},
                (false, message) => return (StatusCode::BAD_REQUEST, "Username is not valid: ".to_string() + message)
            }
        }
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to validate Password".to_string())
        }
    }

    match is_valid_password(&payload.password) {
        Ok(o) => {
            match o {
                (true, _) => {}
                (false, message) => return (StatusCode::BAD_REQUEST, "Password is not valid: ".to_string() + message)
            }
        }
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to validate Password".to_string())
        }
    }

    // salt and hash password
    let password_salt = SaltString::generate(&mut OsRng);
    let hashed_password = match Argon2::default().hash_password(payload.password.as_ref(), &password_salt) {
        Ok(o) => o.to_string(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password".to_string())
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


    // add user to UserBucket
    let user_bucket = &appstate.user_bucket;
    let mut user_bucket = user_bucket.lock().await;

    // error handling might unnecessary here as it only returns an error if the uuid is already in the bucket which is unlikely
    if let Some(_) = user_bucket.insert(&uuid, &code) {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert into bucket".to_string())
    }
    
    
    // write model to database
    // INSERT INTO new_users VALUES( ... )
    let conn = &appstate.db_pool;
    let raw_query = r"INSERT INTO new_users VALUES ( $1, $2, $3, $4, $5)";

    let query = sqlx::query(raw_query)
        .bind(&new_user.uuid.to_string())
        .bind(&new_user.username)
        .bind(&new_user.email)
        .bind(&new_user.password)
        .bind(new_user.code as i64)
        .execute(conn.as_ref()).await;



    match query {
        Ok(_) => {},
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write to database".to_string())
        }
    };

    (StatusCode::CREATED, "User created successfully".to_string())
}
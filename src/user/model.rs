use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Permission { User, Admin }


#[derive(Serialize, Deserialize)]
pub struct NewUser {
    uuid: Uuid,
    username: String,
    email: String,
    password: String,
    code: String,
}


#[derive(Serialize, Deserialize)]
pub struct User {
    uuid: Uuid,
    username: String,
    email: String,
    password: String,

    permission: Permission,
    tokenversion: u64,
    
    issued_at: u64
}


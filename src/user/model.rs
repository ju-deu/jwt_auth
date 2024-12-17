use chrono::TimeZone;
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Code {
    code: u16,
    sub: Uuid,
    iat: u64,
    exp: u64,
}

impl Code {
    pub fn is_expired(&self) -> bool {
        let current_time = chrono::Utc::now().timestamp() as u64;
        if current_time >= self.exp {
            return true
        }
        false
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

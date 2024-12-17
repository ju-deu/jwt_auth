use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Permission { User, Admin }


#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    uuid: Uuid,
    username: String,
    email: String,
    password: String,
    code: u32,
}
impl NewUser {
    pub fn from(uuid: Uuid, username: String, email: String, password: String, code: u32) -> Self {
        Self {
            uuid, username, email, password, code,
        }
    }
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Code {
    pub(crate) code: u32,
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
    pub fn new(uuid: &Uuid) -> Self {
        let code = rand::thread_rng().gen_range(100000..=999999);
        let current_time = Utc::now().timestamp() as u64;
        
        Self {
            code,
            sub: uuid.to_owned(),
            iat: current_time,
            exp: current_time + (60*15)
        }
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

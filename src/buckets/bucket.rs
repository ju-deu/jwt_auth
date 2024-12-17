use crate::user::model::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct PasswordBucket(HashMap<Uuid, Code>);
pub struct UsersBucket(HashMap<Uuid, Code>);



impl PasswordBucket {
    pub fn insert(&mut self, uuid: Uuid, code: Code) {
        self.0.insert(uuid, code);
    }

    pub fn compare(&mut self, uuid: Uuid, other_code: Code) -> bool {
        let code = match self.0.get(&uuid) {
            Some(x) => x,
            _ => { return false }
        };
        
        code.to_owned() == other_code
    }

    pub fn remove(&mut self, uuid: Uuid) -> Option<Code> {
        self.0.remove(&uuid)
    }
}

impl UsersBucket {
    pub fn insert(&mut self, uuid: Uuid, code: Code) {
        self.0.insert(uuid, code);
    }

    pub fn compare(&mut self, uuid: Uuid, other_code: Code) -> bool {
        let code = match self.0.get(&uuid) {
            Some(x) => x,
            _ => { return false }
        };

        code.to_owned() == other_code
    }

    pub fn remove(&mut self, uuid: Uuid) -> Option<Code> {
        self.0.remove(&uuid)
    }
}
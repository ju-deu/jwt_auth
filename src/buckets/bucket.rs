use crate::user::model::*;
use std::collections::HashMap;
use uuid::Uuid;


#[derive(Debug)]
pub struct PasswordBucket(HashMap<Uuid, Code>);

#[derive(Debug)]
pub struct UsersBucket(HashMap<Uuid, Code>);



impl PasswordBucket {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, uuid: &Uuid, code: &Code) {
        self.0.insert(uuid.clone(), code.clone());
    }

    pub fn compare(&mut self, uuid: &Uuid, other_code: Code) -> bool {
        let code = match self.0.get(&uuid) {
            Some(x) => x,
            _ => { return false }
        };

        code.to_owned() == other_code
    }

    pub fn remove(&mut self, uuid: &Uuid) -> Option<Code> {
        self.0.remove(uuid)
    }
}

impl UsersBucket {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, uuid: &Uuid, code: &Code) -> Option<Code> {
        self.0.insert(uuid.clone(), code.clone())
    }

    pub fn compare(&mut self, uuid: &Uuid, other_code: Code) -> bool {
        let code = match self.0.get(&uuid) {
            Some(x) => x,
            _ => { return false }
        };

        code.to_owned() == other_code
    }

    pub fn remove(&mut self, uuid: &Uuid) -> Option<Code> {
        self.0.remove(uuid)
    }
}
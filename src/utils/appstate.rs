use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use crate::buckets::bucket::{PasswordBucket, UsersBucket};

pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
    pub user_bucket: Arc<Mutex<UsersBucket>>,
    pub password_bucket: Arc<Mutex<PasswordBucket>>
}
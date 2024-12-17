use axum::routing::{get, post};
use axum::Router;
use sqlx::postgres::PgPool;
use std::sync::{Arc, Mutex};
use axum::handler::Handler;
use jwt_authentication_lib::{buckets::bucket::*, user, utils::appstate::AppState};

#[tokio::main]
async fn main() {
    // load env variables
    dotenv::dotenv().ok();
    
    // create postgres connection pool
    let psql_url = std::env::var("PSQL_URL").expect("Failed to load environment variable: PSQL_URL");
    let pool = PgPool::connect(&psql_url).await.expect("Failed to create PgPool");
    
    let shared_pool = Arc::new(pool);

    // create buckets
    let password_bucket = Arc::new(Mutex::new(PasswordBucket::new()));
    let user_bucket = Arc::new(Mutex::new(UsersBucket::new()));

    let AppState = AppState {
        db_pool: shared_pool,
        user_bucket,
        password_bucket,
    };
    
    
    // make app
    let app = Router::new()
        .route("/ping", get(|| async { "Hello, World!" }))
        //.route("/test", get(test)).with_state(shared_pool.clone())
        //.route("/test", post(user::new::new)).with_state()
    ;

    // serve on 0.0.0.0:8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


/*
async fn test(
    State(pool): State<Arc<Pool<Postgres>>>
) -> String {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&*pool)
        .await
        .unwrap();

    row.0.to_string()
}*/
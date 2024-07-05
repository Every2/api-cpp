use std::env;
use axum::{routing::post, Router};
use real_time_app::create_user;
use sqlx::{migrate::MigrateDatabase, Sqlite};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("create .env file in root directory");
    let url = env::var("DATABASE_URL").expect("ENV VARIABLE DOESN'T EXIST");
    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        println!("Creating Database...");
        match Sqlite::create_database(&url).await {
            Ok(_) => println!("Created db!"),
            Err(error) => panic!("error {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let app = Router::new()
        .route("/auth/register", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();    
}


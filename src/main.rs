use std::env;
use axum::{routing::{get, post}, Router};
use real_time_app::{create_user, login, register};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("create .env file in root directory");
    let url = env::var("DATABASE_URL").expect("ENV VARIABLE DOESN'T EXIST");
    if !Sqlite::database_exists(&url).await.unwrap_or(false) {
        println!("Creating Database...");
        match Sqlite::create_database(&url).await {
            Ok(_) => println!("Database created successfully!!"),
            Err(error) => panic!("error {}", error),
        }
    }
    let db_connection = SqlitePool::connect(&url).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER
        PRIMARY KEY NOT NULL, username VARCHAR(250) NOT NULL,
        email VARCHAR(250) NOT NULL, password VARCHAR(250) NOT NULL);")
        .execute(&db_connection).await.unwrap();
    let app = Router::new()
        .route("/register", get(register))
        .route("/login", get(login))
        .route("/auth/register", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running http://localhost:3000/");
    axum::serve(listener, app).await.unwrap();     
}


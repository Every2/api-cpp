use std::env;
use axum::{http::StatusCode, response::Html, Json};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;


#[derive(Clone, Serialize)]
pub struct User {
    username: String,
    email: String,
    #[serde(skip_serializing)]
    password: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    email: String,
    password: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>,) -> (StatusCode, Json<User>) {
    let user = User {
        username: payload.username.to_string(),
        email: payload.email.to_string(),
        password: payload.password.to_string(),
    };

    let json_response = user.clone();
 
    dotenvy::dotenv().expect("create .env file in root directory");
    let url = env::var("DATABASE_URL").expect("ENV VARIABLE DOESN'T EXIST");
    let db_connection = SqlitePool::connect(&url).await.unwrap();
    sqlx::query("INSERT INTO users (username, email, password) VALUES($1, $2, $3)")
    .bind(user.username)
    .bind(user.email)
    .bind(user.password)
    .execute(&db_connection)
    .await
    .unwrap();
    (StatusCode::CREATED, Json(json_response))
}

pub async fn register() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="pt-br">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Registre-se!</title>
    </head>
    <body>
        <h2>Formulário de Cadastro</h2>
        <form action="/submit-form" method="post">
            <label for="nome">Nome:</label>
            <input type="text" id="nome" name="nome" required><br><br>
    
            <label for="email">Email:</label>
            <input type="email" id="email" name="email" required><br><br>
    
            <label for="senha">Senha:</label>
            <input type="password" id="senha" name="senha" required><br><br>
    
            <button type="submit">Enviar</button>
        </form>
    </body>
    </html>
    "#)
}
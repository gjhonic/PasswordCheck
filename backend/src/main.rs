use axum::{
    extract::Form,
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::get,
    routing::post,
    response::IntoResponse,
    Router,
};
mod validators;
use validators::*;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    password: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ResultCheck {
    status: bool,
    message: String,
}

use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/api/v1/check_password", post(check_password));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> &'static str {
    "Home page"
}

/**
 * Метод принимает пароль и возвращает результат проверки
 */
async fn check_password(Form(input): Form<Input>) -> impl IntoResponse {
    let max_level :i8 = 4;
    let result: ResultCheck = validate_password(input.password, max_level);

    let json = serde_json::to_string(&result).unwrap();
    (
        StatusCode::OK,
        json
    ) 
}

/**
 * Метод запускает валидацию пароля
 */
fn validate_password(password: String, level: i8) -> ResultCheck {
    let mut result = ResultCheck {
        status: true,
        message: String::from("Начало валидации"),
    };
    let mut number_level : i8 = 1;

    while number_level <= level
    {
        match number_level {
            1 => {
                result = validate_level1(password.clone());
            },
            2 =>{
                result = validate_level2(password.clone());
            },
            3 => {
                result = validate_level3(password.clone());
            },
            4 => {
                result = validate_level4(password.clone());
            },
            _ => {
                result = validate_level3(password.clone());
            }
        }

        if result.status == false {
            break;
        }
        number_level += 1;
    }

    if result.status == true {
        result.message = String::from("Хороший пароль!");
    }
    return result;   
}


fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
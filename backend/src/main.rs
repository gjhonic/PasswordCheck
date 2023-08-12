use axum::{
    extract::Form,
    http::{StatusCode},
    routing::get,
    routing::post,
    response::IntoResponse,
    Router,
};
//Импортируем валидные проверки
mod validators;
use validators::*;

//Импортируем фейк проверки
mod scamvalidators;
use scamvalidators::*;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    password: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ResultCheck {
    status: i8,
    message: String,
    level: i8,
}

pub const ERROR_VALIDATE_STATUS: i8 = 1; //Ошибка валидации
pub const WEEK_PASSWORD_STATUS: i8 = 2; //Слабый уровень пароля
pub const GOOD_PASSWORD_STATUS: i8 = 3; //Сильный пароль

use std::{net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/api/v1/check_password", post(check_password))
        .route("/api/v1/check_scam_password", post(check_scam_password));

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
    let max_level :i8 = 5;
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
        status: self::ERROR_VALIDATE_STATUS,
        message: String::from("Начало валидации"),
        level: 0,
    };
    let mut number_level : i8 = 0;

    while number_level <= level
    {
        match number_level {
            1 => {
                result = validate_level0(password.clone());
            },
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
            5 => {
                result = validate_level5(password.clone());
            },
            _ => {
                result = validate_level3(password.clone());
            }
        }

        if result.status != self::GOOD_PASSWORD_STATUS {
            break;
        }
        number_level += 1;
    }

    if result.status == self::GOOD_PASSWORD_STATUS {
        result.message = String::from("Хороший пароль!");
    }
    return result;   
}

/**
 * Метод принимает пароль и возвращает результат проверки (Шуточная проверка)
 */
async fn check_scam_password(Form(input): Form<Input>) -> impl IntoResponse {
    let max_level :i8 = 5;
    let mut result: ResultCheck = validate_scam_password(input.password, max_level);

    if result.level == max_level {
        result.message = String::from("Хороший пароль!");
    }

    let json = serde_json::to_string(&result).unwrap();
    (
        StatusCode::OK,
        json
    ) 
}

/**
 * Метод запускает валидацию пароля 
 */
fn validate_scam_password(password: String, level: i8) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::ERROR_VALIDATE_STATUS,
        message: String::from("Начало валидации"),
        level: 0,
    };
    let mut number_level : i8 = 0;

    while number_level <= level
    {
        match number_level {
            0 => {
                result = validate_level0(password.clone());
            },
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
            5 => {
                result = validate_level5(password.clone());
            },
            6 => {
                result = validate_scam_level1(password.clone());
            },
            _ => {
                result = validate_level0(password.clone());
            }
        }

        if result.status != self::GOOD_PASSWORD_STATUS {
            break;
        }
        number_level += 1;
    }

    if result.status == self::GOOD_PASSWORD_STATUS {
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
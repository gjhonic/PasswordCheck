use crate::ResultCheck;
use regex::Regex;

//Пароль проверяет наличие чисел в строке
pub fn validate_level1(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: true,
        message: String::from("Валидация уровня 1 успешно пройдена"),
    };

    let re = Regex::new(r"\d").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = false;
        result.message = String::from("В пароле должны быть цифры");
    }
    
    return result;
}

//Пароль проверяет наличие букв
pub fn validate_level2(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: true,
        message: String::from("Валидация уровня 2 успешно пройдена"),
    };

    let re = Regex::new(r"[a-z]").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = false;
        result.message = String::from("В пароле должны быть буквы");
    }
    
    return result;
}

//Пароль проверяет наличие заглавных букв
pub fn validate_level3(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: true,
        message: String::from("Валидация уровня 3 успешно пройдена"),
    };

    let re = Regex::new(r"[A-Z]").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = false;
        result.message = String::from("В пароле должны быть заглавные буквы");
    }
    
    return result;
}

//Пароль проверяет количество символов в пароле
pub fn validate_level4(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: true,
        message: String::from("Валидация уровня 3 успешно пройдена"),
    };

    let length_password = password.len();
    if (length_password < 10) {
        result.status = false;
        result.message = String::from("Длина пароля должна быть больше 10 символов");
    }
    
    return result;
}
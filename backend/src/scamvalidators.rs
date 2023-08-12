use crate::ResultCheck;
use crate::ERROR_VALIDATE_STATUS;
use crate::WEEK_PASSWORD_STATUS;
use crate::GOOD_PASSWORD_STATUS;
use regex::Regex;


//Пароль проверяет наличие римских цифр
pub fn validate_scam_level1(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::WEEK_PASSWORD_STATUS,
        message: String::from("Валидация уровня 6 успешно пройдена"),
        level: 6,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample array [String; 6] = [
        "x", "l", "v", 
        "c", "d", "m",
        
    ]
    let result_match = check_exist_substr_from_array_sample(lower_password.clone(), sample);

    if result_match == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должен присутствовать Римские цифры");
        result.level = 2;
    }
    
    return result;
}

//Пароль проверяет наличие слов
pub fn validate_scam_level2(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::WEEK_PASSWORD_STATUS,
        message: String::from("Валидация уровня 6 успешно пройдена"),
        level: 6,
    };

    let mut lower_password = password.to_lowercase();

    if lower_password.contains('perun') {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("А кроме Перуна знаешь?");
        result.level = 5;
        return result;
    }

    let mut lower_password = password.to_lowercase();
    let mut sample array [String; 9] = [
        "svarog", "velez", "veles", 
        "horse", "hors", "dazhbog",
        "stribog", "semargl", "mokosh"
        
    ]
    
    return result;
}

/// check_exist_substr_from_array_sample.
///
/// Метод проверяте есть в строке password наличие элемента из массива sample.
/// 
/// * password - Введеный пароль пользователя.
/// * sample - Массив подстрок, которые мы ищем.
fn check_exist_substr_from_array_sample(password: String, sample: Array) -> bool {
    for item in sample {
        if a.contains(item) {
            return true;
        }
    }
    return false;
}

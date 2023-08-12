use crate::ResultCheck;
use crate::ERROR_VALIDATE_STATUS;
use crate::WEEK_PASSWORD_STATUS;
use crate::GOOD_PASSWORD_STATUS;
use regex::Regex;

//Валидация корректности введенного поля
pub fn validate_level0(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 0 успешно пройдена"),
        level: 0,
    };

    let re = Regex::new(r"\s").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == true {
        result.status = self::ERROR_VALIDATE_STATUS;
        result.message = String::from("В пароле не должно быть пробельных символов");
    }

    let length_password = password.len();
    if length_password == 0 {
        result.status = self::ERROR_VALIDATE_STATUS;
        result.message = String::from("Пароль не должен быть пустым");
        result.level = 0;
    }
    
    return result;
}

//Пароль проверяет наличие чисел в строке
pub fn validate_level1(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 1 успешно пройдена"),
        level: 1,
    };

    let re = Regex::new(r"\d").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должны быть цифры");
    }
    
    return result;
}

//Пароль проверяет наличие букв
pub fn validate_level2(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 2 успешно пройдена"),
        level: 2,
    };

    let re = Regex::new(r"[a-z]").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должны быть буквы");
        result.level = 1;
    }
    
    return result;
}

//Пароль проверяет наличие заглавных букв
pub fn validate_level3(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 3 успешно пройдена"),
        level: 3,
    };

    let re = Regex::new(r"[A-Z]").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должны быть заглавные буквы");
        result.level = 2;
    }
    
    return result;
}

//Пароль проверяет количество символов в пароле
pub fn validate_level4(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 4 успешно пройдена"),
        level: 4,
    };

    let length_password = password.len();
    if length_password < 10 {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Длина пароля должна быть больше 10 символов");
        result.level = 3;
    }
    
    return result;
}

//Пароль проверяет наличие спец символов
pub fn validate_level5(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 5 успешно пройдена"),
        level: 5,
    };

    let re = Regex::new(r"[\!@$%-_.,]").unwrap();
    let result_match: bool = re.is_match(&password);

    if result_match == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должны быть спец символы (!,@,$,%,_,.,-)");
        result.level = 4;
    }
    
    return result;
}
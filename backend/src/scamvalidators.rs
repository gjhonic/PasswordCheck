use crate::ResultCheck;
use crate::ERROR_VALIDATE_STATUS;
use crate::WEEK_PASSWORD_STATUS;
use crate::GOOD_PASSWORD_STATUS;
use regex::Regex;

///Пароль проверяет наличие римских цифр
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level1(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 6 успешно пройдена"),
        level: 6,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 6] = [
        String::from("x"),  String::from("l"), String::from("v"),   
        String::from("c"), String::from("d"), String::from("m")
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должен присутствовать Римские цифры");
        result.level = 2;
    }
    
    return result;
}

///Пароль проверяет наличие имени славянского бога
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level2(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 7 успешно пройдена"),
        level: 7,
    };

    let mut lower_password = password.to_lowercase();

    if lower_password.contains("perun") {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("А кроме Перуна знаешь?");
        result.level = 5;
        return result;
    }

    let mut sample: [String; 9] = [
        String::from("svarog"),  String::from("velez"), String::from("veles"),   
        String::from("horse"), String::from("hors"), String::from("dazhbog"),   
        String::from("stribog"), String::from("semargl"), String::from("mokosh")
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должно присутствовать имя славянского бога");
        result.level = 2;
    }
    
    return result;
}

///Пароль проверяет наличие числа 300
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level3(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 8 успешно пройдена"),
        level: 8,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("300")
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("В пароле должно присутствовать число 300");
        result.level = 2;
    }
    
    return result;
}

///Пароль проверяет наличие строки russian
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level4(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 9 успешно пройдена"),
        level: 9,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 2] = [
        String::from("z"), String::from("russian"), 
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Докажи что ты патриот");
        result.level = 2;
    }
    
    return result;
}


///Пароль проверяет ответа на уравнение
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level5(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 10 успешно пройдена"),
        level: 10,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("342"),
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Пароль должен содержать ответ уравнения: 364 - 2(x + 7) = 8 - x");
        result.level = 2;
    }
    
    return result;
}

///Пароль проверяет ответа загадку
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level6(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 11 успешно пройдена"),
        level: 11,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("echo"),
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Пароль должен ответ на загадку: What can’t talk but will reply when spoken to?");
        result.level = 2;
    }
    
    return result;
}

///Пароль проверяет наличие числа
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level7(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 12 успешно пройдена"),
        level: 12,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("17098246"),
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Пароль должен включать площадь России");
        result.level = 2;
    }
    
    return result;
}

///Пароль должен быть не больше 40 символов
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level8(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 13 успешно пройдена"),
        level: 13,
    };

    let length_password = password.len();
    if length_password > 25 {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Пароль должен быть не больше 40 символов");
        result.level = 12;
    }

    return result;
}
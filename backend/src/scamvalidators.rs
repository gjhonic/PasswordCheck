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
        result.level = 5;
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
        result.level = 6;
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
        result.level = 6;
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
        result.level = 7;
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
        result.level = 8;
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
        result.level = 9;
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
        result.level = 10;
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
        result.level = 11;
    }
    
    return result;
}

///Пароль должен быть не больше 25 символов
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level8(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 13 успешно пройдена"),
        level: 13,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("family"),
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        let length_password = password.len();
        if length_password > 25 {
            result.status = self::WEEK_PASSWORD_STATUS;
            result.message = String::from("Пароль должен быть не больше 25 символов");
            result.level = 12;
        }
    }

    return result;
}

///Пароль должен быть число 21
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level9(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 14 успешно пройдена"),
        level: 14,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("21"),
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
        result.message = String::from("Пароль должен включать 9 номер числа фибоначи (начиная с 0)");
        result.level = 13;
    }

    return result;
}

///Пароль должен быть слово 463
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level10(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 15 успешно пройдена"),
        level: 15,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("463"),
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
        result.message = String::from("Пароль должен включать число, которое я загадал: К загаданному числу прибавил 37 потом отнял 400 далее прибавил 21 и наконец взял под корень в итоге получилось 11");
        result.level = 14;
    }

    return result;
}

///Пароль должен быть слово family
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level11(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 16 успешно пройдена"),
        level: 16,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("family"),
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
        result.message = String::from("Пароль должен включать слово из книги: Harry Potter and the Philosopher's Stone Глава: 3 Абзац: 7 Слово№: 10");
        result.level = 15;
    }

    return result;
}

///Пароль должен быть слово lotos
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level12(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 17 успешно пройдена"),
        level: 17,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 2] = [
        String::from("lotus"), String::from("lotos"),
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
        result.message = String::from("Ps Не спрашивай почему твой пароль больше 25 и все ок, но пусть твой пароль содержит название озера по координатам [42.45397403650395, 130.6473932235334]");
        result.level = 16;
    }

    return result;
}

///Пароль должен быть любое слов
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level13(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 18 успешно пройдена"),
        level: 18,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 3] = [
        String::from("ship"), String::from("architect"),
        String::from("fishing"),
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
        result.message = String::from("Твой пароль должен включать лишние слово из списка [horse, ship, architect, fishing]");
        result.level = 17;
    }

    return result;
}


///Пароль должен включать слово painfully
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level14(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 19 успешно пройдена"),
        level: 19,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("painfully"), 
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
        result.message = String::from("Твой пароль должен включать 42 слово из песни Я боюсь людей");
        result.level = 18;
    }

    return result;
}

///Пароль должен быть не больше 47 символов
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level15(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 20 успешно пройдена"),
        level: 20,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("light"),
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        let length_password = password.len();
        if length_password > 47 {
            result.status = self::WEEK_PASSWORD_STATUS;
            result.message = String::from("Пароль должен быть не больше 47 символов");
            result.level = 19;
        }
    }

    return result;
}

///Пароль должен включать слово light 
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level16(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 21 успешно пройдена"),
        level: 21,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("light"),
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
        result.message = String::from("Пароль должен включать ответ на загадку: What can fill a room but takes up no space?");
        result.level = 20;
    }

    return result;
}

///Пароль должен включать слово light 
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level17(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 21 успешно пройдена"),
        level: 22,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("282"),
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
        result.message = String::from("Пароль должен включать число сколько миллионов просмотров на самом первом видео на youtube");
        result.level = 21;
    }

    return result;
}

///Пароль должен включать название операционной системы 
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level18(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 22 успешно пройдена"),
        level: 23,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample: [String; 1] = [
        String::from("windows"),
    ];
    
    let mut flag: bool = false;
    for item in sample {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == true {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Всё понятно с тобой, но я дам тебе еще попытку");
        result.level = 22;
        
        return result;
    }
    //----

    let mut lower_password = password.to_lowercase();
    let mut sample2: [String; 4] = [
        String::from("linux"), String::from("macos"), String::from("freebsd"), String::from("chromeos"), 
    ];
    
    let mut flag: bool = false;
    for item in sample2 {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Пароль должен название операционной системы (pc версия)");
        result.level = 22;
    }

    return result;
}


///Пароль должен включать дату рождения Владимира Жириновского 
/// 
/// String password введеный пользователем пароль
pub fn validate_scam_level19(password: String) -> ResultCheck {
    let mut result = ResultCheck {
        status: self::GOOD_PASSWORD_STATUS,
        message: String::from("Валидация уровня 23 успешно пройдена"),
        level: 24,
    };

    let mut lower_password = password.to_lowercase();
    let mut sample2: [String; 3] = [
        String::from("25041946"), String::from("25.04.1946"), String::from("25_04_1946")
    ];
    
    let mut flag: bool = false;
    for item in sample2 {
        if lower_password.rfind(&item).is_some() {
            flag = true;
            break;
        }
    }

    if flag == false {
        result.status = self::WEEK_PASSWORD_STATUS;
        result.message = String::from("Пароль должен включать дату рождения Владимира Жириновского");
        result.level = 23;
    }

    return result;
}
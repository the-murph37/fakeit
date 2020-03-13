use crate::data::computer;
use crate::misc;
use crate::datetime;
use std::ops::Add;

pub fn chrome() -> String {
    let randNum = misc::random(531, 536) + misc::random(0, 2);
    format!(
        "Mozilla/5.0 ({}) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.{}.0 Mobile Safari/{}",
        random_platform(),
        randNum,
        misc::random(36, 40),
        misc::random(800, 899),
        randNum
    )
}

pub fn firefox() -> String {
    // @TODO should be 2006-02-01
    let date = format!("{}-{}-{}", datetime::year(), datetime::month(), datetime::day());
    let platform = match misc::random(1,3) {
        1 => format!("({}; en-US; rv:1.9.{}.20)", windows_platform_token(), misc::random(0,3)),
        2 => format!("({}; rv:{}.0)", linux_platform_token(), misc::random(5, 8)),
        _ => format!("({} rv:{}.0)", mac_platform_token(), misc::random(2, 7)),
    };
    format!("Mozilla/5.0 {} Gecko/{} Firefox/{}.0", platform, date, misc::random(35, 37))
}

pub fn safari() -> String {
    let randNum = format!(
        "{}.{}.{}",
        misc::random(531, 536),
        misc::random(1, 51),
        misc::random(1, 8),
    );

    let ver = format!("{}.{}", misc::random(4, 6), misc::random(0,2 ));

    let mobileDevices = match misc::random(1, 2) {
        1 => String::from("iPhone; CPU iPhone OS"),
        _ => String::from("iPad; CPU OS"),
    };

    let platforms = match misc::random(1,3) {
        1 => format!("(Windows; U; {}) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}", windows_platform_token(), randNum, ver, randNum),
        2 => format!("({} rv:{}.0; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}", mac_platform_token(), misc::random(4, 7), randNum, ver, randNum),
        _ => format!("({} {}_{}_{} like Mac OS X; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{}.0.5 Mobile/8B{} Safari/6{}", mobileDevices, misc::random(7,9 ), misc::random(0, 3), misc::random(1, 3), randNum, misc::random(3, 5), misc::random(111, 120), randNum)
    };

    format!("Mozilla/5.0 {}", platforms)
}

pub fn opera() -> String {
    let platform = format!("({}; en-US) Presto/2.{}.{} Version/{}.00", random_platform(), misc::random(8, 13), misc::random(160, 355), misc::random(10, 13));

    format!("Opera/{}.{} {}", misc::random(8, 10), misc::random(10, 99), platform)
}

pub fn linux_platform_token() -> String {
    format!("X11; Linux {}", misc::random_data_str(computer::LINUX_PROCESSOR).to_string())
}

pub fn mac_platform_token() -> String {
    format!(
        "Macintosh; {} Mac OS X 10_{}_{}",
        misc::random_data_str(computer::MAC_PROCESSOR).to_string(),
        misc::random(5, 9),
        misc::random(0, 10),
    )
}

pub fn windows_platform_token() -> String {
    misc::random_data_str(computer::WINDOWS_PLATFORM).to_string()
}

pub fn random_platform() -> String {
    match misc::random(1,3) {
        1 => linux_platform_token(),
        2 => mac_platform_token(),
        _ => windows_platform_token(),
    }
}
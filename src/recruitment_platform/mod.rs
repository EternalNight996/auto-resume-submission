use core::fmt;
use std::str::FromStr;

pub mod boss_zhipin;
pub mod liepin;
#[derive(Debug, serde::Deserialize)]
pub struct EmptyData {}

// 请求任务类型
#[derive(Debug, Clone, Copy)]
pub enum CmdType {
    List,
    Sayhi,
    AutoMsg,
    Token,
    SendResume,
    PrintSayhi,
    UpdateSayhi,
    PrintCity,
    UpdateCity,
    GetBossInfo,
    GetExpectList
}
impl fmt::Display for CmdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CmdType::List => "list",
            CmdType::Sayhi => "sayhi",
            CmdType::Token => "token",
            CmdType::SendResume => "send_resume",
            CmdType::PrintSayhi => "print_sayhi",
            CmdType::UpdateSayhi => "update_sayhi",
            CmdType::PrintCity => "print_city",
            CmdType::UpdateCity => "update_city",
            CmdType::GetBossInfo =>  "get_boss_info",
            CmdType::GetExpectList => "get_expect_list",
            CmdType::AutoMsg => "auto_msg",
        };
        write!(f, "{}", s)
    }
}
impl From<String> for CmdType {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
impl From<&str> for CmdType {
    fn from(s: &str) -> Self {
        match s {
            "list" => CmdType::List,
            "sayhi" => CmdType::Sayhi,
            "token" => CmdType::Token,
            "send_resume" => CmdType::SendResume,
            "print_sayhi" => CmdType::PrintSayhi,
            "update_sayhi" => CmdType::UpdateSayhi,
            "print_city" => CmdType::PrintCity,
            "update_city" => CmdType::UpdateCity,
            "get_boss_info" => CmdType::GetBossInfo,
            "get_expect_list" => CmdType::GetExpectList,
            "auto_msg" => CmdType::AutoMsg,
            _ => todo!(),
        }
    }
}

// 请求平台
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ReqPlatform {
    Zp,
    Lp
}
impl fmt::Display for ReqPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ReqPlatform::Zp => "zp",
            ReqPlatform::Lp => "lp",
        };
        write!(f, "{}", s)
    }
}
impl From<String> for ReqPlatform {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
impl From<&str> for ReqPlatform {
    fn from(s: &str) -> Self {
        match s {
            "zp" => ReqPlatform::Zp,
            "lp" =>ReqPlatform::Lp,
            _ => todo!(),
        }
    }
}

impl FromStr for ReqPlatform {
    type Err = (); // 定义错误类型，可以根据实际情况修改

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 将字符串解析为 ReqPlatform 实例的逻辑
        s.parse::<ReqPlatform>()
    }
}

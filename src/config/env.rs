use std::env;

use serde::{Deserialize, Serialize};

use crate::{
    common::parse_sayhi,
    recruitment_platform::{liepin::LpCookies, ReqPlatform},
};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct EnvConfig {
    pub name: String,
    pub city: usize,
    pub page: usize,
    pub page_end: usize,
    pub page_size: usize,
    pub zp_cookie: String,
    pub zp_token: String,
    pub sayhi_repeat: bool,
    pub sayhi: Vec<String>,
    pub sayhi_prefix: String,
    pub log_level: u8,
    pub auto_request_platform_list: Vec<ReqPlatform>,
    pub max_core: usize,
    pub timeout: u64,
    pub auto_send: bool,
    pub encrypt_expect_id: usize,
    pub salary: String,
    pub experience: String,
    pub degree: String,
    pub template_id: u8,
    pub sort_type: String,
    pub sayhi_tempalte_id: String,
    pub sayhi_send_resume: bool,
    pub multi_business_district: String,
    pub search: String,
    pub lp_token: String,
    pub lp_cookie: String,
    pub lp_cookies: Option<LpCookies>,
}

impl Default for EnvConfig {
    fn default() -> EnvConfig {
        return EnvConfig {
            name: env::var("ARS_NAME").unwrap_or("ARS".to_string()),
            city: env::var("ARS_CITY")
                .unwrap_or("0".to_string())
                .parse::<usize>()
                .unwrap(),
            page: env::var("ARS_PAGE")
                .unwrap_or("1".to_string())
                .parse::<usize>()
                .unwrap(),
            page_end: env::var("ARS_PAGE_END")
                .unwrap_or("2".to_string())
                .parse::<usize>()
                .unwrap(),
            page_size: env::var("ARS_PAGE_SIZE")
                .unwrap_or("15".to_string())
                .parse::<usize>()
                .unwrap(),
            zp_cookie: env::var("ARS_ZP_COOKIE").unwrap_or("".to_string()),
            sayhi_repeat: env::var("ARS_SAYHI_REPEAT")
                .unwrap_or("false".to_string())
                .parse::<bool>()
                .unwrap(),
            sayhi: env::var("ARS_SAYHI")
                .and_then(|x| Ok(parse_sayhi(&x)))
                .unwrap_or(vec![]),
            sayhi_prefix: env::var("ARS_SAYHI_PREFIX").unwrap_or("".to_string()),
            log_level: env::var("RUST_LOG")
                .unwrap_or(if cfg!(debug_assertions) { 4 } else { 3 }.to_string())
                .parse::<u8>()
                .unwrap(),
            auto_request_platform_list: {
                let s = env::var("ARS_AUTO_REQUEST_PLATFORM_LIST").unwrap();
                if s.find(',').is_some() {
                    s.split(",")
                        .filter_map(|x| Some(ReqPlatform::from(x)))
                        .collect::<Vec<ReqPlatform>>()
                } else {
                    vec![ReqPlatform::from(s)]
                }
            },
            max_core: env::var("ARS_MAX_CORE")
                .unwrap_or("10".to_string())
                .parse::<usize>()
                .unwrap(),
            timeout: env::var("ARS_TIMEOUT")
                .unwrap_or("5000".to_string())
                .parse::<u64>()
                .unwrap(),
            auto_send: env::var("ARS_AUTO_SEND")
                .unwrap_or("false".to_string())
                .parse::<bool>()
                .unwrap(),

            encrypt_expect_id: {
                let id = env::var("ARS_ENCRYPT_EXPECT_ID")
                    .unwrap_or("0".to_string())
                    .parse::<usize>()
                    .unwrap();
                if id > 2 {
                    panic!("请正确设置ARS_ENCRYPT_EXPECT_ID变量，正确范围为0-2;")
                }
                id
            },
            salary: env::var("ARS_SALARY").unwrap_or("".to_string()),
            experience: env::var("ARS_EXPERIENCE").unwrap_or("".to_string()),
            degree: env::var("ARS_DEGREE").unwrap_or("".to_string()),
            template_id: env::var("ARS_TEMPLATE_ID")
                .unwrap_or("0".to_string())
                .parse::<u8>()
                .unwrap(),
            sort_type: env::var("ARS_SORT_TYPE").unwrap_or("".to_owned()),
            sayhi_tempalte_id: env::var("ARS_SAYHI_TEMPLATE_ID").unwrap_or("".to_owned()),
            sayhi_send_resume: env::var("ARS_SAYHI_SEND_RESUME")
                .unwrap_or("fasle".to_string())
                .parse::<bool>()
                .unwrap(),
            multi_business_district: env::var("ARS_MULTI_BUSINESS_DISTRICT")
                .unwrap_or("".to_owned()),
            search: env::var("ARS_SEARCH").unwrap_or("".to_string()),
            zp_token: env::var("ARS_ZP_TOKEN").unwrap_or("".to_string()),
            lp_token: env::var("ARS_LP_TOKEN").unwrap_or("".to_string()),
            lp_cookie: env::var("ARS_LP_COOKIE").unwrap_or("".to_string()),
            lp_cookies: None,
        };
    }
}

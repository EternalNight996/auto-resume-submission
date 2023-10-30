// #![warn(
//     rust_2018_idioms,
//     unreachable_pub,
//     bad_style,
//     // const_err,
//     dead_code,
//     improper_ctypes,
//     non_shorthand_field_patterns,
//     no_mangle_generic_items,
//     overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     private_in_public,
//     unconditional_recursion,
//     unused,
//     unused_allocation,
//     unused_comparisons,
//     unused_parens,
//     while_true
// )]
// #![doc(test(attr(allow(unused_variables), deny(warnings))))]
use crate::config::env::EnvConfig;
use cmd::ars_task;
use once_cell::sync::Lazy;
use recruitment_platform::liepin::common::lp_parse_cookies;

mod chat;
mod cmd;
mod common;
mod config;
mod recruitment_platform;

static GLOBAL_CONFIG_DATA: Lazy<EnvConfig> = Lazy::new(|| {
    let mut env = EnvConfig::default();
    if env.lp_cookie.len() > 0 {
        env.lp_cookies = Some(lp_parse_cookies(&env.lp_cookie));
    }
    return env;
});
#[tokio::main]
async fn main() {
    dotenv::from_path(".env").unwrap();
    // init log level with config
    config::logger::init_logger(&GLOBAL_CONFIG_DATA.name, GLOBAL_CONFIG_DATA.log_level);
    let res = ars_task(&GLOBAL_CONFIG_DATA).await;
}

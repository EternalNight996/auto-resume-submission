use super::LpCookies;

pub fn lp_parse_cookies(s: &str) -> LpCookies {
    let s2 = s.split_whitespace().collect::<Vec<&str>>();
    let s3 = s2.concat();
    let cookie_list: Vec<(&str, &str)> = s3.split(';').filter_map(|x| x.split_once('=')).collect();
    let mut c = LpCookies::default();
    for s in cookie_list {
        match s.0 {
            "__uuid" => c.uuid = s.1.to_string(),
            "XSRF-TOKEN" => c.xsrf_token = s.1.to_string(),
            "__gc_id" => c.gc_id = s.1.to_string(),
            "inited_user" => c.inited_user = s.1.to_string(),
            "user_roles" => c.user_roles = s.1.to_string(),
            "user_name" => c.user_name = s.1.to_string(),
            "need_bind_tel" => c.need_bind_tel = s.1.to_string(),
            "new_user" => c.new_user = if s.1 == "false" { false } else { true },
            "c_flag" => c.c_flag = s.1.to_string(),
            "imId" => c.im_id = s.1.to_string(),
            "imId_0" => c.im_id_0 = s.1.to_string(),
            "imClientId" => c.im_client_id = s.1.to_string(),
            "imClientId_0" => c.im_client_id_0 = s.1.to_string(),
            "imApp_0" => c.im_app_0 = s.1.to_string(),
            "user_photo" => c.user_photo = s.1.to_string(),
            "UniqueKey" => c.unique_key = s.1.to_string(),
            "fe_im_openchatwin" => c.fe_im_openchatwin = s.1.to_string(),
            _ => continue,
        }
    }
    c
}

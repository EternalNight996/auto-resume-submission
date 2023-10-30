/*
 * @Author: EternalNight eternalnightyeah2@yeah.net
 * @Date: 2023-10-11 11:07:55
 * @LastEditors: EternalNight eternalnightyeah2@yeah.net
 * @LastEditTime: 2023-10-19 21:34:57
 * @FilePath: \auto-resume-submission\src\cmd.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */

use std::{any::Any, collections::HashMap, thread::spawn, time};

use futures::executor::block_on;
use reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE, HOST, USER_AGENT},
    Client, Url,
};
use tracing::{debug, error, info, warn};

use crate::{
    chat::connect_chat,
    config::env::EnvConfig,
    recruitment_platform::{
        boss_zhipin::{
            services::{
                get_zp_boss_info, get_zp_city_list, get_zp_expect_list, get_zp_list,
                get_zp_resume_list, get_zp_sayhi_list, get_zp_token, update_zp_location_city,
                update_zp_sayhi_template, zp_send_resume,
            },
            ZpCityRes, ZpListData, ZpListList, ZpRes, ZpSayhi, ZpSendResume, ZpToken,
            BOSS_ZHIPIN_TOKEN,
        },
        liepin::{
            services::{
                get_lp_expect_list, get_lp_list, lp_auto_send_msg, lp_chat_msg_list,
                lp_contact_list, lp_send_resume, lp_send_sayhi,
            },
            LpDataJobRes, LpDataRes, LpListData, LpListData2, LpListDataExpect, LpSayhiData,
            LpSayhiDataInfo, LpSendResumeData,
        },
        CmdType, ReqPlatform,
    },
};
use tokio::time::{sleep, Duration};

// 自动化任务请求
pub async fn ars_task(config: &EnvConfig) {
    if config.auto_request_platform_list.len() > 0 {
        let mut handler: Vec<_> = Vec::with_capacity(config.auto_request_platform_list.len());
        for rp in &config.auto_request_platform_list {
            debug!("trying to running platform -> {}", rp);
            let cp_config = config.clone();
            let cp_rp = rp.clone();
            handler.push(tokio::spawn(async move {
                let _r = auto_cmd(cp_rp, &cp_config).await;
            }));
        }
        let _r = futures::future::join_all(handler).await;
        let mut i = 0;
        loop {
            i += 1;
            if i == 60 {
                break;
            }
            warn!("timeout超时 {}", i);
            sleep(Duration::from_millis(config.timeout)).await;
        }
    } else {
        panic!("未填写应聘平台")
    }
}
// 自动化命令
pub async fn auto_cmd(rp: ReqPlatform, config: &EnvConfig) {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36".parse().unwrap());
    let gtoken = |r: ReqPlatform, c: EnvConfig, h: HeaderMap| async move {
        ars_cmd(&r, CmdType::Token, config, h, None)
            .await
            .unwrap()
            .downcast::<ZpToken>()
            .unwrap()
            .token
    };
    match rp {
        ReqPlatform::Zp => {
            headers.insert(COOKIE, config.zp_cookie.parse().unwrap());
            let token = if !config.zp_token.is_empty() {
                config.zp_token.clone()
            } else {
                gtoken(rp.clone(), config.clone(), headers.clone()).await
            }
            .parse()
            .unwrap();
            headers.insert(BOSS_ZHIPIN_TOKEN, token);
            zp_init_data(&config, headers.clone()).await.unwrap();
        }
        ReqPlatform::Lp => {
            let _ = headers.insert(
                "x-xsrf-token",
                config
                    .lp_cookies
                    .clone()
                    .unwrap()
                    .xsrf_token
                    .parse()
                    .unwrap(),
            );
            let _ = headers.insert(
                "x-fscp-std-info",
                r#"{"client_id": "40106"}"#.parse().unwrap(),
            );
            let _ = headers.insert("x-client-type", "web".parse().unwrap());
            let _ = headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
            let _ = headers.insert("x-fscp-version", "1.1".parse().unwrap());
            let _ = headers.insert(
                "x-fscp-trace-id",
                "d8e52bd2-1424-45e3-8a1f-671d275db643".parse().unwrap(),
            );
            let _ = headers.insert(COOKIE, config.lp_cookie.parse().unwrap());
        }
    };
    let mut cp_config = config.clone();
    let cp_rp = rp.clone();
    let cp2_config = config.clone();
    let cp_headers = headers.clone();
    spawn(move || {
        let res_auto_msg = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(ars_cmd(
                &cp_rp,
                CmdType::AutoMsg,
                &cp2_config,
                cp_headers,
                None,
            ));
        info!("Platform[{}] Cmd[AutoMsg] 结束运行！", cp_rp);
    });
    let mut error = 0;
    for start in config.page..config.page_end {
        let res_list = ars_cmd(&rp, CmdType::List, &cp_config, headers.clone(), None).await;
        if res_list.is_some() {
            let res_sayhi =
                ars_cmd(&rp, CmdType::Sayhi, &cp_config, headers.clone(), res_list).await;
            // if res_sayhi.is_none() {
            //     info!("Platform[{}] Cmd[Sayhi] 错误结束运行！", rp);
            //     break;
            // }
        } else {
            if error > 3 {
                error!(
                    "Platform[{}] error count {}； 结束运行 请检查令牌",
                    rp, error
                );
                break;
            }
            error += 1;
            warn!("Platform[{}] error count {}", rp, error);
        }
        cp_config.page = start;
        sleep(Duration::from_millis(config.timeout)).await;
    }
}

// 命令
pub async fn ars_cmd(
    req_platform: &ReqPlatform,
    cmd_type: CmdType,
    config: &EnvConfig,
    headers: HeaderMap,
    data: Option<Box<dyn Any + Send + 'static>>,
) -> Option<Box<dyn Any + Send + 'static>> {
    // let semaphore = Arc::new(Semaphore::new(config.max_core));
    match req_platform {
        ReqPlatform::Zp => match cmd_type {
            CmdType::List => {
                let client = Client::new();
                let list = get_zp_expect_list(&client, headers.clone())
                    .await?
                    .expect_list;
                let res = get_zp_list(&client, config, headers, list).await?;
                return Some(Box::new(res));
            }
            CmdType::Sayhi => {
                let d = data?.downcast::<Vec<ZpListData>>().ok()?.clone();
                let mut i = 0;
                let mut handler: Vec<_> = Vec::with_capacity(d.len());
                for ref j in *d {
                    i += 1;
                    let url = format!("https://www.zhipin.com/wapi/zpgeek/friend/add.json?securityId={}&jobId={}&lid={}", j.security_id, j.encrypt_job_id, j.lid);
                    let mut cp_headers = headers.clone();
                    let token = get_zp_token(&Client::new(), cp_headers.clone()).await?;
                    cp_headers.insert(BOSS_ZHIPIN_TOKEN, token.token.parse().unwrap());
                    let cp_req_platform = req_platform.clone();
                    let cp_cmd_type = cmd_type.clone();
                    let cp_config = config.clone();
                    let timeout = config.timeout * i;
                    let cp_data = j.clone();
                    let sayhi_prefix = if cp_config.sayhi_prefix.len() > 0 {
                        cp_config.sayhi_prefix.clone()
                    } else {
                        format!("您好 {}，", j.boss_name)
                    };
                    info!(
                        "{}",
                        format!(
                            "No.{} -> Platform[{}] CMD[{}] URL[{}]",
                            i, cp_req_platform, cp_cmd_type, url
                        )
                    );
                    handler.push(tokio::spawn(async move {
                        let client = Client::new();
                        match client.post(&url).headers(cp_headers.clone()).send().await {
                            Ok(d) => {
                                match d.json::<ZpRes<ZpSayhi>>().await {
                                    Ok(d2) => {
                                        let res = d2.zp_data;
                                        // 追加自定义问候
                                        if res.show_greeting == true
                                            || res.show_greeting == false && cp_config.sayhi_repeat
                                        {
                                            if cp_config.auto_send {
                                                // let hi =
                                                //     format!("{}{}", sayhi_prefix, cp_config.sayhi);
                                                // // 有BUG
                                                // let res2 = connect_chat(
                                                //     &cp_req_platform,
                                                //     &cp_config,
                                                //     cp_headers.clone(),
                                                // )
                                                // .await;
                                                // todo!()
                                            }
                                        } else {
                                            // 发送简历
                                            if cp_config.sayhi_send_resume {
                                                if let Some(list) =
                                                    get_zp_resume_list(&client, cp_headers.clone())
                                                        .await
                                                {
                                                    let encrypt_resume_id = list.resume_list
                                                        [list.resume_count - 1]
                                                        .resume_id
                                                        .to_string();
                                                    if let Some(boss_info) = get_zp_boss_info(
                                                        &client,
                                                        cp_headers.clone(),
                                                        &cp_data,
                                                    )
                                                    .await
                                                    {
                                                        let _r = zp_send_resume(
                                                            &client,
                                                            ZpSendResume {
                                                                security_id: boss_info
                                                                    .data
                                                                    .security_id,
                                                                r#type: "3".to_owned(),
                                                                encrypt_resume_id,
                                                            },
                                                            cp_headers,
                                                        )
                                                        .await;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        // 发送简历
                                        if cp_config.sayhi_send_resume {
                                            if let Some(list) =
                                                get_zp_resume_list(&client, cp_headers.clone())
                                                    .await
                                            {
                                                let encrypt_resume_id = list.resume_list
                                                    [list.resume_count - 1]
                                                    .resume_id
                                                    .to_string();
                                                let _r = zp_send_resume(
                                                    &client,
                                                    ZpSendResume {
                                                        security_id: cp_data.security_id,
                                                        r#type: "3".to_owned(),
                                                        encrypt_resume_id,
                                                    },
                                                    cp_headers,
                                                )
                                                .await;
                                            }
                                        }
                                        error!("{}", e)
                                    }
                                }
                            }
                            Err(e) => error!("{:?}", e),
                        }
                    }));
                    // 延时任务
                    sleep(Duration::from_millis(timeout)).await;
                }
                let _r = futures::future::join_all(handler).await;
                return Some(Box::new(()));
            }
            CmdType::Token => {
                let res = get_zp_token(&Client::new(), headers).await?;
                return Some(Box::new(res));
            }
            CmdType::SendResume => {
                let d = data?.downcast::<ZpListData>().ok()?;
                // 发送简历
                if let Some(list) = get_zp_resume_list(&Client::new(), headers.clone()).await {
                    let encrypt_resume_id = list.resume_list[list.resume_count - 1]
                        .resume_id
                        .to_string();
                    if let Some(r) = zp_send_resume(
                        &Client::new(),
                        ZpSendResume {
                            security_id: (*d.security_id).to_string(),
                            r#type: "3".to_owned(),
                            encrypt_resume_id,
                        },
                        headers,
                    )
                    .await
                    {
                        return Some(Box::new(r));
                    }
                }
            }
            CmdType::PrintSayhi => {
                let list = get_zp_sayhi_list(&Client::new(), headers).await?;
                for t in &list.greeting_template_id {
                    println!("{}={}={}", t.template_id, t.encrypt_id, t.template);
                }
                return Some(Box::new(list));
            }
            CmdType::UpdateSayhi => {
                if !config.sayhi_tempalte_id.is_empty() {
                    if let Some(res) =
                        update_zp_sayhi_template(&Client::new(), config, headers).await
                    {
                        return Some(Box::new(res));
                    }
                }
            }
            CmdType::PrintCity => {
                let list = get_zp_city_list(&Client::new(), headers).await?;
                fn print_city_next(t: &Vec<ZpCityRes>) -> Vec<String> {
                    let mut ls: Vec<String> = vec![];
                    for n in t {
                        if let Some(n2) = &n.sub_level_model_list {
                            let r = print_city_next(n2);
                            let mut ts = "".to_string();
                            for s in r {
                                ts.push_str(&format!("{},", s));
                            }
                            ls.push(format!("({}={})[{}]\n", n.code, n.name, ts));
                        } else {
                            ls.push(format!("{}={}", n.code, n.name));
                        }
                    }
                    return ls;
                }
                let ls = print_city_next(&list.city_list);
                for s in ls {
                    println!("{}", s);
                }
                return Some(Box::new(list));
            }
            CmdType::UpdateCity => {
                let r = update_zp_location_city(&Client::new(), config, headers).await?;
                return Some(Box::new(r));
            }
            CmdType::GetBossInfo => {
                let d = data?.downcast::<ZpListData>().ok()?;
                let r = get_zp_boss_info(&Client::new(), headers, &*d).await?;
                return Some(Box::new(r));
            }
            CmdType::GetExpectList => {
                let list = get_zp_expect_list(&Client::new(), headers).await?;
                return Some(Box::new(list));
            }
            CmdType::AutoMsg => {}
        },
        ReqPlatform::Lp => match cmd_type {
            CmdType::List => {
                let client = Client::new();
                let list = get_lp_expect_list(&client, headers.clone()).await?;
                let res = get_lp_list(&client, &config, headers, &list.valid_expects).await?;
                return Some(Box::new(res));
            }
            CmdType::Sayhi => {
                let d = data?.downcast::<Vec<LpDataJobRes>>().ok()?.clone();
                let mut i = 0;
                for ref j in *d {
                    i += 1;
                    let timeout = config.timeout * i;
                    let decode = urlencoding::decode(&j.data_info).unwrap().to_string();
                    let dinfo = serde_json::from_str::<LpSayhiDataInfo>(&decode).unwrap();
                    match lp_send_sayhi(
                        &Client::new(),
                        headers.clone(),
                        LpSayhiData {
                            head_id: dinfo.head_id,
                            ck_id: "".to_string(),
                            job_id: dinfo.job_id,
                            job_kind: dinfo.job_kind,
                            recruiter_id: j.recruiter.recruiter_id.clone(),
                            shield_comp: true,
                        },
                    )
                    .await
                    {
                        Some(r) => info!("CmdType::Sayhi {:?} ", r),
                        None => error!("CmdType::Sayhi error"),
                    };
                    // 延时任务
                    sleep(Duration::from_millis(timeout)).await;
                }
                return Some(Box::new(()));
            }
            CmdType::Token => todo!(),
            CmdType::SendResume => todo!(),
            CmdType::PrintSayhi => todo!(),
            CmdType::UpdateSayhi => todo!(),
            CmdType::PrintCity => todo!(),
            CmdType::UpdateCity => todo!(),
            CmdType::GetBossInfo => todo!(),
            CmdType::GetExpectList => {
                let list = get_lp_expect_list(&Client::new(), headers).await?;
                return Some(Box::new(list));
            }
            CmdType::AutoMsg => {
                if config.auto_send || config.sayhi_send_resume {
                    let mut i = 0;
                    let l = lp_contact_list(&Client::new(), config, headers.clone())
                        .await?
                        .list;
                    for cl in l {
                        i += 1;
                        let timeout = config.timeout * i;
                        let client = Client::new();
                        if let Some(x) =
                            lp_chat_msg_list(&client, config, headers.clone(), &cl).await
                        {
                            // 因为打招呼完后，默认有三条消息
                            if x.list.len() == 3 || x.list.len() > 3 && config.sayhi_repeat {
                                if config.auto_send {
                                    if let Some(r) =
                                        lp_auto_send_msg(&client, &config, headers.clone(), &cl)
                                            .await
                                    {
                                        if r.code == Some("200".to_owned())
                                            || r.code == Some("0".to_owned())
                                        {
                                            info!("成功发送简历 {:?}", r);
                                        } else {
                                            error!("发送简历失败 {:?}", r);
                                        }
                                    } else {
                                        error!("发送消息失败");
                                    }
                                }
                                // 发送简历
                                if config.sayhi_send_resume {
                                    if let Some(r) =
                                        lp_send_resume(&client, &config, headers.clone(), &cl).await
                                    {
                                        if r.code == Some("200".to_owned())
                                            || r.code == Some("0".to_owned())
                                        {
                                            info!("成功发送简历 {:?}", r);
                                        } else {
                                            error!("发送简历失败 {:?}", r);
                                        }
                                    } else {
                                        error!("发送简历失败");
                                    }
                                }
                            } else {
                                warn!("{}. 规则无法进行消息发送 {}[{}]", i, cl.name, cl.user_id);
                            }
                        };
                        // 延时任务
                        sleep(Duration::from_millis(timeout)).await;
                    }
                }
            }
        },
    };
    None
}

/* 初始化配置数据 */
pub async fn zp_init_data(config: &EnvConfig, headers: HeaderMap) -> Option<()> {
    // 打印城市模板
    // let _ = ars_cmd(
    //     &ReqPlatform::Zp,
    //     CmdType::PrintCity,
    //     config,
    //     headers.clone(),
    //     None,
    // )
    // .await;

    // 打印打招呼模板
    // let _ = ars_cmd(
    //     &ReqPlatform::Zp,
    //     CmdType::PrintSayhi,
    //     config,
    //     headers.clone(),
    //     None,
    // )
    // .await;
    // 更新定位
    let _ = ars_cmd(
        &ReqPlatform::Zp,
        CmdType::UpdateCity,
        config,
        headers.clone(),
        None,
    )
    .await;
    // 更新打招呼模板
    let _ = ars_cmd(
        &ReqPlatform::Zp,
        CmdType::UpdateSayhi,
        config,
        headers,
        None,
    )
    .await;
    Some(())
}

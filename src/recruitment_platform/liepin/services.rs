use std::time;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Client,
};
use tracing::{error, info};

use crate::{common::now, config::env::EnvConfig, recruitment_platform::EmptyData};

use super::{
    LpChatDataRes, LpContactDataRes, LpContactListRes, LpDataJobRes, LpDataRes, LpExpectListRes,
    LpListData, LpListData2, LpListDataExpect, LpRes, LpSayhiData, LpSendMsgData,
    LpSendMsgDataPayload, LpSendMsgDataPayloadBodies, LpSendMsgDataPayloadExt,
    LpSendMsgDataPayloadExtBody, LpSendMsgDataPayloadExtBodyBizData,
    LpSendMsgDataPayloadExtBodyBizDataQuote, LpSendMsgDataPayloadExtBodyBsData, LpSendMsgRes,
    LpSendMsgType, LpSendResumeData,
};

use std::time::{SystemTime, UNIX_EPOCH};

/* 猎聘获取聊天消息列表 */
pub async fn lp_chat_msg_list(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
    d: &LpContactListRes,
) -> Option<LpContactDataRes<LpChatDataRes>> {
    let url = "https://api-c.liepin.com/api/com.liepin.im.c.chat.chat-list";
    let c = config.lp_cookies.clone()?;
    let params = [
        ("imUserType", 0.to_string()),
        ("imClientId", c.im_client_id),
        ("imId", c.im_id),
        ("imApp", c.im_app_0),
        ("oppositeImId", d.opposite_im_id.clone()),
        ("maxMessageId", "-1".to_string()),
        ("pageSize", config.page_size.to_string()),
    ];
    let body = serde_urlencoded::to_string(&params).ok()?;
    match client
        .post(url)
        .body(body)
        .headers(headers)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("x-fscp-std-info", r#"{"client_id": "40338"}"#)
        .send()
        .await
    {
        Ok(x) => match x.json::<LpRes<LpContactDataRes<LpChatDataRes>>>().await {
            Ok(x2) => return x2.data,
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}
/* 猎聘获取聊天列表 */
pub async fn lp_contact_list(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
) -> Option<LpContactDataRes<LpContactListRes>> {
    let url = "https://api-c.liepin.com/api/com.liepin.im.c.contact.my-contact-list";
    let c = config.lp_cookies.clone()?;
    let params = [
        ("imUserType", 0.to_string()),
        ("imClientId", c.im_client_id),
        ("imId", c.im_id),
        ("imApp", c.im_app_0),
        ("curPage", 0.to_string()),
        ("hasContact", "false".to_owned()),
        ("currentPage", 0.to_string()),
        ("pageSize", 30.to_string()),
    ];
    let body = serde_urlencoded::to_string(&params).ok()?;
    match client
        .post(url)
        .body(body)
        .headers(headers)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("x-fscp-std-info", r#"{"client_id": "40338"}"#)
        .send()
        .await
    {
        Ok(x) => match x.json::<LpRes<LpContactDataRes<LpContactListRes>>>().await {
            Ok(x2) => return x2.data,

            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}
/* 猎聘发送消息 */
pub async fn lp_send_msg(
    client: &Client,
    headers: HeaderMap,
    d: LpSendMsgData,
) -> Option<LpRes<LpSendMsgRes>> {
    let url = "https://api-c.liepin.com/api/com.liepin.im.c.chat.send-push";
    let params = [
        ("imUserType", d.im_user_type),
        ("imClientId", d.im_client_id),
        ("imId", d.im_id),
        ("imApp", d.im_app),
        ("oppositeImId", d.opposite_im_id),
        ("oppositeImUserType", d.opposite_im_user_type),
        ("msgTime", d.msg_time.to_string()),
        ("chatType", d.chat_type.to_string()),
        ("msgType", d.msg_type),
        ("save", d.save),
        ("count", d.count.to_string()),
        ("payload", serde_json::to_string(&d.payload).ok()?),
    ];
    let body = serde_urlencoded::to_string(&params).ok()?;
    match client
        .post(url)
        .body(body)
        .headers(headers)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
    {
        Ok(x) => match x.json::<LpRes<LpSendMsgRes>>().await {
            Ok(x2) => return Some(x2),
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}
/* 猎聘发送简历 */
pub async fn lp_send_resume(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
    cl: &LpContactListRes,
) -> Option<LpRes<EmptyData>> {
    let cookies = config.lp_cookies.clone()?;
    let url = "https://api-c.liepin.com/api/com.liepin.im.c.chat.send-resume";
    let d = LpSendResumeData {
        im_user_type: cl.im_user_type.clone(),
        im_client_id: cookies.im_client_id,
        im_id: cookies.im_id,
        im_app: cookies.im_app_0,
        opposite_im_id: cl.opposite_im_id.clone(),
    };
    let params = [
        ("imUserType", d.im_user_type),
        ("imClientId", d.im_client_id),
        ("imId", d.im_id),
        ("imApp", d.im_app),
        ("oppositeImId", d.opposite_im_id),
    ];
    let body = serde_urlencoded::to_string(&params).ok()?;
    match client
        .post(url)
        .body(body)
        .headers(headers)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
    {
        Ok(x) => match x.json::<LpRes<EmptyData>>().await {
            Ok(x2) => return Some(x2),
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}
pub async fn lp_auto_send_msg(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
    cl: &LpContactListRes,
) -> Option<LpRes<LpSendMsgRes>> {
    let cookies = config.lp_cookies.clone()?;
    // 发送消息
    let sayhi_prefix = if config.sayhi_prefix.len() > 0 {
        config.sayhi_prefix.clone()
    } else {
        format!("您好 {}，", cl.name)
    };
    let mut msg_list = vec![];
    let msg_type = LpSendMsgType::Text;
    for s in &config.sayhi {
        let msg = format!("{}{}", sayhi_prefix, s);
        msg_list.push(LpSendMsgDataPayloadBodies {
            msg,
            r#type: msg_type.to_string(),
        });
    }
    let msg_len = msg_list.len();
    let res = lp_send_msg(
        client,
        headers.clone(),
        LpSendMsgData {
            im_user_type: cl.im_user_type.clone(),
            im_client_id: cookies.im_client_id.clone(),
            im_id: cookies.im_id.clone(),
            im_app: cookies.im_app_0.clone(),
            opposite_im_id: cl.opposite_im_id.clone(),
            opposite_im_user_type: cl.opposite_im_user_type.clone(),
            msg_time: now()?,
            chat_type: msg_type.to_chat_type(),
            msg_type: msg_type.to_string(),
            save: "".to_owned(),
            count: msg_len,
            payload: LpSendMsgDataPayload {
                ext: LpSendMsgDataPayloadExt {
                    ext_type: 1,
                    ext_body: LpSendMsgDataPayloadExtBody {
                        biz_type: "1".to_string(),
                        biz_data: LpSendMsgDataPayloadExtBodyBizData {
                            quote: LpSendMsgDataPayloadExtBodyBizDataQuote {},
                            sender_user_id: cookies.inited_user,
                            receiver_user_id: cl.user_id.clone(),
                        },
                        bs_data: LpSendMsgDataPayloadExtBodyBsData {},
                    },
                },
                bodies: msg_list,
                push: msg_len.to_string(),
            },
        },
    )
    .await?;
    Some(res)
}
/* 获取猎聘数据 */
pub async fn lp_send_sayhi(
    client: &Client,
    headers: HeaderMap,
    d: LpSayhiData,
) -> Option<LpRes<EmptyData>> {
    let url = format!("https://api-c.liepin.com/api/com.liepin.im.c.chat.open-chat");
    let params = [
        ("head_id", d.head_id),
        ("ck_id", "".to_owned()),
        ("jobId", d.job_id.to_string()),
        ("jobKind", d.job_kind),
        ("recruiterId", d.recruiter_id),
        ("shieldComp", d.shield_comp.to_string()),
    ];
    let body = serde_urlencoded::to_string(&params).ok()?;
    match client
        .post(&url)
        .body(body)
        .headers(headers.clone())
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send()
        .await
    {
        Ok(x) => match x.json::<LpRes<EmptyData>>().await {
            Ok(x2) => return Some(x2),
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}

/* 获取猎聘数据 */
pub async fn get_lp_list(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
    d: &Vec<LpListDataExpect>,
) -> Option<Vec<LpDataJobRes>> {
    let task = |client: Client, config: EnvConfig, headers: HeaderMap, selected_expect: String| async move {
        let url = if config.template_id == 0 || config.template_id == 1 {
            "https://api-c.liepin.com/api/com.liepin.csearch.home-recommend-job-new"
        } else if config.template_id == 2 {
            "https://api-c.liepin.com/api/com.liepin.csearch.home-recommend-job-new"
        } else {
            panic!("请正确配置ARS_TEMPLATE_ID变量");
        };
        let operate_kind = "LOGIN";
        let exist_fallback_result = false;
        let sort_type = if config.sort_type == "1" {
            "PC_HP_MIX"
        } else if config.sort_type == "2" {
            "PC_HP_NEW"
        } else {
            panic!("ARS_SORT_TYPE 变量未正常配置")
        };
        match client
            .post(url)
            .headers(headers)
            .json(&LpListData {
                data: LpListData2 {
                    exist_fallback_result,
                    operate_kind,
                    selected_expect: &selected_expect,
                    sort_type,
                },
            })
            .send()
            .await
        {
            Ok(x) => match x.json::<LpRes<LpDataRes<LpDataJobRes>>>().await {
                Ok(x2) => return Some(x2.data?),
                Err(e) => error!("{}", e),
            },
            Err(e) => error!("{}", e),
        }
        None
    };
    if config.encrypt_expect_id == 0 {
        let mut rlist = vec![];
        for v in d {
            let selected_expect = serde_json::to_string(&v).ok()?;
            if let Some(x) = task(
                client.clone(),
                config.clone(),
                headers.clone(),
                selected_expect,
            )
            .await
            {
                rlist.push(x.data);
            }
        }
        let c = rlist.concat();
        return Some(c);
    } else {
        let selected_expect = serde_json::to_string(&d[config.encrypt_expect_id - 1]).ok()?;
        if let Some(x) = task(
            client.clone(),
            config.clone(),
            headers.clone(),
            selected_expect,
        )
        .await
        {
            return Some(x.data);
        }
    }
    None
}
/* 获取猎聘职业列表数据 */
pub async fn get_lp_expect_list(client: &Client, headers: HeaderMap) -> Option<LpExpectListRes> {
    let url = "https://api-c.liepin.com/api/com.liepin.csearch.pc.get-valid-expect-info";
    match client.post(url).headers(headers).send().await {
        Ok(x) => match x.json::<LpRes<LpExpectListRes>>().await {
            Ok(x2) => return Some(x2.data?),
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}

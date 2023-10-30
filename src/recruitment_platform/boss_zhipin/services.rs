use reqwest::{header::HeaderMap, Client};
use tracing::{error, info};

use crate::recruitment_platform::EmptyData;
use crate::{
    config::env::EnvConfig,
    recruitment_platform::boss_zhipin::{
        ZpCityListRes, ZpListList, ZpRes, ZpResumeListRes, ZpSayhiTemplateList, ZpSendResume,
        ZpSendResumeRes, ZpToken,
    },
};

use super::{ZpBossInfoRes, ZpExpectDataRes, ZpExpectListRes, ZpListData};

/* 获取猎聘职业列表数据 */
pub async fn get_zp_expect_list(client: &Client, headers: HeaderMap) -> Option<ZpExpectListRes> {
    let url = "https://www.zhipin.com/wapi/zpgeek/recommend/expect/list.json";
    match client.get(url).headers(headers).send().await {
        Ok(x) => match x.json::<ZpRes<ZpExpectListRes>>().await {
            Ok(x2) => return Some(x2.zp_data),
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}

/* 获取BOSS数据 */
pub async fn get_zp_boss_info(
    client: &Client,
    headers: HeaderMap,
    d: &ZpListData,
) -> Option<ZpBossInfoRes> {
    let url = format!(
        "https://www.zhipin.com/wapi/zpchat/geek/getBossData?bossId={}&bossSource={}&securityId={}",
        d.encrypt_boss_id, 0, d.security_id
    );
    match client.get(url).headers(headers).send().await {
        Ok(x) => match x.json::<ZpRes<ZpBossInfoRes>>().await {
            Ok(x2) => return Some(x2.zp_data),
            Err(e) => error!("{}", e),
        },
        Err(e) => error!("{}", e),
    }
    None
}
/* 更新定位城市 */
pub async fn update_zp_location_city(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
) -> Option<()> {
    let list = &get_zp_expect_list(client, headers.clone())
        .await?
        .expect_list[config.encrypt_expect_id];
    let city = if config.city != 0 {
        config.city
    } else {
        list.location
    };
    let url = format!(
        "https://www.zhipin.com/?city={}&ka=city-sites-{}",
        city, city
    );
    client
        .get(url)
        .headers(headers)
        .send()
        .await
        .ok()
        .and_then(|_x| Some(()))
}
/* 获取BOSS直聘列表 */
pub async fn get_zp_list(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
    expects: Vec<ZpExpectDataRes>,
) -> Option<Vec<ZpListData>> {
    let task = |client: Client, config: EnvConfig, headers: HeaderMap, e: ZpExpectDataRes| async move {
        let city = if config.city != 0 {
            config.city
        } else {
            e.location
        };
        let url = if config.template_id == 0 {
            format!("https://www.zhipin.com/wapi/zpgeek/pc/recommend/job/list.json?city={}&experience={}&payType=&partTime=&degree={}&industry=&scale=&jobType=&salary={}encryptExpectId={}&page={}&pageSize={}", city, config.experience, config.degree, config.salary, e.high_salary, config.page, config.page_size)
        } else if config.template_id == 1 {
            format!("https://www.zhipin.com/wapi/zpgeek/recommend/job/list.json?expectId={}&experience={}&degree={}&scale=&stage=&welfare=&salary={}&positionType=&payType=&multiBusinessDistrict=&multiSubway=&page={}&pageSize={}&sortType={}", e.encrypt_id, config.experience, config.degree, config.salary, config.page, config.page_size,config.sort_type)
        } else if config.template_id == 2 {
            format!("https://www.zhipin.com/wapi/zpgeek/search/joblist.json?scene=1&query={}&city={}&experience={}&payType=&partTime=&degree={}&industry=&scale=&stage=&position={}&jobType=&salary={}&multiBusinessDistrict={}&multiSubway=&page={}&pageSize={}", config.search,city,config.experience,config.degree, e.position, config.salary, config.multi_business_district, config.page, config.page_size)
        } else {
            panic!("ARS_TEMPLATE_LIST 变量参数出错");
        };

        match client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .ok()?
            .json::<ZpRes<ZpListList>>()
            .await
        {
            Ok(d) => return Some(d.zp_data),
            Err(e) => error!("{}", e),
        }
        None
    };
    if config.encrypt_expect_id == 0 {
        let mut l = vec![];
        for v in expects {
            if let Some(x) = task(client.clone(), config.clone(), headers.clone(), v).await {
                l.push(x.job_list);
            }
        }
        let d = l.concat();
        return Some(d);
    } else {
        if let Some(x) = task(
            client.clone(),
            config.clone(),
            headers.clone(),
            expects[config.encrypt_expect_id - 1].clone(),
        )
        .await
        {
            return Some(x.job_list);
        }
    }
    None
}
/* BOSS直聘获取令牌 */
pub async fn get_zp_token(client: &Client, headers: HeaderMap) -> Option<ZpToken> {
    let url = "https://www.zhipin.com/wapi/zppassport/get/zpToken";
    client
        .get(url)
        .headers(headers)
        .send()
        .await
        .ok()?
        .json::<ZpRes<ZpToken>>()
        .await
        .and_then(|x| Ok(x.zp_data))
        .ok()
}
/* BOSS直聘打招呼列表模板 */
pub async fn get_zp_sayhi_list(client: &Client, headers: HeaderMap) -> Option<ZpSayhiTemplateList> {
    let url = "https://www.zhipin.com/wapi/zpchat/greeting/getGreetingList";
    client
        .get(url)
        .headers(headers)
        .send()
        .await
        .ok()?
        .json::<ZpRes<ZpSayhiTemplateList>>()
        .await
        .and_then(|r| Ok(r.zp_data))
        .ok()
}
/* BOSS直聘城市列表模板 */
pub async fn get_zp_city_list(client: &Client, headers: HeaderMap) -> Option<ZpCityListRes> {
    let url = "https://www.zhipin.com/wapi/zpCommon/data/city.json";
    client
        .get(url)
        .headers(headers)
        .send()
        .await
        .ok()?
        .json::<ZpRes<ZpCityListRes>>()
        .await
        .and_then(|x| Ok(x.zp_data))
        .ok()
}
/* 获取简历列表 */
pub async fn get_zp_resume_list(client: &Client, headers: HeaderMap) -> Option<ZpResumeListRes> {
    let url = "https://www.zhipin.com/wapi/zpgeek/resume/attachment/checkbox.json";
    match client
        .get(url)
        .headers(headers)
        .send()
        .await
        .ok()?
        .json::<ZpRes<ZpResumeListRes>>()
        .await
    {
        Ok(x) => Some(x.zp_data),
        Err(e) => {
            error!("{}", e);
            None
        }
    }
}
/* 更新打招呼模板 */
pub async fn update_zp_sayhi_template(
    client: &Client,
    config: &EnvConfig,
    headers: HeaderMap,
) -> Option<ZpRes<EmptyData>> {
    let url = "https://www.zhipin.com/wapi/zpchat/greeting/updateGreeting";
    let res = client
        .post(url)
        .headers(headers)
        .multipart(
            reqwest::multipart::Form::new()
                .text("status", "1")
                .text("templateId", config.sayhi_tempalte_id.clone()),
        )
        .send()
        .await
        .ok()?
        .json::<ZpRes<EmptyData>>()
        .await
        .ok()?;
    if res.code != 0 {
        error!("因无法设置模板，请正确配置ARS_SAYHI_TEMPLATE_ID变量");
        return None;
    }
    Some(res)
}
/* 发送简历 */
pub async fn zp_send_resume(
    client: &Client,
    d: ZpSendResume,
    headers: HeaderMap,
) -> Option<ZpSendResumeRes> {
    let url = "https://www.zhipin.com/wapi/zpchat/exchange/request";
    match client
        .post(url)
        .multipart(
            reqwest::multipart::Form::new()
                .text("securityId", d.security_id)
                .text("type", d.r#type)
                .text("encryptResumeId", d.encrypt_resume_id.clone()),
        )
        .headers(headers)
        .send()
        .await
    {
        Ok(x) => {
            match x
                .json::<ZpRes<ZpSendResumeRes>>()
                .await
                .and_then(|x| Ok(x.zp_data))
            {
                Ok(x2) => {
                    info!("成功发送简历[{}]: {:?}", d.encrypt_resume_id, x2);
                    return Some(x2);
                }
                Err(e) => error!("{}", e),
            }
        }
        Err(e) => error!("发送简历[{}]: {:?} 失败", d.encrypt_resume_id, e),
    }
    None
}

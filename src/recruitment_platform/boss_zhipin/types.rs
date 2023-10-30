pub const BOSS_ZHIPIN_TOKEN: &'static str = "zp_token";
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpRes<T> {
    pub code: i32,
    pub message: String,
    #[serde(rename(deserialize = "zpData"))]
    pub zp_data: T,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpListList {
    #[serde(rename(deserialize = "jobList"))]
    pub job_list: Vec<ZpListData>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSayhiTemplateList {
    pub categorys: Vec<ZpSayhiTemplateCategorys>,
    #[serde(rename(deserialize = "greetingTemplateList"))]
    pub greeting_template_id: Vec<ZpSayhiTemplateGreetingList>,
    pub greeting: ZpSayhiTemplateGreeting,
    #[serde(rename(deserialize = "displayButton"))]
    pub display_button: bool,
    pub token: Option<String>,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct ZpSayhiTemplateUpdateData<'a> {
    pub status: u8,
    #[serde(rename(serialize = "templateId"))]
    pub template_id: &'a str,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSayhiTemplateGreetingList {
    identity: usize,
    pub template: String,
    demo: String,
    #[serde(rename(deserialize = "type"))]
    ctype: usize,
    #[serde(rename(deserialize = "addTime"))]
    add_time: usize,
    #[serde(rename(deserialize = "greetingType"))]
    greeting_type: usize,
    category: usize,
    #[serde(rename(deserialize = "encryptId"))]
    pub encrypt_id: String,
    #[serde(rename(deserialize = "templateId"))]
    pub template_id: usize,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSayhiTemplateGreeting {
    id: usize,
    #[serde(rename(deserialize = "userId"))]
    user_id: usize,
    identity: usize,
    status: usize,
    #[serde(rename(deserialize = "customStatus"))]
    custom_status: usize,
    #[serde(rename(deserialize = "addTime"))]
    add_time: usize,
    #[serde(rename(deserialize = "updateTime"))]
    update_time: usize,
    enabled: bool,
    #[serde(rename(deserialize = "templateId"))]
    template_id: usize,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSayhiTemplateCategorys {
    name: String,
    value: i64,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpListData {
    // 安全ID
    #[serde(rename(deserialize = "securityId"))]
    pub security_id: String,
    // BOSS头像
    #[serde(rename(deserialize = "bossAvatar"))]
    pub boss_avatar: String,
    #[serde(rename(deserialize = "encryptBossId"))]
    pub encrypt_boss_id: String,
    // BOSS名称
    #[serde(rename(deserialize = "bossName"))]
    pub boss_name: String,
    // BOSS是否在线
    #[serde(rename(deserialize = "bossOnline"))]
    pub boss_online: bool,
    #[serde(rename(deserialize = "encryptJobId"))]
    pub encrypt_job_id: String,
    #[serde(rename(deserialize = "expectId"))]
    pub expect_id: i64,
    //
    pub lid: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpToken {
    pub token: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSayhi {
    #[serde(rename(deserialize = "showGreeting"))]
    pub show_greeting: bool,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSendResume {
    #[serde(rename(deserialize = "securityId", serialize = "securityId"))]
    pub security_id: String,
    pub r#type: String,
    #[serde(rename(deserialize = "encryptResumeId", serialize = "encryptResumeId"))]
    pub encrypt_resume_id: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpSendResumeRes {
    pub r#type: u8,
    pub status: u8,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpResumeListRes {
    #[serde(rename(deserialize = "resumeCount"))]
    pub resume_count: usize,
    #[serde(rename(deserialize = "resumeList"))]
    pub resume_list: Vec<ZpResumeRes>,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpResumeRes {
    #[serde(rename(deserialize = "resumeId"))]
    pub resume_id: String,
    #[serde(rename(deserialize = "showName"))]
    pub show_name: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpCityListRes {
    #[serde(rename(deserialize = "hotCityList"))]
    pub hot_city_list: Vec<ZpCityRes>,
    #[serde(rename(deserialize = "cityList"))]
    pub city_list: Vec<ZpCityRes>,
    #[serde(rename(deserialize = "locationCity"))]
    pub location_city: ZpCityRes,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpCityRes {
    pub code: usize,
    pub name: String,
    #[serde(rename(deserialize = "subLevelModelList"))]
    pub sub_level_model_list: Option<Vec<ZpCityRes>>,
    #[serde(rename(deserialize = "regionCode"))]
    pub region_code: usize,
    #[serde(rename(deserialize = "cityType"))]
    pub city_type: usize,
    #[serde(rename(deserialize = "cityCode"))]
    pub city_code: Option<String>,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpBossInfoRes {
    pub data: ZpBossInfoDataRes,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpBossInfoDataRes {
    #[serde(rename(deserialize = "companyName"))]
    pub company_name: String,
    #[serde(rename(deserialize = "securityId"))]
    pub security_id: String,
    pub name: String,
    #[serde(rename(deserialize = "hasInterview"))]
    pub has_interview: bool,
    #[serde(rename(deserialize = "encryptBossId"))]
    pub encrypt_boss_id: String,
    #[serde(rename(deserialize = "encryptJobId"))]
    pub encrypt_job_id: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpExpectListRes {
    #[serde(rename(deserialize = "expectList"))]
    pub expect_list: Vec<ZpExpectDataRes>,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ZpExpectDataRes {
    #[serde(rename(deserialize = "encryptId"))]
    pub encrypt_id: String,
    #[serde(rename(deserialize = "highSalary"))]
    pub high_salary: usize,
    pub id: usize,
    pub location: usize,
    #[serde(rename(deserialize = "locationName"))]
    pub location_name: String,
    #[serde(rename(deserialize = "lowSalary"))]
    pub low_salary: usize,
    pub position: usize,
    #[serde(rename(deserialize = "positionName"))]
    pub position_name: String,
    #[serde(rename(deserialize = "positionType"))]
    pub position_type: usize,
    pub salary: String,
    #[serde(rename(deserialize = "subLocation"))]
    pub sub_location: usize,
    #[serde(rename(deserialize = "subLocationName"))]
    pub sub_location_name: Option<String>
}

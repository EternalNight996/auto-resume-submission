use core::fmt;

use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, serde::Serialize, Clone)]
pub struct LpListData2<'a> {
    #[serde(rename(serialize = "existFallbackResult"))]
    pub exist_fallback_result: bool,
    #[serde(rename(serialize = "operateKind"))]
    pub operate_kind: &'a str,
    #[serde(rename(serialize = "selectedExpect"))]
    pub selected_expect: &'a str,
    #[serde(rename(serialize = "sortType"))]
    pub sort_type: &'a str,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpListData<'a> {
    pub data: LpListData2<'a>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpExpectListRes {
    #[serde(rename(deserialize = "validExpects"))]
    pub valid_expects: Vec<LpListDataExpect>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct LpListDataExpect {
    #[serde(rename = "expectJobtitle")]
    pub expect_jobtitle: String,
    #[serde(rename = "expectDq")]
    pub expect_dq: String,
    #[serde(rename = "expectMonthSalaryUpper")]
    pub expect_month_salary_upper: usize,
    #[serde(rename = "expectMonthSalaryLower")]
    pub expect_month_salary_lower: usize,
    #[serde(rename = "expectSalmonths")]
    pub expect_salmonths: usize,
    #[serde(rename = "expectIndustry")]
    pub expect_industry: String,
    #[serde(rename = "expectIndustryName")]
    pub expect_industry_name: String,
    #[serde(rename = "expectJobtitleName")]
    pub expect_jobtitle_name: String,
    #[serde(rename = "expectDqName")]
    pub expect_dq_name: String,
    pub modifytime: String,
    #[serde(rename = "expectId")]
    pub expect_id: usize,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpRes<T> {
    pub data: Option<T>,
    pub flag: usize,
    pub code: Option<String>,
    pub msg: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpDataRes<T> {
    pub data: Vec<T>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpDataJobRes {
    pub comp: LpDataCompRes,
    #[serde(rename(deserialize = "dataInfo"))]
    pub data_info: String,
    pub job: LpDataJob2Res,
    pub recruiter: LpDataJobRecruiterRes,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpDataJobRecruiterRes {
    pub chatted: bool,
    #[serde(rename(deserialize = "imId"))]
    pub im_id: String,
    #[serde(rename(deserialize = "imUserType"))]
    pub im_user_type: String,
    #[serde(rename(deserialize = "recruiterId"))]
    pub recruiter_id: String,
    #[serde(rename(deserialize = "recruiterName"))]
    pub recruiter_name: String,
    #[serde(rename(deserialize = "recruiterPhoto"))]
    pub recruiter_photo: String,
    #[serde(rename(deserialize = "recruiterTitle"))]
    pub recruiter_title: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpDataJob2Res {
    #[serde(rename(deserialize = "jobId"))]
    pub job_id: usize,
    #[serde(rename(deserialize = "jobKind"))]
    pub job_kind: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpDataCompRes {
    // #[serde(rename(deserialize = "compId"))]
    // pub comp_id: usize,
    #[serde(rename(deserialize = "compName"))]
    pub comp_name: String,
    #[serde(rename(deserialize = "fullCompanyName"))]
    pub full_company_name: String,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSayhiData {
    #[serde(rename(serialize = "head_id"))]
    pub head_id: String,
    #[serde(rename(serialize = "ck_id"))]
    pub ck_id: String,
    #[serde(rename(serialize = "jobId"))]
    pub job_id: usize,
    #[serde(rename(serialize = "jobKind"))]
    pub job_kind: String,
    #[serde(rename(serialize = "recruiterId"))]
    pub recruiter_id: String,
    #[serde(rename(serialize = "shieldComp"))]
    pub shield_comp: bool,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpSayhiDataInfo {
    #[serde(rename(deserialize = "jobId"))]
    pub job_id: usize,
    #[serde(rename(deserialize = "as_from"))]
    pub as_from: String,
    #[serde(rename(deserialize = "jobKind"))]
    pub job_kind: String,
    #[serde(rename(deserialize = "d_headId"))]
    pub head_id: String,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendResumeData {
    #[serde(rename(serialize = "imUserType"))]
    pub im_user_type: String,
    #[serde(rename(serialize = "imClientId"))]
    pub im_client_id: String,
    #[serde(rename(serialize = "imId"))]
    pub im_id: String,
    #[serde(rename(serialize = "imApp"))]
    pub im_app: String,
    #[serde(rename(serialize = "oppositeImId"))]
    pub opposite_im_id: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpSendMsgRes {
    #[serde(rename(deserialize = "msgId"))]
    pub msg_id: String,
    #[serde(rename(deserialize = "msgTime"))]
    pub msg_time: usize,
    #[serde(rename(deserialize = "msgToken"))]
    pub msg_token: String,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgData {
    #[serde(rename(serialize = "imUserType"))]
    pub im_user_type: String,
    #[serde(rename(serialize = "imClientId"))]
    pub im_client_id: String,
    #[serde(rename(serialize = "imId"))]
    pub im_id: String,
    #[serde(rename(serialize = "imApp"))]
    pub im_app: String,
    #[serde(rename(serialize = "oppositeImId"))]
    pub opposite_im_id: String,
    #[serde(rename(serialize = "oppositeImUserType"))]
    pub opposite_im_user_type: String,
    #[serde(rename(serialize = "msgTime"))]
    pub msg_time: u128,
    #[serde(rename(serialize = "chatType"))]
    pub chat_type: u8,
    #[serde(rename(serialize = "msgType"))]
    pub msg_type: String,
    pub save: String,
    pub count: usize,
    pub payload: LpSendMsgDataPayload,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayload {
    pub ext: LpSendMsgDataPayloadExt,
    pub bodies: Vec<LpSendMsgDataPayloadBodies>,
    pub push: String,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayloadBodies {
    pub msg: String,
    pub r#type: String,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayloadExt {
    #[serde(rename(serialize = "extType"))]
    pub ext_type: usize,
    #[serde(rename(serialize = "extBody"))]
    pub ext_body: LpSendMsgDataPayloadExtBody,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayloadExtBody {
    #[serde(rename(serialize = "bizType"))]
    pub biz_type: String,
    #[serde(rename(serialize = "bizData"))]
    pub biz_data: LpSendMsgDataPayloadExtBodyBizData,
    #[serde(rename(serialize = "bsData"))]
    pub bs_data: LpSendMsgDataPayloadExtBodyBsData,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayloadExtBodyBizData {
    pub quote: LpSendMsgDataPayloadExtBodyBizDataQuote,
    #[serde(rename(serialize = "senderUserId"))]
    pub sender_user_id: String,
    #[serde(rename(serialize = "receiverUserId"))]
    pub receiver_user_id: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpChatDataRes {
    #[serde(rename(deserialize = "chatType"))]
    pub chat_type: String,
    pub direction: String,
    #[serde(rename(deserialize = "imId"))]
    pub im_id: String,
    #[serde(rename(deserialize = "imUserType"))]
    pub im_user_type: String,
    #[serde(rename(deserialize = "msgId"))]
    pub msg_id: String,
    #[serde(rename(deserialize = "msgTime"))]
    pub msg_time: usize,
    #[serde(rename(deserialize = "msgType"))]
    pub msg_type: String,
    #[serde(rename(deserialize = "oppositeImId"))]
    pub opposite_im_id: String,
    #[serde(rename(deserialize = "oppositeImUserType"))]
    pub opposite_im_user_type: String,
    #[serde(rename(deserialize = "oppositeRead"))]
    pub opposite_read: String,
    #[serde(rename(deserialize = "oppositeUserId"))]
    pub opposite_user_id: String,
    pub payload: String,
    pub readflag: String,
    #[serde(rename(deserialize = "revokeFlag"))]
    pub revoke_flag: bool,
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String
}
#[derive(Debug, serde::Deserialize, Clone)]
pub struct LpContactDataRes<T> {
    // #[serde(rename(deserialize = "curPage"))]
    // pub cur_page: usize,
    // #[serde(rename(deserialize = "hasMore"))]
    // pub has_more: bool,
    pub list: Vec<T>,
    // #[serde(rename(deserialize = "pageSize"))]
    // pub page_size: usize,
    // #[serde(rename(deserialize = "totalCount"))]
    // pub total_count: usize
}
#[derive(Debug, serde::Deserialize, Clone, Default)]
pub struct LpContactListRes {
    // #[serde(rename(deserialize = "hunterType"))]
    // pub hunter_type: String,
    pub title: String,
    pub photo: String,
    #[serde(rename(deserialize = "userTag"))]
    pub user_tag: String,
    pub company: String,
    #[serde(rename(deserialize = "hunterLevel"))]
    pub hunter_level: usize,
    #[serde(rename(deserialize = "homePage"))]
    pub home_page: String,
    pub name: String,
    pub id: String,
    #[serde(rename(deserialize = "imId"))]
    pub im_id: String,
    pub contact: bool,
    pub direction: String,
    #[serde(rename(deserialize = "latestMsgId"))]
    pub latest_msg_id: String,
    #[serde(rename(deserialize = "lastPayload"))]
    pub last_payload: String,
    #[serde(rename(deserialize = "unReadCnt"))]
    pub un_read_cnt: usize,
    #[serde(rename(deserialize = "latestMsgType"))]
    pub latest_msg_type: String,
    pub topuser: bool,
    pub toptime: usize,
    #[serde(rename(deserialize = "oppositeUserId"))]
    pub opposite_user_id: String,
    #[serde(rename(deserialize = "oppositeImId"))]
    pub opposite_im_id: String,
    #[serde(rename(deserialize = "oppositeImUserType"))]
    pub opposite_im_user_type: String,
    #[serde(rename(deserialize = "chatType"))]
    pub chat_type: String,
    #[serde(rename(deserialize = "latestMsgTime"))]
    pub latest_msg_time: usize,
    #[serde(rename(deserialize = "imUserType"))]
    pub im_user_type: String,
    pub quiet: bool,
    #[serde(rename(deserialize = "stickStatus"))]
    pub stick_status: usize,
    #[serde(rename(deserialize = "latestMsgIsRevoke"))]
    pub latest_msg_is_revoke: bool,
    #[serde(rename(deserialize = "userId"))]
    pub user_id: String,
}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayloadExtBodyBizDataQuote {}
#[derive(Debug, serde::Serialize, Clone)]
pub struct LpSendMsgDataPayloadExtBodyBsData {}
#[derive(Clone, Debug)]
pub enum LpSendMsgType {
    Text,
}
impl LpSendMsgType {
    pub fn to_chat_type(&self) -> u8 {
        match self {
            LpSendMsgType::Text => 0,
        }
    }
}
impl<'a> fmt::Display for LpSendMsgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            LpSendMsgType::Text => "txt".to_owned(),
        };
        write!(f, "{}", res)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct LpCookies {
    pub uuid: String,
    pub xsrf_token: String,
    pub gc_id: String,
    pub inited_user: String,
    pub user_roles: String,
    pub user_name: String,
    pub need_bind_tel: String,
    pub new_user: bool,
    pub c_flag: String,
    pub sec_project: String,
    pub im_id: String,
    pub im_id_0: String,
    pub im_client_id: String,
    pub im_client_id_0: String,
    pub im_app_0: String,
    pub user_photo: String,
    pub tlog: String,
    pub myus: String,
    pub fe_im: String,
    pub unique_key: String,
    pub lt_auth: String,
    pub acw_tc: String,
    pub fe_im_openchatwin: String,
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct TopicData {
    pub info_hash: String,
    pub forum_id: u32,
    pub poster_id: u32,
    pub size: u32,
    pub reg_time: u32,
    pub tor_status: u8,
    pub seeders: u16,
    pub topic_title: String,
    pub seeder_last_seen: u32,
    pub dl_count: u32,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub result: T,
}


#[derive(Debug, Deserialize)]
pub struct TopicRequest {
    pub by: String,
    pub val: String,
    pub api_key: String,
}

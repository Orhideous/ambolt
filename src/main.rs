mod models;

use std::collections::HashMap;
use poem::{handler};
use poem::get;
use poem::listener::TcpListener;
use poem::Route;
use poem::Server;
use poem::web::{Json, Query};
use rand::Rng;
use crate::models::{ApiResponse, TopicData, TopicRequest};
use rand::distributions::{Alphanumeric, DistString};


#[handler]
async fn get_tor_topic_data(Query(TopicRequest { by, val, api_key }): Query<TopicRequest>) -> Json<ApiResponse<HashMap<u32, Option<TopicData>>>> {
    let mut rng = rand::thread_rng();
    let ids: Vec<u32> = val.split(",").map(|i| i.parse().unwrap()).collect();
    let mut result: HashMap<u32, Option<TopicData>> = HashMap::with_capacity(ids.len());
    for topic_id in ids {
        let is_empty = rng.gen_ratio(1, 10);
        let topic_data: Option<TopicData> = if is_empty {
            None
        } else {
            Some(TopicData {
                info_hash: Alphanumeric.sample_string(&mut rng, 40),
                forum_id: rng.gen(),
                poster_id: rng.gen(),
                size: rng.gen(),
                reg_time: rng.gen(),
                tor_status: rng.gen_range(0..=11),
                seeders: rng.gen(),
                topic_title: Alphanumeric.sample_string(&mut rng, 255),
                seeder_last_seen: rng.gen(),
                dl_count: rng.gen(),
            })
        };
        result.insert(topic_id, topic_data);
    }
    Json(ApiResponse { result })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/get_tor_topic_data", get(get_tor_topic_data));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

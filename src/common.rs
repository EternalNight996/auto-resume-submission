use std::time::{SystemTime, UNIX_EPOCH};

use base64::encode;
use rand::Rng;

pub fn generate_websocket_key() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    encode(&random_bytes)
}

pub fn now() -> Option<u128> {
    Some(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_millis(),
    )
}

pub fn parse_sayhi(s: &str) -> Vec<String> {
    let mut l = vec![];
    let d: Vec<&str> = s.split("#;").collect();
    if d.len() > 0 {
        for x in d {
            l.push(x.to_string());
        }
    } else {
        l.push(s.to_string());
    }
    l
}

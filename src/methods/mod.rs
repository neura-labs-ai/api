pub mod get;
pub mod post;

use ring::rand::{SecureRandom, SystemRandom};
use serde::Deserialize;

pub fn generate_api_key() -> String {
    let rng = SystemRandom::new();
    let mut api_key = [0u8; 32];
    rng.fill(&mut api_key).unwrap();
    base64::encode(api_key)
}

#[derive(Deserialize)]
pub struct RequestBody<T> {
    pub data: T,
}
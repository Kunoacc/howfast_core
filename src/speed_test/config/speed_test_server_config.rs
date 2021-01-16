use crate::speed_test::config::*;

#[derive(Serialize, Deserialize)]
pub struct SpeedTestServerConfig {
    pub name: String,
    pub address: String,
    pub ping_address: String,
    latency: f32
}
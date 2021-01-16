use crate::speed_test::config::*;
use std::net::Ipv4Addr;
use crate::speed_test::location::distance::Location;

#[derive(Serialize, Deserialize)]
pub struct SpeedTestClientConfig {
    pub ip: Ipv4Addr,
    pub location: Location,
    pub isp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data: String
}

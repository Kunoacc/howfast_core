use crate::speed_test::location::*;

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
    pub location: Option<String>
}
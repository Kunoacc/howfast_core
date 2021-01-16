use crate::speed_test::location::distance::Location;

pub mod distance;

const RADII_OF_EARTH: f32 = 6371.0;

pub fn calculate_distance(start: &Location, end: &Location) -> f32 {
    let distance_lat = (start.latitude - end.latitude).to_radians();
    let distance_long = (start.longitude - end.longitude).to_radians();
    let cosant_of_origin_lat = start.latitude.to_radians().cos();
    let cosant_of_destination_lat = end.latitude.to_radians().cos();
    let sine_of_lat = ((distance_lat / 2.0).sin()).powf(2.0);
    let sine_of_long = ((distance_long / 2.0).sin()).powf(2.0);
    let a = sine_of_lat + sine_of_long * cosant_of_destination_lat * cosant_of_origin_lat;
    let distance = a.sqrt().atan2((1.0 - a).sqrt()) * 2.0;

    return RADII_OF_EARTH * distance;
}
// TODO: generic over both f64 and f32

use std::f64::consts::PI;

/// longitude/latitude to spherical mercator in [0..1] range
pub(crate) fn longitude_to_x(lon: f64) -> f64 {
    lon / 360.0 + 0.5
}

/// longitude/latitude to spherical mercator in [0..1] range
pub(crate) fn latitude_to_y(lat: f64) -> f64 {
    let sin = f64::sin(lat * PI / 180.0);
    let y = 0.5 - 0.25 * f64::ln((1.0 + sin) / (1.0 - sin)) / PI;
    y.clamp(0.0, 1.0)
}

/// spherical mercator to longitude/latitude
pub(crate) fn x_to_longitude(x: f64) -> f64 {
    (x - 0.5) * 360.0
}

/// spherical mercator to longitude/latitude
pub(crate) fn y_to_latitude(y: f64) -> f64 {
    let y2 = (180.0 - y * 360.0) * PI / 180.0;
    360.0 * f64::atan(f64::exp(y2)) / PI - 90.0
}

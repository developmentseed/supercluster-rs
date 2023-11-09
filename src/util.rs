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

// Note: these tests were copied from supercluster-rs under the MIT license
// https://github.com/chargetrip/supercluster-rust/blob/d722a680406c494aedc0c36fae05a10aec854d4d/src/lib.rs#L893-L925
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lng_x() {
        assert_eq!(longitude_to_x(0.0), 0.5);
        assert_eq!(longitude_to_x(180.0), 1.0);
        assert_eq!(longitude_to_x(-180.0), 0.0);
        assert_eq!(longitude_to_x(90.0), 0.75);
        assert_eq!(longitude_to_x(-90.0), 0.25);
    }

    #[test]
    fn test_lat_y() {
        assert_eq!(latitude_to_y(0.0), 0.5);
        assert_eq!(latitude_to_y(90.0), 0.0);
        assert_eq!(latitude_to_y(-90.0), 1.0);
        assert_eq!(latitude_to_y(45.0), 0.35972503691520497);
        assert_eq!(latitude_to_y(-45.0), 0.640274963084795);
    }

    #[test]
    fn test_x_lng() {
        assert_eq!(x_to_longitude(0.5), 0.0);
        assert_eq!(x_to_longitude(1.0), 180.0);
        assert_eq!(x_to_longitude(0.0), -180.0);
        assert_eq!(x_to_longitude(0.75), 90.0);
        assert_eq!(x_to_longitude(0.25), -90.0);
    }

    #[test]
    fn test_y_lat() {
        assert_eq!(y_to_latitude(0.5), 0.0);
        assert_eq!(y_to_latitude(0.875), -79.17133464081944);
        assert_eq!(y_to_latitude(0.125), 79.17133464081945);
    }
}

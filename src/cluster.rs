use crate::util::{latitude_to_y, longitude_to_x};

// encode both zoom and point index on which the cluster originated -- offset by total length of
// features
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClusterId(usize);

// Note: I think like many of mourner's projects, here the ID can have multiple meanings depending
// on whether it's a leaf or an internal node of the tree
//
// Thus we have multiple constructors
impl ClusterId {
    pub fn new(i: usize, zoom: usize, length: usize) -> Self {
        let id = (i << 5) + (zoom + 1) + length;
        Self(id)
    }

    pub fn new_source_id(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(self) -> usize {
        self.0
    }

    /// get index of the point from which the cluster originated
    // Note: I _think_ this doesn't return a ClusterId
    pub(crate) fn get_origin_idx(&self, length: usize) -> usize {
        (self.0 - length) >> 5
    }

    /// get zoom of the point from which the cluster originated
    pub(crate) fn get_origin_zoom(&self, length: usize) -> usize {
        (self.0 - length) % 32
    }
}

#[derive(Debug, Clone)]
pub struct ClusterData {
    /// projected point x
    pub(crate) x: f64,

    /// projected point y
    pub(crate) y: f64,

    /// The last zoom the point was processed at
    pub(crate) zoom: Option<usize>,

    // index of the source feature in the original input array
    pub(crate) source_id: ClusterId,

    // parent cluster id
    pub(crate) parent_id: Option<ClusterId>,

    // number of points in a cluster
    pub(crate) num_points: usize,
}

impl ClusterData {
    /// Create a new object from longitude-latitude x and y values
    pub fn new(lon: f64, lat: f64, source_id: ClusterId) -> Self {
        let x = longitude_to_x(lon);
        let y = latitude_to_y(lat);
        Self::new_projected(x, y, source_id)
    }

    /// Create a new object from spherical mercator x and y values
    pub fn new_projected(x: f64, y: f64, source_id: ClusterId) -> Self {
        Self {
            x,
            y,
            zoom: None,
            source_id,
            parent_id: None,
            num_points: 1,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_origin_idx() {
        let id = ClusterId::new_source_id(100);
        let x = id.get_origin_idx(0);
        dbg!(&x);
    }
}

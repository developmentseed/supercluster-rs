use crate::util::{latitude_to_y, longitude_to_x};

// encode both zoom and point index on which the cluster originated -- offset by total length of
// features
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ClusterId(usize);

impl ClusterId {
    pub fn new(i: usize, zoom: usize, length: usize) -> Self {
        let id = (i << 5) + (zoom + 1) + length;
        Self(id)
    }

    pub fn as_usize(self) -> usize {
        self.0
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

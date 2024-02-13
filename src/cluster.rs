use crate::util::{latitude_to_y, longitude_to_x, x_to_longitude, y_to_latitude};

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

impl From<ClusterId> for usize {
    fn from(value: ClusterId) -> Self {
        value.0
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
    pub fn new_geographic(lon: f64, lat: f64, source_id: ClusterId) -> Self {
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

    /// The x value of this point in Spherical Mercator projection
    pub fn x(&self) -> f64 {
        self.x
    }

    /// The y value of this point in Spherical Mercator projection
    pub fn y(&self) -> f64 {
        self.y
    }
}

/// Information describing a cluster of points.
#[derive(Clone, Debug)]
pub struct ClusterInfo {
    /// If this is a cluster,
    ///
    /// If this is not a cluster, this references the positional index of data added to
    /// Supercluster.
    id: ClusterId,

    /// The longitude of the cluster
    x: f64,

    /// The latitude of the cluster
    y: f64,

    /// If true, references a cluster with containing data. Otherwise, is an original point that
    /// was added to the index.
    cluster: bool,

    /// Note: this will always be 1 if `is_cluster` is false
    point_count: usize,
}

impl From<ClusterInfo> for ClusterId {
    fn from(value: ClusterInfo) -> Self {
        value.id()
    }
}

impl From<&ClusterInfo> for ClusterId {
    fn from(value: &ClusterInfo) -> Self {
        value.id()
    }
}

impl ClusterInfo {
    pub(crate) fn new_cluster(id: ClusterId, x: f64, y: f64, count: usize) -> Self {
        Self {
            id,
            x: x_to_longitude(x),
            y: y_to_latitude(y),
            cluster: true,
            point_count: count,
        }
    }

    /// NOTE: here the x and y are already in the user's own coordinate system (usually lon-lat),
    /// so no need to reproject back.
    pub(crate) fn new_leaf(id: ClusterId, x: f64, y: f64) -> Self {
        Self {
            id,
            x,
            y,
            cluster: false,
            point_count: 1,
        }
    }

    /// If this is a cluster (i.e. [`cluster()`][Self::cluster] is `true`)
    ///
    /// If this is not a cluster (i.e. [`cluster()`][Self::cluster] is `false`), this references
    /// the positional index of data originall added via SuperclusterBuilder.
    pub fn id(&self) -> ClusterId {
        self.id
    }

    /// The longitude of the cluster
    pub fn x(&self) -> f64 {
        self.x
    }

    /// The latitude of the cluster
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Whether this object represents a cluster of containing points or a single input point.
    ///
    /// If true, references a cluster with containing data. Otherwise, is an original point that
    /// was added to the index.
    pub fn is_cluster(&self) -> bool {
        self.cluster
    }

    /// The number of points contained in this cluster
    ///
    /// This will always be 1 if [`is_cluster`][Self::is_cluster] is `false`.
    pub fn count(&self) -> usize {
        self.point_count
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

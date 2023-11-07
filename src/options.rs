/// Options for Supercluster generation
#[derive(Debug, Clone, Copy)]
pub struct SuperclusterOptions {
    /// min zoom to generate clusters on
    pub(crate) min_zoom: usize,

    /// max zoom level to cluster the points on
    pub(crate) max_zoom: usize,

    /// minimum points to form a cluster
    pub(crate) min_points: usize,

    /// cluster radius in pixels
    pub(crate) radius: f64,

    /// tile extent (radius is calculated relative to it)
    pub(crate) extent: f64,

    /// size of the KD-tree leaf node, affects performance
    pub(crate) node_size: usize,
}

impl SuperclusterOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_min_zoom(self, min_zoom: usize) -> Self {
        let mut new = self;
        new.min_zoom = min_zoom;
        new
    }
    pub fn with_max_zoom(self, max_zoom: usize) -> Self {
        let mut new = self;
        new.max_zoom = max_zoom;
        new
    }
    pub fn with_min_points(self, min_points: usize) -> Self {
        let mut new = self;
        new.min_points = min_points;
        new
    }
    pub fn with_radius(self, radius: f64) -> Self {
        let mut new = self;
        new.radius = radius;
        new
    }
    pub fn with_extent(self, extent: f64) -> Self {
        let mut new = self;
        new.extent = extent;
        new
    }
    pub fn with_node_size(self, node_size: usize) -> Self {
        let mut new = self;
        new.node_size = node_size;
        new
    }
}

impl Default for SuperclusterOptions {
    fn default() -> Self {
        Self {
            min_zoom: 0,
            max_zoom: 16,
            min_points: 2,
            radius: 40.0,
            extent: 512.0,
            node_size: 64,
        }
    }
}

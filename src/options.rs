/// Options for Supercluster generation
#[derive(Debug, Clone, Copy)]
pub struct SuperclusterOptions {
    /// Minimum zoom level at which clusters are generated.
    ///
    /// Defaults to `0`.
    pub min_zoom: usize,

    /// Maximum zoom level at which clusters are generated.
    ///
    /// Defaults to `16`.
    pub max_zoom: usize,

    /// Minimum number of points to form a cluster.
    ///
    /// Defaults to `2`.
    pub min_points: usize,

    /// Cluster radius, in pixels.
    ///
    /// Defaults to `40`.
    pub radius: f64,

    /// Tile extent. Radius is calculated relative to this value.
    ///
    /// Defaults to `512`.
    pub extent: f64,

    /// Size of the KD-tree leaf node. Affects performance.
    ///
    /// Defaults to `64`.
    pub node_size: usize,
}

impl SuperclusterOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_min_zoom(self, min_zoom: usize) -> Self {
        SuperclusterOptions { min_zoom, ..self }
    }
    pub fn with_max_zoom(self, max_zoom: usize) -> Self {
        SuperclusterOptions { max_zoom, ..self }
    }
    pub fn with_min_points(self, min_points: usize) -> Self {
        SuperclusterOptions { min_points, ..self }
    }
    pub fn with_radius(self, radius: f64) -> Self {
        SuperclusterOptions { radius, ..self }
    }
    pub fn with_extent(self, extent: f64) -> Self {
        SuperclusterOptions { extent, ..self }
    }
    pub fn with_node_size(self, node_size: usize) -> Self {
        SuperclusterOptions { node_size, ..self }
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

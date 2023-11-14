use pyo3::prelude::*;

/// Options for Supercluster generation
#[derive(FromPyObject, Debug, Clone, Copy)]
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

impl From<SuperclusterOptions> for supercluster_rs::SuperclusterOptions {
    fn from(value: SuperclusterOptions) -> Self {
        supercluster_rs::SuperclusterOptions::new()
            .with_min_zoom(value.min_zoom)
            .with_max_zoom(value.max_zoom)
            .with_min_points(value.min_points)
            .with_radius(value.radius)
            .with_extent(value.extent)
            .with_node_size(value.node_size)
    }
}

use std::collections::HashMap;

use flatbush::kdbush::r#trait::KdbushIndex;

use crate::cluster::{ClusterData, ClusterId};
use crate::options::SuperclusterOptions;
use crate::tree::TreeWithData;
use crate::Supercluster;

pub struct SuperclusterBuilder {
    options: SuperclusterOptions,
    // TODO: in the future, this should be a chunked array of geoarrow points
    points: Vec<(f64, f64)>,
    pos: usize,
    // If points are already in spherical mercator
    // preprojected: bool,
}

impl SuperclusterBuilder {
    pub fn new(num_items: usize) -> Self {
        Self::new_with_options(num_items, Default::default())
    }

    pub fn new_with_options(num_items: usize, options: SuperclusterOptions) -> Self {
        let max_zoom = options.max_zoom;
        let points = Vec::with_capacity(num_items);

        Self {
            options,
            points,
            pos: 0,
        }
    }

    // Add a point to the index
    pub fn add(&mut self, x: f64, y: f64) -> usize {
        let idx = self.pos;
        self.coords.push(x);
        self.coords.push(y);
        self.pos += 1;
        idx
    }

    pub fn finish(self) -> Supercluster {
        assert_eq!(
            self.pos,
            self.points.len(),
            "Expected {} added points, got {}.",
            self.points.len(),
            self.pos
        );

        let min_zoom = self.options.min_zoom;
        let max_zoom = self.options.max_zoom;
        let node_size = self.options.node_size;

        let mut data = Vec::with_capacity(self.points.len());
        for (i, (lon, lat)) in self.points.iter().enumerate() {
            data.push(ClusterData::new(lon, lat, i));
        }

        let mut tree_with_data = TreeWithData::new(data, node_size);
        let mut trees = HashMap::with_capacity(max_zoom - min_zoom + 1);
        trees.insert(max_zoom + 1, tree_with_data);

        for zoom in max_zoom..min_zoom {
            tree_with_data = self.cluster(tree_with_data, zoom);
            trees.insert(zoom, tree_with_data);
        }

        Supercluster::new(self.points, trees, self.options)
    }

    pub fn load(&mut self, x: Vec<f64>, y: Vec<f64>) {
        let min_zoom = self.options.min_zoom;
        let max_zoom = self.options.max_zoom;
    }

    fn cluster(&self, tree_with_data: TreeWithData, zoom: usize) -> Vec<ClusterData> {
        let radius = self.options.radius;
        let extent = self.options.extent;
        let min_points = self.options.min_points;

        let r = radius / (extent * usize::pow(2, zoom.try_into().unwrap()) as f64);
        let cluster_data = tree_with_data.data;
        let tree = tree_with_data.tree;
        let mut next_data = vec![];

        // loop through each point
        for (idx, data) in cluster_data.iter().enumerate() {
            // if we've already visited the point at this zoom level, skip it
            if data.zoom.is_some_and(|z| z <= zoom) {
                continue;
            }

            data.zoom = Some(zoom);

            // find all nearby points
            let x = data.x;
            let y = data.y;
            let neighbor_ids = tree.as_flatbush().within(x, y, r);

            let num_points_origin = data.num_points;
            let mut num_points = num_points_origin;

            // count the number of points in a potential cluster
            for neighbor_id in neighbor_ids {
                // filter out neighbors that are already processed
                if cluster_data[neighbor_id].zoom.is_some_and(|z| z > zoom) {
                    num_points += data.num_points;
                }
            }

            // if there were neighbors to merge, and there are enough points to form a cluster
            if num_points > num_points_origin && num_points >= min_points {
                let wx = x * num_points_origin as f64;
                let wy = y * num_points_origin as f64;

                // encode both zoom and point index on which the cluster originated -- offset by total length of features
                let id = ClusterId::new(idx, zoom, self.points.len());

                for neighbor_id in neighbor_ids {
                    let neighbor_data = cluster_data[neighbor_id];

                    if neighbor_data.zoom.is_some_and(|z| z <= zoom) {
                        continue;
                    }
                    // save the zoom (so it doesn't get processed twice)
                    neighbor_data.zoom = Some(zoom);

                    let num_points2 = neighbor_data.num_points as f64;
                    // accumulate coordinates for calculating weighted center
                    wx += neighbor_data.x * num_points2;
                    wy += neighbor_data.y * num_points2;

                    neighbor_data.parent_id = Some(id);
                }

                cluster_data[idx].parent_id = Some(id);
                next_data.push(ClusterData {
                    x: wx / num_points as f64,
                    y: wy / num_points as f64,
                    zoom: None,
                    source_id: id,
                    parent_id: None,
                    num_points,
                });
            } else {
                // left points as unclustered
                todo!()
            }
        }

        next_data
    }
}

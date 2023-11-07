use flatbush::kdbush::r#trait::KdbushIndex;

use crate::cluster::{ClusterData, ClusterId};
use crate::options::SuperclusterOptions;
use crate::tree::TreeWithData;

pub struct SuperclusterBuilder {
    options: SuperclusterOptions,
}

impl SuperclusterBuilder {
    pub fn new() -> Self {
        Self::new_with_options(Default::default())
    }

    pub fn new_with_options(options: SuperclusterOptions) -> Self {
        let max_zoom = options.max_zoom;

        // TODO: don't create trees for lower zooms if unused
        Self {
            options,
            trees: Vec::with_capacity(max_zoom + 1),
        }
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
                let id = ClusterId::new(idx, zoom, todo!());
                // self.points.len()

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

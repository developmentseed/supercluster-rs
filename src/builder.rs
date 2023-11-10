use std::collections::HashMap;

use flatbush::kdbush::r#trait::KdbushIndex;

use crate::cluster::{ClusterData, ClusterId};
use crate::options::SuperclusterOptions;
use crate::tree::TreeWithData;
use crate::Supercluster;

/// A data class used to construct a [Supercluster] instance.
pub struct SuperclusterBuilder {
    options: SuperclusterOptions,
    // TODO: in the future, this should be a chunked array of geoarrow points
    points: Vec<(f64, f64)>,
    pos: usize,
    // If points are already in spherical mercator
    // preprojected: bool,
}

impl SuperclusterBuilder {
    /// Construct a new [SuperclusterBuilder] with the given number of points and default options.
    pub fn new(num_items: usize) -> Self {
        Self::new_with_options(num_items, Default::default())
    }

    /// Construct a new [SuperclusterBuilder] with the given number of points and default options.
    pub fn new_with_options(num_items: usize, options: SuperclusterOptions) -> Self {
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
        self.points.push((x, y));
        self.pos += 1;
        idx
    }

    /// Convert a [SuperclusterBuilder] to a [Supercluster] by running hierarchical clustering.
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
            data.push(ClusterData::new(*lon, *lat, ClusterId::new_source_id(i)));
        }

        let full_res_tree = TreeWithData::new(data, node_size);

        let mut trees = HashMap::with_capacity(max_zoom - min_zoom + 1);
        trees.insert(max_zoom + 1, full_res_tree);

        for zoom in (min_zoom..=max_zoom).rev() {
            // The tree at the next higher zoom
            let previous_tree = trees.get_mut(&(zoom + 1)).unwrap();
            let current = self.cluster(previous_tree, zoom);

            trees.insert(zoom, current);
        }

        Supercluster::new(self.points, trees, self.options)
    }

    /// Note: this mutates previous_tree's `data`.
    // This is derived from Supercluster._cluster in the original JS implementation
    fn cluster(&self, previous_tree_with_data: &mut TreeWithData, zoom: usize) -> TreeWithData {
        let radius = self.options.radius;
        let extent = self.options.extent;
        let min_points = self.options.min_points;

        let r = radius / (extent * usize::pow(2, zoom.try_into().unwrap()) as f64);
        let cluster_data = &mut previous_tree_with_data.data;
        let previous_tree = &previous_tree_with_data.tree;
        let mut next_data = vec![];

        // loop through each point
        //  (idx, data) in cluster_data.iter().enumerate()
        for idx in 0..cluster_data.len() {
            let data = &cluster_data[idx];

            // if we've already visited the point at this zoom level, skip it
            if data.zoom.is_some_and(|z| z <= zoom) {
                continue;
            }

            // We clone this data instance to mutate it in isolation, and then set it back on the
            // cluster_data vec at the end
            let mut data = data.clone();

            data.zoom = Some(zoom);

            // find all nearby points
            let x = data.x;
            let y = data.y;
            let neighbor_ids = previous_tree.as_kdbush().within(x, y, r);

            let num_points_origin = data.num_points;
            let mut num_points = num_points_origin;

            // count the number of points in a potential cluster
            for neighbor_id in neighbor_ids.iter() {
                // filter out neighbors that are already processed
                let neighbor_data = &cluster_data[*neighbor_id];
                if neighbor_data.zoom.is_some_and(|z| z > zoom) {
                    num_points += data.num_points;
                }
            }

            // if there were neighbors to merge, and there are enough points to form a cluster
            if num_points > num_points_origin && num_points >= min_points {
                let mut wx = x * num_points_origin as f64;
                let mut wy = y * num_points_origin as f64;

                // encode both zoom and point index on which the cluster originated -- offset by total length of features
                let id = ClusterId::new(idx, zoom, self.points.len());

                for neighbor_id in neighbor_ids {
                    let neighbor_data = &cluster_data[neighbor_id];

                    if neighbor_data.zoom.is_some_and(|z| z <= zoom) {
                        continue;
                    }

                    // TODO: change this section to just create a new ClusterData object manually
                    // instead of cloning and then setting fields

                    // Clone this value to mutate it
                    let mut neighbor_data = neighbor_data.clone();

                    // save the zoom (so it doesn't get processed twice)
                    neighbor_data.zoom = Some(zoom);

                    let num_points2 = neighbor_data.num_points as f64;
                    // accumulate coordinates for calculating weighted center
                    wx += neighbor_data.x * num_points2;
                    wy += neighbor_data.y * num_points2;

                    neighbor_data.parent_id = Some(id);

                    // Re-set changed onto array
                    cluster_data[neighbor_id] = neighbor_data;
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
                // TODO: double check I'm adding the right thing
                next_data.push(data.clone());

                if num_points > 1 {
                    for neighbor_id in neighbor_ids.iter() {
                        let neighbor_data = &cluster_data[*neighbor_id];

                        if neighbor_data.zoom.is_some_and(|z| z <= zoom) {
                            continue;
                        }

                        let mut neighbor_data = neighbor_data.clone();
                        neighbor_data.zoom = Some(zoom);

                        // Need to set back onto cluster_data to emulate changing the `data` array
                        // directly like JS does
                        cluster_data[*neighbor_id] = neighbor_data.clone();
                        next_data.push(neighbor_data);
                    }
                }
            }

            // Need to set back onto cluster_data to emulate changing the `data` array
            // directly like JS does
            cluster_data[idx] = data;
        }

        TreeWithData::new(next_data, self.options.node_size)
    }
}

#[cfg(test)]
mod test {
    use crate::test::load_fixture::load_places;

    use super::*;

    #[test]
    fn test_builder() {
        let coords = load_places();
        let mut builder = SuperclusterBuilder::new(coords.len());
        for coord in coords {
            builder.add(coord[0], coord[1]);
        }
        let _supercluster = builder.finish();
        // dbg!(supercluster);
    }
}

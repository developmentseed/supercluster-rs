use flatbush::{KdbushBuilder, OwnedKdbush};
// TODO: fix export
use flatbush::kdbush::r#trait::KdbushIndex;

use crate::cluster::ClusterId;
use crate::options::SuperclusterOptions;
use crate::tree::TreeWithData;

#[derive(Debug, Clone)]
pub struct Supercluster {
    options: SuperclusterOptions,

    /// Vector of KDBush structures for different zoom levels
    // TODO: switch to Vec<Option<TreeWithData>>,
    trees: Vec<TreeWithData>,
}

impl Supercluster {
    /// Returns a vec with the cluster ids
    pub fn get_clusters(
        &self,
        min_lng: f64,
        min_lat: f64,
        max_lng: f64,
        max_lat: f64,
        zoom: usize,
    ) -> Vec<ClusterId> {
        let mut min_lng = ((min_lng + 180.0) % 360.0 + 360.0) % 360.0 - 180.0;
        let min_lat = min_lat.clamp(-90.0, 90.0);
        let mut max_lng = if max_lng == 180.0 {
            180.0
        } else {
            ((max_lng + 180.0) % 360.0 + 360.0) % 360.0 - 180.0
        };
        let max_lat = max_lat.clamp(-90.0, 90.0);

        if max_lng - min_lng >= 360.0 {
            min_lng = -180.0;
            max_lng = 180.0;
        } else if min_lng > max_lng {
            let mut eastern_hem = self.get_clusters(min_lng, min_lat, 180.0, max_lat, zoom);
            let mut western_hem = self.get_clusters(-180.0, min_lat, max_lng, max_lat, zoom);
            eastern_hem.append(&mut western_hem);
            return eastern_hem;
        }

        let tree_with_data = self.trees[self.clamp_zoom(zoom)];
        let ids = tree_with_data
            .tree
            .as_flatbush()
            .range(min_lng, min_lat, max_lng, max_lat);
        let data = tree_with_data.data;

        let mut clusters = Vec::with_capacity(ids.len());
        for id in ids {
            let cluster_data = data[id];
            let num_points = cluster_data.num_points;
            // If there's more than one point in this cluster, group them.
            if num_points > 1 {
                let tmp: Vec<ClusterId> = todo!();
                clusters.extend(tmp);
            } else {
                let cluster_id = cluster_data.source_id;
                clusters.push(cluster_id);
            }
        }

        clusters
    }

    pub fn get_children(&self, cluster_id: ClusterId) -> Vec<usize> {
        let origin_id = cluster_id.get_origin_idx(self.points.length);
        let origin_zoom = cluster_id.get_origin_zoom(self.points.length);

        let tree_with_data = self.trees[origin_zoom];
        // assert!(tree_with_data);

        let cluster_data = tree_with_data.data;
        let tree = tree_with_data.tree;
        // if (origin_id * this.stride >= data.length) throw new Error(errorMsg);

        let r = self.options.radius
            / (self.options.extent * usize::pow(2, (origin_zoom - 1).try_into().unwrap()) as f64);
        let x = cluster_data[origin_id].x;
        let y = cluster_data[origin_id].y;
        let ids = tree.as_flatbush().within(x, y, r);
        let mut children = vec![];

        for id in ids {
            let child_data = cluster_data[id];
            if child_data
                .parent_id
                .is_some_and(|parent_id| parent_id == cluster_id)
            {
                if child_data.num_points > 1 {
                    todo!()
                } else {
                    children.push(self.points[child_data.source_id.as_usize()]);
                }
            }
        }

        assert!(children.len() > 0);

        return children;
    }

    pub fn get_leaves(
        &self,
        cluster_id: ClusterId,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Vec<_> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);

        let mut leaves = vec![];

        leaves
    }

    pub fn get_tile(self, z: usize, x: usize, y: usize) {
        let tree = self.trees[self.clamp_zoom(z)];
        let z2 = usize::pow(2, z.try_into().unwrap());
        let p = self.options.radius / self.options.extent;
        // let top = (y - p) / z2;
        // let bottom = (y + 1 + p) / z2;

        todo!()
    }

    pub fn get_cluster_expansion_zoom(&self, cluster_id: ClusterId) -> usize {
        let mut expansion_zoom = cluster_id.get_origin_zoom(self.points.length) - 1;
        while expansion_zoom <= self.options.max_zoom {
            let children = self.get_children(cluster_id);
            expansion_zoom += 1;
            if children.len() != 1 {
                break;
            }
            cluster_id = children[0].properties.cluster_id;
        }
        return expansion_zoom;

    }


    fn append_leaves(
        &self,
        cluster_id: ClusterId,
        limit: usize,
        offset: usize,
        skipped: usize,
    ) -> Vec<usize> {
        todo!()
    }

    fn get_cluster_json(&self) {}

    fn clamp_zoom(&self, z: usize) -> usize {
        z.clamp(self.options.min_zoom, self.options.max_zoom + 1)
    }
}

// impl Default for Supercluster {
//     fn default() -> Self {
//         Self::new()
//     }
// }

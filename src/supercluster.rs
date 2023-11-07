use flatbush::{KdbushBuilder, OwnedKdbush};
// TODO: fix export
use flatbush::kdbush::r#trait::KdbushIndex;

use crate::options::SuperclusterOptions;
use crate::tree::TreeWithData;

#[derive(Debug, Clone)]
pub struct Supercluster {
    options: SuperclusterOptions,

    /// Vector of KDBush structures for different zoom levels
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
    ) -> Vec<usize> {
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
                let tmp: Vec<usize> = self.get_cluster_json();
                clusters.extend(tmp);
            } else {
                let cluster_id = cluster_data.source_idx;
                clusters.push(cluster_id);
            }
        }

        return clusters;
    }

    pub fn get_children(&self, cluster_id: usize) -> Vec<usize> {}

    pub fn get_tile(self, z: usize, x: usize, y: usize) {
        let tree = self.trees[self.clamp_zoom(z)];
        let z2 = usize::pow(2, z.try_into().unwrap());
        let p = self.options.radius / self.options.extent;
        // let top = (y - p) / z2;
        // let bottom = (y + 1 + p) / z2;

        todo!()
    }

    // get index of the point from which the cluster originated
    fn get_origin_id(&self, cluster_id: usize) -> usize {
        todo!()
        // return (clusterId - this.points.length) >> 5;
    }

    // get zoom of the point from which the cluster originated
    fn get_origin_zoom(&self, cluster_id: usize) -> usize {
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

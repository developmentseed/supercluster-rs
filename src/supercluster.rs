use std::collections::HashMap;

use flatbush::kdbush::KdbushIndex;

use crate::cluster::{ClusterId, ClusterInfo};
use crate::error::SuperclusterError;
use crate::options::SuperclusterOptions;
use crate::tree::TreeWithData;
use crate::util::{latitude_to_y, longitude_to_x};

#[derive(Debug, Clone)]
pub struct Supercluster {
    options: SuperclusterOptions,

    /// Vector of KDBush structures for different zoom levels
    trees: HashMap<usize, TreeWithData>,

    /// Note: these points are in the user's original coordinate system (usually lon-lat).
    points: Vec<(f64, f64)>,
}

impl Supercluster {
    pub(crate) fn new(
        points: Vec<(f64, f64)>,
        trees: HashMap<usize, TreeWithData>,
        options: SuperclusterOptions,
    ) -> Self {
        Self {
            options,
            trees,
            points,
        }
    }

    /// Returns a vec with the cluster ids
    pub fn get_clusters(
        &self,
        min_lng: f64,
        min_lat: f64,
        max_lng: f64,
        max_lat: f64,
        zoom: usize,
    ) -> Vec<ClusterInfo> {
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

        let tree_with_data = self.trees.get(&self.clamp_zoom(zoom)).unwrap();

        // NOTE! it is intentional for max_lat to be passed to min_y and for min_lat to be passed
        // to max_y. Apparently the spherical mercator coord system has a flipped y.
        let ids = tree_with_data.tree.as_kdbush().range(
            longitude_to_x(min_lng),
            latitude_to_y(max_lat),
            longitude_to_x(max_lng),
            latitude_to_y(min_lat),
        );

        let data = tree_with_data.data();

        let mut clusters = Vec::with_capacity(ids.len());
        for id in ids {
            let cluster_data = &data[id];

            // If there's more than one point in this cluster, group them.
            if cluster_data.num_points > 1 {
                clusters.push(ClusterInfo::new_cluster(
                    cluster_data.source_id,
                    cluster_data.x,
                    cluster_data.y,
                    cluster_data.num_points,
                ));
            } else {
                let (x, y) = self.points[id];
                clusters.push(ClusterInfo::new_leaf(cluster_data.source_id, x, y))
            }
        }

        clusters
    }

    pub fn get_children(
        &self,
        cluster_id: ClusterId,
    ) -> Result<Vec<ClusterInfo>, SuperclusterError> {
        let origin_id = self.get_origin_idx(cluster_id);
        let origin_zoom = self.get_origin_zoom(cluster_id);

        let tree_with_data = match self.trees.get(&origin_zoom) {
            Some(tree_with_data) => tree_with_data,
            None => return Err(SuperclusterError::NoClusterFound),
        };

        let data = tree_with_data.data();
        let tree = tree_with_data.tree();
        if origin_id >= data.len() {
            return Err(SuperclusterError::NoClusterFound);
        }

        let r = self.options.radius
            / (self.options.extent * usize::pow(2, (origin_zoom - 1).try_into().unwrap()) as f64);
        let x = data[origin_id].x;
        let y = data[origin_id].y;
        let ids = tree.as_kdbush().within(x, y, r);
        let mut children = vec![];

        for id in ids {
            let cluster_data = &data[id];

            if cluster_data
                .parent_id
                .is_some_and(|parent_id| parent_id == cluster_id)
            {
                if cluster_data.num_points > 1 {
                    children.push(ClusterInfo::new_cluster(
                        cluster_data.source_id,
                        cluster_data.x,
                        cluster_data.y,
                        cluster_data.num_points,
                    ));
                } else {
                    let (x, y) = self.points[id];
                    children.push(ClusterInfo::new_leaf(cluster_data.source_id, x, y))
                }
            }
        }

        if children.is_empty() {
            return Err(SuperclusterError::NoClusterFound);
        }

        Ok(children)
    }

    pub fn get_leaves(
        &self,
        cluster_id: ClusterId,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<ClusterInfo>, SuperclusterError> {
        let limit = limit.unwrap_or(10);
        let offset = offset.unwrap_or(0);

        let mut leaves = vec![];
        self.append_leaves(&mut leaves, cluster_id, limit, offset, 0)?;

        Ok(leaves)
    }

    // pub fn get_tile(self, z: usize, x: usize, y: usize) {
    //     let tree = self.trees.get(&self.clamp_zoom(z)).unwrap();
    //     let z2 = usize::pow(2, z.try_into().unwrap()) as f64;
    //     let p = self.options.radius / self.options.extent;
    //     let top = (y as f64 - p) / z2;
    //     let bottom = (y as f64 + 1.0 + p) / z2;

    //     todo!()
    // }

    /// Returns the zoom on which the cluster expands into several children (useful for "click to
    /// zoom" feature) given the cluster's cluster_id.
    pub fn get_cluster_expansion_zoom(
        &self,
        cluster_id: ClusterId,
    ) -> Result<usize, SuperclusterError> {
        let mut cluster_id = cluster_id;
        let mut expansion_zoom = cluster_id.get_origin_zoom(self.points.len()) - 1;
        while expansion_zoom <= self.options.max_zoom {
            let children = self.get_children(cluster_id)?;
            expansion_zoom += 1;
            if children.len() != 1 {
                break;
            }
            cluster_id = children[0].id();
        }

        Ok(expansion_zoom)
    }

    fn append_leaves(
        &self,
        result: &mut Vec<ClusterInfo>,
        cluster_id: ClusterId,
        limit: usize,
        offset: usize,
        skipped: usize,
    ) -> Result<usize, SuperclusterError> {
        let children = self.get_children(cluster_id)?;

        let mut skipped = skipped;

        for child in children {
            if child.cluster() {
                if skipped + child.count() <= offset {
                    // skip the whole cluster
                    skipped += child.count();
                } else {
                    // enter the cluster
                    skipped = self.append_leaves(result, child.id(), limit, offset, skipped)?;
                    // exit the cluster
                }
                skipped += 1;
            } else if skipped < offset {
                // skip a single point
                skipped += 1;
            } else {
                // add a single point
                result.push(child);
            }

            if result.len() == limit {
                break;
            }
        }

        Ok(skipped)
    }

    fn clamp_zoom(&self, zoom: usize) -> usize {
        zoom.clamp(self.options.min_zoom, self.options.max_zoom + 1)
    }

    /// get index of the point from which the cluster originated
    fn get_origin_idx(&self, cluster_id: ClusterId) -> usize {
        cluster_id.get_origin_idx(self.points.len())
    }

    /// get zoom of the point from which the cluster originated
    fn get_origin_zoom(&self, cluster_id: ClusterId) -> usize {
        cluster_id.get_origin_zoom(self.points.len())
    }
}

#[cfg(test)]
mod test {
    use crate::test::load_fixture::load_places;
    use crate::SuperclusterBuilder;

    #[test]
    fn test_builder() {
        let coords = load_places();
        let mut builder = SuperclusterBuilder::new(coords.len());
        for coord in coords {
            builder.add(coord[0], coord[1]);
        }
        let supercluster = builder.finish();
        let clusters = supercluster.get_clusters(-50., -50., 50., 50., 0);
        dbg!(clusters.len());
        dbg!(&clusters);
        // dbg!(supercluster);
    }
}

use flatbush::{KdbushBuilder, OwnedKdbush};

use crate::cluster::ClusterData;

#[derive(Debug, Clone)]
pub struct TreeWithData {
    pub(crate) tree: OwnedKdbush,
    pub(crate) data: Vec<ClusterData>,
}

impl TreeWithData {
    // This is akin to Supercluster._createTree in the original implementation
    pub fn new(data: Vec<ClusterData>, node_size: usize) -> Self {
        let mut tree_builder = KdbushBuilder::new_with_node_size(data.len(), node_size);
        for item in data.iter() {
            tree_builder.add(item.x(), item.y());
        }
        let tree = tree_builder.finish();
        Self { tree, data }
    }

    pub(crate) fn data(&self) -> &[ClusterData] {
        &self.data
    }

    pub(crate) fn tree(&self) -> &OwnedKdbush {
        &self.tree
    }
}

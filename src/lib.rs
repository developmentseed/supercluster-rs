#![doc = include_str!("../README.md")]

mod builder;
mod cluster;
pub mod error;
mod options;
mod statistics;
mod supercluster;
mod tree;
pub(crate) mod util;

pub use builder::SuperclusterBuilder;
pub use cluster::{ClusterData, ClusterId, ClusterInfo};
pub use options::SuperclusterOptions;
pub use supercluster::Supercluster;

#[cfg(test)]
pub(crate) mod test;

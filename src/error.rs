use std::fmt::Debug;
use thiserror::Error;

/// Enum with all errors in this crate.
#[derive(Error, Debug)]
pub enum SuperclusterError {
    #[error("No cluster with the specified id.")]
    NoClusterFound,
}

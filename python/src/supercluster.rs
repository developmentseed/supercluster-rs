use std::sync::Arc;

use arrow::array::{Array, BooleanBuilder, Float64Builder, UInt32Builder, UInt64Builder};
use arrow::datatypes::DataType;
use arrow::datatypes::{Field, Schema};
use arrow::pyarrow::PyArrowType;
use arrow::record_batch::RecordBatch;
use pyo3::prelude::*;
use supercluster_rs::cluster::ClusterInfo;
use supercluster_rs::Supercluster as _Supercluster;

#[pyclass]
pub struct Supercluster(pub(crate) _Supercluster);

#[pymethods]
impl Supercluster {
    fn get_clusters(
        &self,
        min_lng: f64,
        min_lat: f64,
        max_lng: f64,
        max_lat: f64,
        zoom: usize,
    ) -> PyResult<PyArrowType<RecordBatch>> {
        let clusters = self
            .0
            .get_clusters(min_lng, min_lat, max_lng, max_lat, zoom);
        Ok(PyArrowType(clusters_to_record_batch(clusters)))
    }
}

fn clusters_to_record_batch(clusters: Vec<ClusterInfo>) -> RecordBatch {
    let mut id_arr = UInt64Builder::with_capacity(clusters.len());
    let mut x_arr = Float64Builder::with_capacity(clusters.len());
    let mut y_arr = Float64Builder::with_capacity(clusters.len());
    let mut is_cluster_arr = BooleanBuilder::with_capacity(clusters.len());
    let mut point_count_arr = UInt32Builder::with_capacity(clusters.len());

    for cluster in clusters {
        id_arr.append_value(cluster.id().as_usize().try_into().unwrap());
        x_arr.append_value(cluster.x());
        y_arr.append_value(cluster.y());
        is_cluster_arr.append_value(cluster.cluster());
        point_count_arr.append_value(cluster.count().try_into().unwrap());
    }

    let id_arr = id_arr.finish();
    let x_arr = x_arr.finish();
    let y_arr = y_arr.finish();
    let is_cluster_arr = is_cluster_arr.finish();
    let point_count_arr = point_count_arr.finish();

    let arrays: Vec<Arc<dyn Array>> = vec![
        Arc::new(id_arr),
        Arc::new(x_arr),
        Arc::new(y_arr),
        Arc::new(is_cluster_arr),
        Arc::new(point_count_arr),
    ];

    let fields = vec![
        Field::new("id", DataType::UInt64, false),
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
        Field::new("is_cluster", DataType::Boolean, false),
        Field::new("count", DataType::UInt32, false),
    ];

    let schema = Schema::new(fields);

    RecordBatch::try_new(schema.into(), arrays).unwrap()
}

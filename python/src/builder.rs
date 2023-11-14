use arrow::array::{make_array, Array, ArrayData, FixedSizeListArray, Float64Array};
use arrow::datatypes::DataType;
use arrow::pyarrow::PyArrowType;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use supercluster_rs::SuperclusterBuilder as _SuperclusterBuilder;

use crate::options::SuperclusterOptions;
use crate::supercluster::Supercluster;

#[pyclass]
pub struct SuperclusterBuilder(pub(crate) _SuperclusterBuilder);

#[pymethods]
impl SuperclusterBuilder {
    #[new]
    fn new(num_items: usize, options: Option<SuperclusterOptions>) -> Self {
        Self(_SuperclusterBuilder::new_with_options(
            num_items,
            options
                .map(|py_options| py_options.into())
                .unwrap_or_default(),
        ))
    }

    fn add(&mut self, x: f64, y: f64) -> usize {
        self.0.add(x, y)
    }

    // Note: we can't force the consumption of the builder in `finish` (i.e. we can't take self as
    // value) because its memory is tied to python separately. We'd have to use `&self` and wait
    // for python to garbage collect the builder's memory
}

#[pyfunction]
pub fn create_index(
    array: PyArrowType<ArrayData>,
    options: Option<SuperclusterOptions>,
) -> PyResult<Supercluster> {
    let array = make_array(array.0);

    if array.len() > usize::pow(2, 32) - 1 {
        return Err(PyTypeError::new_err(
            "Greater than 2^32 elements not yet supported",
        ));
    }

    let array = match array.data_type() {
        DataType::FixedSizeList(_field, list_size) => {
            if *list_size != 2 {
                return Err(PyValueError::new_err(
                    "Expected fixed size list to have size of 2.",
                ));
            }
            array.as_any().downcast_ref::<FixedSizeListArray>().unwrap()
        }
        _ => {
            return Err(PyTypeError::new_err(
                "Expected an array of data type FixedSizeList",
            ))
        }
    };

    if array.null_count() > 0 {
        return Err(PyValueError::new_err(
            "Null list items in the array are not yet supported",
        ));
    }

    let values_array = array.values();
    let values = match values_array.data_type() {
        DataType::Float64 => values_array
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap(),
        _ => {
            return Err(PyTypeError::new_err(
                "Expected inner list to be of type float64",
            ))
        }
    };

    let mut builder = _SuperclusterBuilder::new_with_options(
        array.len(),
        options
            .map(|py_options| py_options.into())
            .unwrap_or_default(),
    );

    for point in values.values().chunks_exact(2) {
        builder.add(point[0], point[1]);
    }

    Ok(Supercluster(builder.finish()))
}

use pyo3::prelude::*;

mod builder;
mod options;
mod supercluster;

/// A Python module implemented in Rust.
#[pymodule]
fn _rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<supercluster::Supercluster>()?;

    m.add_function(wrap_pyfunction!(builder::create_index, m)?)?;
    Ok(())
}

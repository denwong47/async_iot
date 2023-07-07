use pyo3::prelude::*;

pub use async_iot_models as models;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn func(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_async_iot(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(func, m)?)?;
    Ok(())
}

extern crate sorted as st;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule]
fn sorted(py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}

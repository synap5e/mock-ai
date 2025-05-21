mod cli;
mod models;
mod openai;
mod server;

use pyo3::prelude::*;
use tokio::runtime::Runtime;

#[pyfunction]
fn main() -> PyResult<()> {
    Runtime::new()
        .expect("Start tokio runtime")
        .block_on(server::run())
        .map_err(|e| e.into())
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}

	
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
#[pyo3(signature = (cache_file_path=None))]
fn get_gpu_node_id(cache_file_path: Option<String>) -> PyResult<String> {
    let optional_path: Option<PathBuf> = cache_file_path.map(PathBuf::from);
    let optional_path_ref: Option<&PathBuf> = optional_path.as_ref();

    crate::get_gpu_node_id(optional_path_ref)
        .map_err(|err | pyo3::exceptions::PyRuntimeError::new_err(err.to_string()))
}

#[pymodule]
pub fn gni_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_gpu_node_id, m)?)
}

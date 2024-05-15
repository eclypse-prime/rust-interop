use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

// C part

#[no_mangle]
pub extern "C" fn rs_print() {
    println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn rs_add(a: i32, b: i32) -> i32
{
    a + b
}

// Python part
// Functions need to be prefixed with #[pyfunction], and encapsulate the result in PyResult

#[pyfunction]
fn py_print() {
    println!("Goodbye world...");
}

#[pyfunction]
fn py_add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pyfunction]
fn py_fail(b: bool) -> PyResult<()> {
    if b {
        Err(PyValueError::new_err("rust method error"))
    } else {
        Ok(())
    }
}

// Each function must explicitly be added here. The name of the function is the name of the module
#[pymodule]
fn rust_interop(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_print, m)?)?;
    m.add_function(wrap_pyfunction!(py_fail, m)?)?;
    m.add_function(wrap_pyfunction!(py_add, m)?)?;
    Ok(())
}
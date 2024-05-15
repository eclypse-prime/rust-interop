use pyo3::prelude::*;

#[no_mangle]
pub extern "C" fn rs_print() {
    println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn rs_add(a: i32, b: i32) -> i32
{
    a + b
}

#[pyfunction]
fn py_print() {
    println!("Goodbye world...");
}

#[pyfunction]
fn py_add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pymodule]
fn rust_interop(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_print, m)?)?;
    m.add_function(wrap_pyfunction!(py_add, m)?)?;
    Ok(())
}
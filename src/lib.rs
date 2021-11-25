
extern crate tera;
use pyo3::exceptions;
use tera::Context;
use tera::Tera;
use std::collections::HashMap;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn one_off(template: String, context_dict: HashMap<String, String>, autoescape: bool) -> PyResult<String> {
    let mut context = Context::new();
    for (key, value) in context_dict {
        context.insert(key, &value);
    }
    return match Tera::one_off(&template, &context, autoescape) {
        Ok(t) => Ok(t),
        Err(e) => Err(exceptions::PyValueError::new_err(e.to_string())),
    }
}

#[pymodule]
fn pytera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(one_off, m)?)?;

    Ok(())
}

#[pyfunction]
fn new(dir: String) -> PyResult<TeraInstance> {
    return match Tera::new(dir) {
        Ok(t) => Ok(TeraInstance{
            instance: t
        }),
        Err(e) => Err(exceptions::PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
struct TeraInstance {
   instance: Tera,
}

// #[pymethods]
// impl Tera {

// }
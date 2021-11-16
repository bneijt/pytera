
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

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pytera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(one_off, m)?)?;

    Ok(())
}

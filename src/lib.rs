extern crate tera;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::collections::HashMap;
use tera::Context;
use tera::Tera;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn one_off(
    template: String,
    context_dict: HashMap<String, String>,
    autoescape: bool,
) -> PyResult<String> {
    let mut context = Context::new();
    for (key, value) in context_dict {
        context.insert(key, &value);
    }
    return match Tera::one_off(&template, &context, autoescape) {
        Ok(t) => Ok(t),
        Err(e) => Err(exceptions::PyValueError::new_err(e.to_string())),
    };
}

#[pymodule]
fn pytera(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(one_off, m)?)?;
    m.add_function(wrap_pyfunction!(new, m)?)?;
    Ok(())
}

#[pyfunction]
fn new(dir: String) -> PyResult<TeraInstance> {
    return match Tera::new(dir.as_str()) {
        Ok(t) => Ok(TeraInstance { instance: t }),
        Err(e) => Err(exceptions::PyValueError::new_err(e.to_string())),
    };
}

#[pyclass]
struct TeraInstance {
    instance: Tera,
}

#[pymethods]
impl TeraInstance {
    fn render(
        self_: PyRef<Self>,
        template_name: String,
        context_dict: HashMap<String, String>,
    ) -> PyResult<String> {
        let mut context = Context::new();
        for (key, value) in context_dict {
            context.insert(key, &value);
        }
        return match self_.instance.render(&template_name, &context) {
            Ok(t) => Ok(t),
            Err(e) => Err(exceptions::PyValueError::new_err(e.to_string())),
        };
    }
}

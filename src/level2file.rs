use pyo3::prelude::*;


#[pyclass(name = "Level2File")]
pub struct PyLevel2File {
    a: f64,
    b: f64,
}

impl PyLevel2File {
    pub(crate) fn new(a: f64, b: f64) -> Self {
        Self {
            a,
            b
        }
    }
}

#[pymethods]
impl PyLevel2File {
    fn __repr__(&self) -> String {
        format!(
            "Level2File({}, {})",
            self.a, self.b
        )
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}
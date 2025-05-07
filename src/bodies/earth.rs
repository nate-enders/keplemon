use crate::saal::env_const_interface;
use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Debug, PartialEq)]
pub struct Earth {}

#[pymethods]
impl Earth {
    #[staticmethod]
    pub fn get_equatorial_radius() -> f64 {
        env_const_interface::get_earth_radius()
    }

    #[staticmethod]
    pub fn get_ke() -> f64 {
        env_const_interface::get_ke()
    }
}

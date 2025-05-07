use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeodeticPosition {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

#[pymethods]
impl GeodeticPosition {
    #[new]
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }
}

use super::CartesianVector;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalVector {
    range: f64,
    right_ascension: f64,
    declination: f64,
}

#[pymethods]
impl SphericalVector {
    #[new]
    pub fn new(range: f64, right_ascension: f64, declination: f64) -> Self {
        Self {
            range,
            right_ascension,
            declination,
        }
    }

    #[getter]
    pub fn get_range(&self) -> f64 {
        self.range
    }

    #[getter]
    pub fn get_right_ascension(&self) -> f64 {
        self.right_ascension
    }

    #[getter]
    pub fn get_declination(&self) -> f64 {
        self.declination
    }

    #[setter]
    pub fn set_range(&mut self, range: f64) {
        self.range = range;
    }

    #[setter]
    pub fn set_right_ascension(&mut self, right_ascension: f64) {
        self.right_ascension = right_ascension;
    }

    #[setter]
    pub fn set_declination(&mut self, declination: f64) {
        self.declination = declination;
    }

    pub fn to_cartesian(&self) -> CartesianVector {
        let right_ascension_rad = self.right_ascension.to_radians();
        let declination_rad = self.declination.to_radians();
        let cos_dec = declination_rad.cos();
        let x = self.range * right_ascension_rad.cos() * cos_dec;
        let y = self.range * right_ascension_rad.sin() * cos_dec;
        let z = self.range * declination_rad.sin();

        CartesianVector::new(x, y, z)
    }
}

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct HorizonElements {
    pub range: Option<f64>,
    pub range_rate: Option<f64>,
    pub azimuth: f64,
    pub elevation: f64,
    pub azimuth_rate: Option<f64>,
    pub elevation_rate: Option<f64>,
}
impl Copy for HorizonElements {}

#[pymethods]
impl HorizonElements {
    #[new]
    pub fn new(azimuth: f64, elevation: f64) -> Self {
        Self {
            range: None,
            range_rate: None,
            azimuth,
            elevation,
            azimuth_rate: None,
            elevation_rate: None,
        }
    }

    #[getter]
    pub fn get_azimuth(&self) -> f64 {
        self.azimuth
    }

    #[getter]
    pub fn get_elevation(&self) -> f64 {
        self.elevation
    }

    #[getter]
    pub fn get_range(&self) -> Option<f64> {
        self.range
    }

    #[getter]
    pub fn get_range_rate(&self) -> Option<f64> {
        self.range_rate
    }

    #[getter]
    pub fn get_azimuth_rate(&self) -> Option<f64> {
        self.azimuth_rate
    }

    #[getter]
    pub fn get_elevation_rate(&self) -> Option<f64> {
        self.elevation_rate
    }

    #[setter]
    pub fn set_range(&mut self, range: Option<f64>) {
        match range {
            Some(r) => self.range = Some(r),
            None => self.range = None,
        }
    }

    #[setter]
    pub fn set_range_rate(&mut self, range_rate: Option<f64>) {
        match range_rate {
            Some(rr) => self.range_rate = Some(rr),
            None => self.range_rate = None,
        }
    }

    #[setter]
    pub fn set_azimuth_rate(&mut self, azimuth_rate: Option<f64>) {
        match azimuth_rate {
            Some(az) => self.azimuth_rate = Some(az),
            None => self.azimuth_rate = None,
        }
    }

    #[setter]
    pub fn set_elevation_rate(&mut self, elevation_rate: Option<f64>) {
        match elevation_rate {
            Some(el) => self.elevation_rate = Some(el),
            None => self.elevation_rate = None,
        }
    }

    #[setter]
    pub fn set_azimuth(&mut self, azimuth: f64) {
        self.azimuth = azimuth
    }

    #[setter]
    pub fn set_elevation(&mut self, elevation: f64) {
        self.elevation = elevation
    }
}

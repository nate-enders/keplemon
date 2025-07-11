use super::HorizonAccess;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;

#[pyclass]
pub struct HorizonAccessReport {
    start: Epoch,
    end: Epoch,
    elevation_threshold: f64,
    duration_threshold: TimeSpan,
    accesses: Vec<HorizonAccess>,
}

impl HorizonAccessReport {
    pub fn new(start: Epoch, end: Epoch, elevation_threshold: f64, duration_threshold: TimeSpan) -> Self {
        Self {
            start,
            end,
            elevation_threshold,
            duration_threshold,
            accesses: Vec::new(),
        }
    }

    pub fn set_accesses(&mut self, horizon_accesses: Vec<HorizonAccess>) {
        self.accesses = horizon_accesses;
    }
}

#[pymethods]
impl HorizonAccessReport {
    #[getter]
    pub fn get_start(&self) -> Epoch {
        self.start
    }

    #[getter]
    pub fn get_end(&self) -> Epoch {
        self.end
    }

    #[getter]
    pub fn get_elevation_threshold(&self) -> f64 {
        self.elevation_threshold
    }

    #[getter]
    pub fn get_duration_threshold(&self) -> TimeSpan {
        self.duration_threshold
    }

    #[getter]
    pub fn get_accesses(&self) -> Vec<HorizonAccess> {
        self.accesses.clone()
    }
}

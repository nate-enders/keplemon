use super::CloseApproach;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
pub struct CloseApproachReport {
    start: Epoch,
    end: Epoch,
    distance_threshold: f64,
    close_approaches: Vec<CloseApproach>,
}

#[pymethods]
impl CloseApproachReport {
    #[new]
    pub fn new(start: Epoch, end: Epoch, distance_threshold: f64) -> Self {
        Self {
            start,
            end,
            distance_threshold,
            close_approaches: Vec::new(),
        }
    }

    #[getter]
    pub fn get_start(&self) -> Epoch {
        self.start
    }

    #[getter]
    pub fn get_end(&self) -> Epoch {
        self.end
    }

    #[getter]
    pub fn get_distance_threshold(&self) -> f64 {
        self.distance_threshold
    }

    #[getter]
    pub fn get_close_approaches(&self) -> Vec<CloseApproach> {
        self.close_approaches.clone()
    }

    #[setter]
    pub fn set_close_approaches(&mut self, close_approaches: Vec<CloseApproach>) {
        self.close_approaches = close_approaches;
    }
}

use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct CloseApproach {
    primary_id: i32,
    secondary_id: i32,
    epoch: Epoch,
    distance: f64,
}

impl CloseApproach {
    pub fn new(primary_id: i32, secondary_id: i32, epoch: Epoch, distance: f64) -> Self {
        Self {
            primary_id,
            secondary_id,
            epoch,
            distance,
        }
    }
}

#[pymethods]
impl CloseApproach {
    #[getter]
    pub fn get_primary_id(&self) -> i32 {
        self.primary_id
    }

    #[getter]
    pub fn get_secondary_id(&self) -> i32 {
        self.secondary_id
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    #[getter]
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}

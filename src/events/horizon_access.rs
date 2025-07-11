use crate::elements::HorizonState;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct HorizonAccess {
    satellite_id: i32,
    start: HorizonState,
    end: HorizonState,
}

impl HorizonAccess {
    pub fn new(satellite_id: i32, start: &HorizonState, end: &HorizonState) -> Self {
        Self {
            satellite_id,
            start: *start,
            end: *end,
        }
    }
}

#[pymethods]
impl HorizonAccess {
    #[getter]
    pub fn get_satellite_id(&self) -> i32 {
        self.satellite_id
    }

    #[getter]
    pub fn get_start(&self) -> HorizonState {
        self.start
    }

    #[getter]
    pub fn get_end(&self) -> HorizonState {
        self.end
    }

    #[setter]
    pub fn set_start(&mut self, start: HorizonState) {
        self.start = start;
    }

    #[setter]
    pub fn set_end(&mut self, end: HorizonState) {
        self.end = end;
    }
}

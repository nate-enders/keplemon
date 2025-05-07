use crate::saal::main_interface::{ALL_KEYMODE_DMA, ALL_KEYMODE_NODUP};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SAALKeyMode {
    NoDuplicates = ALL_KEYMODE_NODUP,
    DirectMemoryAccess = ALL_KEYMODE_DMA,
}

#[pymethods]
impl SAALKeyMode {
    #[getter]
    fn value(&self) -> i32 {
        match self {
            SAALKeyMode::NoDuplicates => ALL_KEYMODE_NODUP as i32,
            SAALKeyMode::DirectMemoryAccess => ALL_KEYMODE_DMA as i32,
        }
    }

    fn __repr__(&self) -> &'static str {
        match self {
            SAALKeyMode::NoDuplicates => "SAALKeyMode.NoDuplicates",
            SAALKeyMode::DirectMemoryAccess => "SAALKeyMode.DirectMemoryAccess",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

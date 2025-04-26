use pyo3::prelude::*;
use std::fmt::{Display, Formatter, Result};

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeSystem {
    UTC,
    TAI,
    UT1,
    TT,
}

impl Display for TimeSystem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TimeSystem::UTC => write!(f, "UTC"),
            TimeSystem::TAI => write!(f, "TAI"),
            TimeSystem::UT1 => write!(f, "UT1"),
            TimeSystem::TT => write!(f, "TT"),
        }
    }
}

#[pymethods]
impl TimeSystem {
    #[getter]
    fn value(&self) -> &str {
        match self {
            TimeSystem::UTC => "UTC",
            TimeSystem::TAI => "TAI",
            TimeSystem::UT1 => "UT1",
            TimeSystem::TT => "TT",
        }
    }

    fn __repr__(&self) -> &str {
        match self {
            TimeSystem::UTC => "TimeSystem.UTC",
            TimeSystem::TAI => "TimeSystem.TAI",
            TimeSystem::UT1 => "TimeSystem.UT1",
            TimeSystem::TT => "TimeSystem.TT",
        }
    }
    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

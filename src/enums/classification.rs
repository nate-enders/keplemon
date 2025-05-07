use pyo3::prelude::*;
use std::str::FromStr;

#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Classification {
    Unclassified = 1,
    Confidential = 2,
    Secret = 3,
}

impl FromStr for Classification {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "U" => Ok(Classification::Unclassified),
            "C" => Ok(Classification::Confidential),
            "S" => Ok(Classification::Secret),
            _ => Err(format!("Invalid TLE classification: {}", s)),
        }
    }
}

impl Classification {
    pub fn as_char(&self) -> &str {
        match self {
            Classification::Unclassified => "U",
            Classification::Confidential => "C",
            Classification::Secret => "S",
        }
    }
}

#[pymethods]
impl Classification {
    #[getter]
    fn value(&self) -> &str {
        self.as_char()
    }

    fn __repr__(&self) -> &str {
        match self {
            Classification::Unclassified => "Classification.Unclassified",
            Classification::Confidential => "Classification.Confidential",
            Classification::Secret => "Classification.Secret",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

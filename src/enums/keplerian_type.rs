use crate::saal::tle_interface::{TLETYPE_SGP, TLETYPE_SGP4, TLETYPE_SP, TLETYPE_XP};
use pyo3::prelude::*;
use std::convert::TryFrom;

#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeplerianType {
    MeanKozaiGP = TLETYPE_SGP,
    MeanBrouwerGP = TLETYPE_SGP4,
    MeanBrouwerXP = TLETYPE_XP,
    Osculating = TLETYPE_SP,
}

impl TryFrom<i32> for KeplerianType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeplerianType::MeanKozaiGP),
            2 => Ok(KeplerianType::MeanBrouwerGP),
            4 => Ok(KeplerianType::MeanBrouwerXP),
            _ => Err("Invalid KeplerianType value"),
        }
    }
}

impl TryFrom<f64> for KeplerianType {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value as i32 {
            0 => Ok(KeplerianType::MeanKozaiGP),
            2 => Ok(KeplerianType::MeanBrouwerGP),
            4 => Ok(KeplerianType::MeanBrouwerXP),
            _ => Err("Invalid KeplerianType value"),
        }
    }
}

#[pymethods]
impl KeplerianType {
    pub fn __repr__(&self) -> &'static str {
        match self {
            KeplerianType::MeanKozaiGP => "KeplerianType.MeanKozaiGP",
            KeplerianType::MeanBrouwerGP => "KeplerianType.MeanBrouwerGP",
            KeplerianType::MeanBrouwerXP => "KeplerianType.MeanBrouwerXP",
            KeplerianType::Osculating => "KeplerianType.Osculating",
        }
    }

    #[getter]
    pub fn value(&self) -> i32 {
        match self {
            KeplerianType::MeanKozaiGP => TLETYPE_SGP as i32,
            KeplerianType::MeanBrouwerGP => TLETYPE_SGP4 as i32,
            KeplerianType::MeanBrouwerXP => TLETYPE_XP as i32,
            KeplerianType::Osculating => TLETYPE_XP as i32,
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

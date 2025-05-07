use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CovarianceType {
    Inertial,
    Relative,
    Equinoctial,
}

#[pymethods]
impl CovarianceType {
    #[getter]
    fn get_value(&self) -> &str {
        match self {
            CovarianceType::Inertial => "Inertial",
            CovarianceType::Relative => "Relative",
            CovarianceType::Equinoctial => "Equinoctial",
        }
    }

    fn __repr__(&self) -> &str {
        match self {
            CovarianceType::Inertial => "CovarianceType.Inertial",
            CovarianceType::Relative => "CovarianceType.Relative",
            CovarianceType::Equinoctial => "CovarianceType.Equinoctial",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

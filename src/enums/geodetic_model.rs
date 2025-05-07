use crate::saal::env_const_interface;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeodeticModel {
    WGS72 = env_const_interface::XF_GEOMOD_WGS72,
    WGS84 = env_const_interface::XF_GEOMOD_WGS84,
    EGM96 = env_const_interface::XF_GEOMOD_EGM96,
}

#[pymethods]
impl GeodeticModel {
    #[getter]
    fn value(&self) -> i32 {
        match self {
            GeodeticModel::WGS72 => env_const_interface::XF_GEOMOD_WGS72 as i32,
            GeodeticModel::WGS84 => env_const_interface::XF_GEOMOD_WGS84 as i32,
            GeodeticModel::EGM96 => env_const_interface::XF_GEOMOD_EGM96 as i32,
        }
    }

    fn __repr__(&self) -> &'static str {
        match self {
            GeodeticModel::WGS72 => "GeodeticModel.WGS72",
            GeodeticModel::WGS84 => "GeodeticModel.WGS84",
            GeodeticModel::EGM96 => "GeodeticModel.EGM96",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

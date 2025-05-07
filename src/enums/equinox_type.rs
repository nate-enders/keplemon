use crate::saal::astro_func_interface;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EquinoxType {
    MeanOfDate = astro_func_interface::YROFEQNX_CURR,
    J2000 = astro_func_interface::YROFEQNX_2000,
}

#[pymethods]
impl EquinoxType {
    #[getter]
    pub fn get_value(&self) -> i32 {
        match self {
            EquinoxType::MeanOfDate => astro_func_interface::YROFEQNX_CURR as i32,
            EquinoxType::J2000 => astro_func_interface::YROFEQNX_2000 as i32,
        }
    }

    fn __repr__(&self) -> &str {
        match self {
            EquinoxType::MeanOfDate => "EquinoxType.MeanOfDate",
            EquinoxType::J2000 => "EquinoxType.J2000",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

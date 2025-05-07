use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferenceFrame {
    TEME,
    EFG,
    ECR,
    J2000,
}

#[pymethods]
impl ReferenceFrame {
    #[getter]
    fn value(&self) -> &str {
        match self {
            ReferenceFrame::TEME => "TEME",
            ReferenceFrame::EFG => "EFG",
            ReferenceFrame::ECR => "ECR",
            ReferenceFrame::J2000 => "J2000",
        }
    }

    fn __repr__(&self) -> &str {
        match self {
            ReferenceFrame::TEME => "ReferenceFrame.TEME",
            ReferenceFrame::EFG => "ReferenceFrame.EFG",
            ReferenceFrame::ECR => "ReferenceFrame.ECR",
            ReferenceFrame::J2000 => "ReferenceFrame.J2000",
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __ne__(&self, other: &Self) -> bool {
        self != other
    }
}

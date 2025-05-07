use std::ops::{Index, IndexMut};

use crate::enums::KeplerianType;
use crate::saal::astro_func_interface;
use pyo3::prelude::*;

use super::EquinoctialElements;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianElements {
    semi_major_axis: f64,
    eccentricity: f64,
    inclination: f64,
    raan: f64,
    argument_of_perigee: f64,
    mean_anomaly: f64,
}

impl From<&EquinoctialElements> for KeplerianElements {
    fn from(eqn: &EquinoctialElements) -> Self {
        let xa_kep = astro_func_interface::equinoctial_to_keplerian(&eqn.into());
        Self::from(xa_kep)
    }
}

impl From<&KeplerianElements> for [f64; astro_func_interface::XA_KEP_SIZE] {
    fn from(kep: &KeplerianElements) -> Self {
        let mut xa_kep = [0.0; astro_func_interface::XA_KEP_SIZE];
        xa_kep[astro_func_interface::XA_KEP_A] = kep.semi_major_axis;
        xa_kep[astro_func_interface::XA_KEP_MA] = kep.mean_anomaly;
        xa_kep[astro_func_interface::XA_KEP_E] = kep.eccentricity;
        xa_kep[astro_func_interface::XA_KEP_INCLI] = kep.inclination;
        xa_kep[astro_func_interface::XA_KEP_NODE] = kep.raan;
        xa_kep[astro_func_interface::XA_KEP_OMEGA] = kep.argument_of_perigee;
        xa_kep
    }
}

impl From<[f64; astro_func_interface::XA_KEP_SIZE]> for KeplerianElements {
    fn from(xa_kep: [f64; astro_func_interface::XA_KEP_SIZE]) -> Self {
        Self {
            semi_major_axis: xa_kep[astro_func_interface::XA_KEP_A],
            eccentricity: xa_kep[astro_func_interface::XA_KEP_E],
            inclination: xa_kep[astro_func_interface::XA_KEP_INCLI],
            raan: xa_kep[astro_func_interface::XA_KEP_NODE],
            argument_of_perigee: xa_kep[astro_func_interface::XA_KEP_OMEGA],
            mean_anomaly: xa_kep[astro_func_interface::XA_KEP_MA],
        }
    }
}

impl IndexMut<usize> for KeplerianElements {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            astro_func_interface::XA_KEP_A => &mut self.semi_major_axis,
            astro_func_interface::XA_KEP_E => &mut self.eccentricity,
            astro_func_interface::XA_KEP_INCLI => &mut self.inclination,
            astro_func_interface::XA_KEP_NODE => &mut self.raan,
            astro_func_interface::XA_KEP_OMEGA => &mut self.argument_of_perigee,
            astro_func_interface::XA_KEP_MA => &mut self.mean_anomaly,
            _ => panic!("Index out of bounds for Keplerian elements"),
        }
    }
}

impl Index<usize> for KeplerianElements {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            astro_func_interface::XA_KEP_A => &self.semi_major_axis,
            astro_func_interface::XA_KEP_E => &self.eccentricity,
            astro_func_interface::XA_KEP_INCLI => &self.inclination,
            astro_func_interface::XA_KEP_NODE => &self.raan,
            astro_func_interface::XA_KEP_OMEGA => &self.argument_of_perigee,
            astro_func_interface::XA_KEP_MA => &self.mean_anomaly,
            _ => panic!("Index out of bounds for Keplerian elements"),
        }
    }
}

impl KeplerianElements {
    pub fn get_xa_kep(&self) -> [f64; astro_func_interface::XA_CLS_SIZE] {
        let mut xa_kep = [0.0; astro_func_interface::XA_KEP_SIZE];
        xa_kep[astro_func_interface::XA_KEP_A] = self.semi_major_axis;
        xa_kep[astro_func_interface::XA_KEP_MA] = self.mean_anomaly;
        xa_kep[astro_func_interface::XA_KEP_E] = self.eccentricity;
        xa_kep[astro_func_interface::XA_KEP_INCLI] = self.inclination;
        xa_kep[astro_func_interface::XA_KEP_NODE] = self.raan;
        xa_kep[astro_func_interface::XA_KEP_OMEGA] = self.argument_of_perigee;
        xa_kep
    }

    pub fn get_xa_cls(&self, element_type: KeplerianType) -> [f64; astro_func_interface::XA_CLS_SIZE] {
        let mut xa_cls = [0.0; astro_func_interface::XA_CLS_SIZE];
        xa_cls[astro_func_interface::XA_CLS_N] = self.get_mean_motion(element_type);
        xa_cls[astro_func_interface::XA_CLS_MA] = self.mean_anomaly;
        xa_cls[astro_func_interface::XA_CLS_E] = self.eccentricity;
        xa_cls[astro_func_interface::XA_CLS_INCLI] = self.inclination;
        xa_cls[astro_func_interface::XA_CLS_NODE] = self.raan;
        xa_cls[astro_func_interface::XA_CLS_OMEGA] = self.argument_of_perigee;
        xa_cls
    }
}

#[pymethods]
impl KeplerianElements {
    #[new]
    pub fn new(
        semi_major_axis: f64,
        eccentricity: f64,
        inclination: f64,
        raan: f64,
        argument_of_perigee: f64,
        mean_anomaly: f64,
    ) -> Self {
        Self {
            semi_major_axis,
            eccentricity,
            inclination,
            raan,
            argument_of_perigee,
            mean_anomaly,
        }
    }

    #[getter]
    pub fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    #[getter]
    pub fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    #[getter]
    pub fn get_inclination(&self) -> f64 {
        self.inclination
    }

    #[getter]
    pub fn get_apoapsis(&self) -> f64 {
        self.semi_major_axis * (1.0 + self.eccentricity)
    }

    #[getter]
    pub fn get_periapsis(&self) -> f64 {
        self.semi_major_axis * (1.0 - self.eccentricity)
    }

    #[getter]
    pub fn get_raan(&self) -> f64 {
        self.raan
    }

    #[getter]
    pub fn get_argument_of_perigee(&self) -> f64 {
        self.argument_of_perigee
    }

    #[getter]
    pub fn get_mean_anomaly(&self) -> f64 {
        self.mean_anomaly
    }

    pub fn get_mean_motion(&self, element_type: KeplerianType) -> f64 {
        let mean_motion = astro_func_interface::sma_to_mean_motion(self.semi_major_axis);
        match element_type {
            KeplerianType::MeanKozaiGP => {
                astro_func_interface::brouwer_to_kozai(self.eccentricity, self.inclination, mean_motion)
            }
            _ => mean_motion,
        }
    }

    #[setter]
    pub fn set_semi_major_axis(&mut self, semi_major_axis: f64) {
        self.semi_major_axis = semi_major_axis;
    }

    #[setter]
    pub fn set_eccentricity(&mut self, eccentricity: f64) {
        self.eccentricity = eccentricity;
    }

    #[setter]
    pub fn set_inclination(&mut self, inclination: f64) {
        self.inclination = inclination;
    }

    #[setter]
    pub fn set_raan(&mut self, raan: f64) {
        self.raan = raan;
    }

    #[setter]
    pub fn set_argument_of_perigee(&mut self, argument_of_perigee: f64) {
        self.argument_of_perigee = argument_of_perigee;
    }

    #[setter]
    pub fn set_mean_anomaly(&mut self, mean_anomaly: f64) {
        self.mean_anomaly = mean_anomaly;
    }

    pub fn to_equinoctial(&self) -> EquinoctialElements {
        EquinoctialElements::from(self)
    }

    pub fn to_mean(&self) -> Self {
        astro_func_interface::osculating_to_mean(&self.get_xa_kep()).into()
    }
}

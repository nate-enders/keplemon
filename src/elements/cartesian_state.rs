use super::{CartesianVector, KeplerianElements, KeplerianState};
use crate::enums::{KeplerianType, ReferenceFrame};
use crate::saal::astro_func_interface;
use crate::time::Epoch;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct CartesianState {
    pub epoch: Epoch,
    pub position: CartesianVector,
    pub velocity: CartesianVector,
    frame: ReferenceFrame,
}

impl CartesianState {
    pub fn set_element(&mut self, element: usize, value: f64) {
        match element {
            0 => self.position[0] = value,
            1 => self.position[1] = value,
            2 => self.position[2] = value,
            3 => self.velocity[0] = value,
            4 => self.velocity[1] = value,
            5 => self.velocity[2] = value,
            _ => panic!("Invalid element index"),
        }
    }

    pub fn get_element(&self, element: usize) -> f64 {
        match element {
            0 => self.position[0],
            1 => self.position[1],
            2 => self.position[2],
            3 => self.velocity[0],
            4 => self.velocity[1],
            5 => self.velocity[2],
            _ => panic!("Invalid element index"),
        }
    }
}

#[pymethods]
impl CartesianState {
    #[new]
    pub fn new(epoch: Epoch, position: CartesianVector, velocity: CartesianVector, frame: ReferenceFrame) -> Self {
        Self {
            epoch,
            position,
            velocity,
            frame,
        }
    }

    #[getter]
    pub fn get_position(&self) -> CartesianVector {
        self.position
    }

    #[getter]
    pub fn get_velocity(&self) -> CartesianVector {
        self.velocity
    }

    #[getter]
    pub fn get_epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn get_frame(&self) -> ReferenceFrame {
        self.frame
    }

    pub fn to_keplerian(&self) -> KeplerianState {
        let pos: [f64; 3] = [self.position[0], self.position[1], self.position[2]];
        let vel: [f64; 3] = [self.velocity[0], self.velocity[1], self.velocity[2]];
        let kep = KeplerianElements::from(astro_func_interface::cartesian_to_keplerian(&pos, &vel));
        KeplerianState::new(self.epoch, kep, self.frame, KeplerianType::Osculating)
    }

    pub fn to_frame(&self, frame: ReferenceFrame) -> CartesianState {
        let in_pos: [f64; 3] = [self.position[0], self.position[1], self.position[2]];
        let in_vel: [f64; 3] = [self.velocity[0], self.velocity[1], self.velocity[2]];
        match self.frame {
            ReferenceFrame::TEME => match frame {
                ReferenceFrame::TEME => *self,
                ReferenceFrame::J2000 => {
                    let (out_pos, out_vel) =
                        astro_func_interface::teme_to_j2000(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::ECR => {
                    let (out_pos, out_vel) =
                        astro_func_interface::teme_to_ecr(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::EFG => {
                    let (out_pos, out_vel) =
                        astro_func_interface::teme_to_efg(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
            },
            ReferenceFrame::J2000 => match frame {
                ReferenceFrame::TEME => {
                    let (out_pos, out_vel) =
                        astro_func_interface::j2000_to_teme(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::J2000 => *self,
                ReferenceFrame::ECR => {
                    let (out_pos, out_vel) =
                        astro_func_interface::j2000_to_ecr(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::EFG => {
                    let (out_pos, out_vel) =
                        astro_func_interface::j2000_to_efg(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
            },
            ReferenceFrame::ECR => match frame {
                ReferenceFrame::TEME => {
                    let (out_pos, out_vel) =
                        astro_func_interface::ecr_to_teme(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::J2000 => {
                    let (out_pos, out_vel) =
                        astro_func_interface::ecr_to_j2000(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::ECR => *self,
                ReferenceFrame::EFG => {
                    let (out_pos, out_vel) =
                        astro_func_interface::ecr_to_efg(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
            },
            ReferenceFrame::EFG => match frame {
                ReferenceFrame::TEME => {
                    let (out_pos, out_vel) =
                        astro_func_interface::efg_to_teme(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::J2000 => {
                    let (out_pos, out_vel) =
                        astro_func_interface::efg_to_j2000(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::ECR => {
                    let (out_pos, out_vel) =
                        astro_func_interface::efg_to_ecr(self.epoch.days_since_1950, &in_pos, &in_vel);
                    CartesianState::new(
                        self.epoch,
                        CartesianVector::from(out_pos),
                        CartesianVector::from(out_vel),
                        frame,
                    )
                }
                ReferenceFrame::EFG => *self,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CartesianState, CartesianVector};
    use crate::enums::{ReferenceFrame, TimeSystem};
    use crate::time::Epoch;
    use approx::assert_abs_diff_eq;

    fn epoch_1() -> Epoch {
        Epoch::from_days_since_1950(25142.432, TimeSystem::UTC)
    }
    fn geo_position() -> CartesianVector {
        CartesianVector::new(42164.0, 0.0, 0.0)
    }
    fn geo_velocity() -> CartesianVector {
        CartesianVector::new(0.0, 3.0746676656429814, 0.0)
    }

    #[test]
    fn test_to_keplerian() {
        let state = CartesianState::new(epoch_1(), geo_position(), geo_velocity(), ReferenceFrame::TEME);
        let osc = state.to_keplerian();
        assert_abs_diff_eq!(osc.get_semi_major_axis(), 42164.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_mean_anomaly(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_eccentricity(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_inclination(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_raan(), 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(osc.get_argument_of_perigee(), 0.0, epsilon = 1e-6);
        assert_eq!(osc.get_frame(), ReferenceFrame::TEME);
        assert_eq!(osc.get_epoch(), state.epoch);
    }
}

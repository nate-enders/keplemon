use crate::bodies::Earth;
use crate::configs::{CONJUNCTION_STEP_MINUTES, MAX_NEWTON_ITERATIONS, NEWTON_TOLERANCE};
use crate::elements::{CartesianState, CartesianVector};
use crate::enums::{ReferenceFrame, TimeSystem};
use crate::events::CloseApproach;
use crate::saal::ext_ephem_interface;
use crate::time::{Epoch, TimeSpan};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, PartialEq)]
pub struct Ephemeris {
    key: i64,
    satellite_id: i32,
}

impl Drop for Ephemeris {
    fn drop(&mut self) {
        ext_ephem_interface::remove_key(self.key);
    }
}

#[pymethods]
impl Ephemeris {
    #[new]
    pub fn new(satellite_id: i32, state: CartesianState) -> Self {
        let frame = match state.get_frame() {
            ReferenceFrame::TEME => ext_ephem_interface::COORD_ECI,
            ReferenceFrame::EFG => ext_ephem_interface::COORD_EFG,
            ReferenceFrame::ECR => ext_ephem_interface::COORD_ECR,
            ReferenceFrame::J2000 => ext_ephem_interface::COORD_J2K,
        };
        let key = ext_ephem_interface::add_satellite(
            satellite_id,
            state.get_epoch().days_since_1950,
            Earth::get_equatorial_radius(),
            Earth::get_ke(),
            frame,
        );
        let ephem = Self { key, satellite_id };
        ext_ephem_interface::add_satellite_state(
            key,
            state.get_epoch().days_since_1950,
            &state.get_position().into(),
            &state.get_velocity().into(),
            None,
        )
        .unwrap();
        ephem
    }

    pub fn add_state(&self, state: CartesianState) {
        ext_ephem_interface::add_satellite_state(
            self.key,
            state.get_epoch().days_since_1950,
            &state.get_position().into(),
            &state.get_velocity().into(),
            None,
        )
        .unwrap();
    }

    #[getter]
    pub fn get_key(&self) -> i64 {
        self.key
    }

    pub fn get_state_at_epoch(&self, epoch: Epoch) -> Option<CartesianState> {
        let result = ext_ephem_interface::get_posvel_at_ds50(self.key, epoch.days_since_1950);
        match result {
            Ok((pos, vel)) => {
                let pos = CartesianVector::from(pos);
                let vel = CartesianVector::from(vel);
                Some(CartesianState::new(epoch, pos, vel, ReferenceFrame::TEME))
            }
            Err(_) => None,
        }
    }

    pub fn get_close_approach(&self, other: &Ephemeris, distance_threshold: f64) -> Option<CloseApproach> {
        let (ds50_start, ds50_end) = ext_ephem_interface::get_ds50_utc_range(self.key).unwrap();
        let start_epoch = Epoch::from_days_since_1950(ds50_start, TimeSystem::UTC);
        let end_epoch = Epoch::from_days_since_1950(ds50_end, TimeSystem::UTC);

        let mut closest_epoch = start_epoch;
        let mut min_distance = f64::MAX;
        let mut current_epoch = start_epoch;
        let step = TimeSpan::from_minutes(CONJUNCTION_STEP_MINUTES);

        while current_epoch <= end_epoch {
            let state_1 = self.get_state_at_epoch(current_epoch);
            let state_2 = other.get_state_at_epoch(current_epoch);

            if state_1.is_none() || state_2.is_none() {
                break;
            }

            // Estimate the time of closest approach
            let t_guess = estimate_close_approach_epoch(&state_1?, &state_2?);

            match t_guess {
                Some(t) => {
                    let t_min = current_epoch;
                    let t_max = current_epoch + step;

                    if t < t_min || t > t_max {
                        current_epoch += step;
                        continue;
                    }
                    if let Some(ca) = refine_close_approach(self, other, t) {
                        if ca.get_distance() < min_distance && ca.get_epoch() >= t_min && ca.get_epoch() < t_max {
                            min_distance = ca.get_distance();
                            closest_epoch = ca.get_epoch();
                        }
                    }
                }
                None => {
                    break;
                }
            }

            current_epoch += step;
        }
        if min_distance < distance_threshold {
            Some(CloseApproach::new(
                self.satellite_id,
                other.satellite_id,
                closest_epoch,
                min_distance,
            ))
        } else {
            None
        }
    }

    #[getter]
    pub fn get_satellite_id(&self) -> i32 {
        self.satellite_id
    }
}

fn estimate_close_approach_epoch(state_1: &CartesianState, state_2: &CartesianState) -> Option<Epoch> {
    if state_1.epoch != state_2.epoch {
        None
    } else {
        let t0 = state_1.epoch;

        // Calculate the relative position and velocity
        let dx0 = state_1.position - state_2.position;
        let dv0 = state_1.velocity - state_2.velocity;

        // Quadratic minimization: d(t)^2 = |dx0 + dv0*(t-t0)|^2
        // d/dt set to zero gives t = t0 - (dx0 . dv0)/(dv0 . dv0)
        let numerator = dx0.dot(&dv0);
        let denominator = dv0.dot(&dv0);

        if denominator.abs() < 1e-12 {
            Some(t0)
        } else {
            Some(t0 - TimeSpan::from_seconds(numerator / denominator))
        }
    }
}

fn refine_close_approach(ephem_1: &Ephemeris, ephem_2: &Ephemeris, t_guess: Epoch) -> Option<CloseApproach> {
    // Use Newton's method to refine the time of closest approach
    let mut t = t_guess;

    for _ in 0..MAX_NEWTON_ITERATIONS {
        // Propagate both satellites to time t and get their positions and velocities
        let state_1 = ephem_1.get_state_at_epoch(t)?;
        let state_2 = ephem_2.get_state_at_epoch(t)?;

        let dr = state_1.position - state_2.position;
        let dv = state_1.velocity - state_2.velocity;
        let drdv = dr.dot(&dv);
        let dvdv = dv.dot(&dv);

        // Newton-Raphson step
        let dt = -drdv / dvdv;
        t += TimeSpan::from_seconds(dt);

        if dt.abs() < NEWTON_TOLERANCE {
            break;
        }
    }

    // At final t, compute range
    let state_1 = ephem_1.get_state_at_epoch(t)?;
    let state_2 = ephem_2.get_state_at_epoch(t)?;
    let range = (state_1.position - state_2.position).get_magnitude();

    Some(CloseApproach::new(ephem_1.satellite_id, ephem_2.satellite_id, t, range))
}

use super::{DAYS_TO_HOURS, DAYS_TO_MINUTES, DAYS_TO_SECONDS, HOURS_TO_DAYS, MINUTES_TO_DAYS, SECONDS_TO_DAYS};
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan {
    days: f64,
}

impl TimeSpan {}

#[pymethods]
impl TimeSpan {
    #[staticmethod]
    pub fn from_days(days: f64) -> Self {
        Self { days }
    }

    #[staticmethod]
    pub fn from_seconds(seconds: f64) -> Self {
        Self {
            days: seconds * SECONDS_TO_DAYS,
        }
    }

    #[staticmethod]
    pub fn from_minutes(minutes: f64) -> Self {
        Self {
            days: minutes * MINUTES_TO_DAYS,
        }
    }

    #[staticmethod]
    pub fn from_hours(hours: f64) -> Self {
        Self {
            days: hours * HOURS_TO_DAYS,
        }
    }

    pub fn in_days(&self) -> f64 {
        self.days
    }

    pub fn in_seconds(&self) -> f64 {
        self.days * DAYS_TO_SECONDS
    }

    pub fn in_minutes(&self) -> f64 {
        self.days * DAYS_TO_MINUTES
    }

    pub fn in_hours(&self) -> f64 {
        self.days * DAYS_TO_HOURS
    }
}

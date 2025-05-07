use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    sensor_id: Option<i32>,
    name: String,
    angular_noise: f64,
    range_noise: Option<f64>,
    range_rate_noise: Option<f64>,
    angular_rate_noise: Option<f64>,
}

#[pymethods]
impl Sensor {
    #[new]
    pub fn new(name: String, angular_noise: f64) -> Self {
        Self {
            sensor_id: None,
            name,
            angular_noise,
            range_noise: None,
            range_rate_noise: None,
            angular_rate_noise: None,
        }
    }

    #[getter]
    pub fn get_sensor_id(&self) -> Option<i32> {
        self.sensor_id
    }

    #[getter]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    pub fn get_angular_noise(&self) -> f64 {
        self.angular_noise
    }

    #[getter]
    pub fn get_range_noise(&self) -> Option<f64> {
        self.range_noise
    }

    #[getter]
    pub fn get_range_rate_noise(&self) -> Option<f64> {
        self.range_rate_noise
    }

    #[getter]
    pub fn get_angular_rate_noise(&self) -> Option<f64> {
        self.angular_rate_noise
    }

    #[setter]
    pub fn set_range_noise(&mut self, range_noise: f64) {
        self.range_noise = Some(range_noise);
    }

    #[setter]
    pub fn set_range_rate_noise(&mut self, range_rate_noise: f64) {
        self.range_rate_noise = Some(range_rate_noise);
    }

    #[setter]
    pub fn set_angular_rate_noise(&mut self, angular_rate_noise: f64) {
        self.angular_rate_noise = Some(angular_rate_noise);
    }
}

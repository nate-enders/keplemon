use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeComponents {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: f64,
}

impl TimeComponents {}

#[pymethods]
impl TimeComponents {
    #[getter]
    pub fn year(&self) -> i32 {
        self.year
    }

    #[getter]
    pub fn month(&self) -> i32 {
        self.month
    }

    #[getter]
    pub fn day(&self) -> i32 {
        self.day
    }

    #[getter]
    pub fn hour(&self) -> i32 {
        self.hour
    }

    #[getter]
    pub fn minute(&self) -> i32 {
        self.minute
    }

    #[getter]
    pub fn second(&self) -> f64 {
        self.second
    }

    #[setter]
    pub fn set_year(&mut self, year: i32) {
        self.year = year;
    }

    #[setter]
    pub fn set_month(&mut self, month: i32) {
        self.month = month;
    }

    #[setter]
    pub fn set_day(&mut self, day: i32) {
        self.day = day;
    }

    #[setter]
    pub fn set_hour(&mut self, hour: i32) {
        self.hour = hour;
    }

    #[setter]
    pub fn set_minute(&mut self, minute: i32) {
        self.minute = minute;
    }

    #[setter]
    pub fn set_second(&mut self, second: f64) {
        self.second = second;
    }

    #[new]
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: f64) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn to_iso(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:.3}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    #[staticmethod]
    pub fn from_iso(iso: &str) -> Self {
        let date_time: Vec<&str> = iso.split("T").collect();
        let ymd: Vec<&str> = date_time[0].split("-").collect();
        let z_stripped = date_time[1].replace("Z", "");
        let hms: Vec<&str> = z_stripped.split(":").collect();

        Self {
            year: ymd[0].parse().unwrap(),
            month: ymd[1].parse().unwrap(),
            day: ymd[2].parse().unwrap(),
            hour: hms[0].parse().unwrap(),
            minute: hms[1].parse().unwrap(),
            second: hms[2].parse().unwrap(),
        }
    }

    fn __eq__(&self, other: &Self) -> PyResult<bool> {
        Ok(self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
            && (self.second - other.second).abs() < f64::EPSILON)
    }

    fn __ne__(&self, other: &Self) -> PyResult<bool> {
        Ok(self.year != other.year
            || self.month != other.month
            || self.day != other.day
            || self.hour != other.hour
            || self.minute != other.minute
            || (self.second - other.second).abs() >= f64::EPSILON)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "TimeComponents(year={}, month={}, day={}, hour={}, minute={}, second={})",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        ))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(TimeComponents::to_iso(self))
    }
}

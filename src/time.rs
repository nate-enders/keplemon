use crate::saal::time_func_interface;
mod epoch;
mod time_components;
mod time_span;

use pyo3::prelude::*;
use pyo3::py_run;

pub use epoch::Epoch;
pub use time_components::TimeComponents;
pub use time_span::TimeSpan;

pub const DAYS_TO_SECONDS: f64 = 86400.0;
pub const DAYS_TO_MINUTES: f64 = 1440.0;
pub const DAYS_TO_HOURS: f64 = 24.0;
pub const SECONDS_TO_DAYS: f64 = 1.0 / DAYS_TO_SECONDS;
pub const MINUTES_TO_DAYS: f64 = 1.0 / DAYS_TO_MINUTES;
pub const HOURS_TO_DAYS: f64 = 1.0 / DAYS_TO_HOURS;

pub fn register_time(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let time = PyModule::new(parent_module.py(), "time")?;
    time.add_function(wrap_pyfunction!(time_func_interface::load_time_constants, &time)?)?;
    time.add_function(wrap_pyfunction!(time_func_interface::time_constants_loaded, &time)?)?;
    time.add_class::<TimeSpan>()?;
    time.add_class::<Epoch>()?;
    time.add_class::<TimeComponents>()?;
    py_run!(
        parent_module.py(),
        time,
        "import sys; sys.modules['keplemon._keplemon.time'] = time"
    );
    parent_module.add_submodule(&time)
}

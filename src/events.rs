mod close_approach;
mod close_approach_report;

pub use close_approach::CloseApproach;
pub use close_approach_report::CloseApproachReport;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_events(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let events = PyModule::new(parent_module.py(), "events")?;
    events.add_class::<CloseApproach>()?;
    events.add_class::<CloseApproachReport>()?;
    py_run!(
        parent_module.py(),
        events,
        "import sys; sys.modules['keplemon._keplemon.events'] = events"
    );
    parent_module.add_submodule(&events)
}

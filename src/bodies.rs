mod constellation;
mod earth;
mod observatory;
mod satellite;
mod sensor;

pub use constellation::Constellation;
pub use earth::Earth;
pub use observatory::Observatory;
pub use satellite::Satellite;
pub use sensor::Sensor;

use pyo3::prelude::*;
use pyo3::py_run;

pub fn register_bodies(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let bodies = PyModule::new(parent_module.py(), "bodies")?;
    bodies.add_class::<Satellite>()?;
    bodies.add_class::<Constellation>()?;
    bodies.add_class::<Earth>()?;
    bodies.add_class::<Sensor>()?;
    bodies.add_class::<Observatory>()?;
    py_run!(
        parent_module.py(),
        bodies,
        "import sys; sys.modules['keplemon._keplemon.bodies'] = bodies"
    );
    parent_module.add_submodule(&bodies)
}

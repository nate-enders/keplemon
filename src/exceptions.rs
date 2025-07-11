use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::py_run;

create_exception!(keplemon.exceptions, SAALError, PyException);

pub fn register_exceptions(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let exceptions = PyModule::new(parent_module.py(), "exceptions")?;
    exceptions.add("SAALError", parent_module.py().get_type::<SAALError>())?;
    py_run!(
        parent_module.py(),
        exceptions,
        "import sys; sys.modules['keplemon._keplemon.exceptions'] = exceptions"
    );
    parent_module.add_submodule(&exceptions)
}

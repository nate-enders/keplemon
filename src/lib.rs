pub mod bodies;
pub mod catalogs;
pub mod configs;
pub mod elements;
pub mod enums;
pub mod estimation;
pub mod events;
pub mod propagation;
pub mod saal;
pub mod time;

use ctor::ctor;
use pyo3::prelude::*;
use rayon::current_num_threads;

#[ctor]
fn init() {
    saal::main_interface::set_key_mode(enums::SAALKeyMode::DirectMemoryAccess);
}

#[pyfunction]
fn get_thread_count() -> usize {
    current_num_threads()
}

#[pyfunction]
fn set_thread_count(count: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(count)
        .build_global()
        .unwrap();
}

// The top-level module that includes functions and nested submodules.
#[pymodule]
fn _keplemon(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_thread_count, m)?)?;
    m.add_function(wrap_pyfunction!(set_thread_count, m)?)?;
    saal::register_saal(m)?;
    enums::register_enums(m)?;
    time::register_time(m)?;
    elements::register_elements(m)?;
    propagation::register_propagation(m)?;
    catalogs::register_catalogs(m)?;
    bodies::register_bodies(m)?;
    events::register_events(m)?;
    estimation::register_estimation(m)?;
    Ok(())
}

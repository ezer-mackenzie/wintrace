mod live;
mod offline;
mod record;
mod xml;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use record::EventRecord;

#[pyfunction]
fn xml_to_dict(py: Python<'_>, xml: &str) -> PyResult<Py<PyAny>> {
    let value = xml::parse_event_xml(xml).map_err(PyValueError::new_err)?;
    Ok(pythonize::pythonize(py, &value)?.unbind())
}

#[pyfunction]
fn xml_to_json(xml: &str) -> PyResult<String> {
    let value = xml::parse_event_xml(xml).map_err(PyValueError::new_err)?;
    serde_json::to_string_pretty(&value).map_err(|error| PyValueError::new_err(error.to_string()))
}

#[pymodule]
fn _wintrace(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<EventRecord>()?;
    module.add_class::<offline::EvtxFile>()?;
    module.add_class::<offline::EvtxRecordIterator>()?;
    module.add_class::<live::LiveEventIterator>()?;
    module.add_class::<live::LiveEventLog>()?;
    module.add_function(wrap_pyfunction!(offline::read_evtx, module)?)?;
    module.add_function(wrap_pyfunction!(offline::iter_evtx, module)?)?;
    module.add_function(wrap_pyfunction!(live::query_events, module)?)?;
    module.add_function(wrap_pyfunction!(live::iter_events, module)?)?;
    module.add_function(wrap_pyfunction!(xml_to_dict, module)?)?;
    module.add_function(wrap_pyfunction!(xml_to_json, module)?)?;
    Ok(())
}

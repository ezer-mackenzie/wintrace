use crate::record::EventRecord;
use pyo3::exceptions::{PyFileNotFoundError, PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};

#[pyclass(module = "wintrace")]
pub struct EvtxRecordIterator {
    records: Arc<Mutex<Receiver<Result<String, String>>>>,
    format: String,
}

#[pyclass(module = "wintrace")]
pub struct EvtxFile {
    path: PathBuf,
}

pub(crate) fn validate_format(format: &str) -> PyResult<()> {
    match format {
        "record" | "dict" | "xml" | "json" => Ok(()),
        _ => Err(PyValueError::new_err(format!(
            "unsupported format {format:?}; expected 'record', 'dict', 'xml', or 'json'"
        ))),
    }
}

fn open_records(path: &Path) -> PyResult<Receiver<Result<String, String>>> {
    if !path.exists() {
        return Err(PyFileNotFoundError::new_err(format!(
            "EVTX file not found: {}",
            path.display()
        )));
    }
    let mut parser = evtx::EvtxParser::from_path(path)
        .map_err(|error| PyValueError::new_err(format!("invalid EVTX file: {error}")))?;
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name("wintrace-evtx-reader".to_owned())
        .spawn(move || {
            for record in parser.records() {
                let result = record
                    .map(|record| record.data)
                    .map_err(|error| error.to_string());
                if sender.send(result).is_err() {
                    break;
                }
            }
        })
        .map_err(|error| PyIOError::new_err(error.to_string()))?;
    Ok(receiver)
}

pub(crate) fn formatted_record(py: Python<'_>, xml: String, format: &str) -> PyResult<Py<PyAny>> {
    validate_format(format)?;
    let record = EventRecord::from_xml(xml.clone())
        .map_err(|error| PyValueError::new_err(format!("could not parse event XML: {error}")))?;
    match format {
        "record" => Py::new(py, record).map(Py::into_any),
        "dict" => Ok(pythonize::pythonize(py, &record.to_value())?.unbind()),
        "xml" => Ok(pythonize::pythonize(py, &Value::String(xml))?.unbind()),
        "json" => serde_json::to_string_pretty(&record.to_value())
            .map_err(|error| PyValueError::new_err(error.to_string()))
            .and_then(|json| {
                pythonize::pythonize(py, &Value::String(json))
                    .map(Bound::unbind)
                    .map_err(|error| PyValueError::new_err(error.to_string()))
            }),
        _ => unreachable!(),
    }
}

impl EvtxRecordIterator {
    fn next_formatted(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let receiver = Arc::clone(&self.records);
        let result = py.detach(move || {
            receiver
                .lock()
                .map_err(|_| "EVTX iterator lock was poisoned".to_owned())
                .map(|receiver| receiver.recv())
        });
        match result {
            Ok(Ok(Ok(xml))) => formatted_record(py, xml, &self.format).map(Some),
            Ok(Ok(Err(error))) => Err(PyRuntimeError::new_err(format!(
                "EVTX record parsing failed: {error}"
            ))),
            Ok(Err(_)) => Ok(None),
            Err(error) => Err(PyRuntimeError::new_err(error)),
        }
    }
}

#[pymethods]
impl EvtxRecordIterator {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.next_formatted(py)
    }
}

#[pymethods]
impl EvtxFile {
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        if !path.exists() {
            return Err(PyFileNotFoundError::new_err(format!(
                "EVTX file not found: {}",
                path.display()
            )));
        }
        evtx::EvtxParser::from_path(&path)
            .map_err(|error| PyValueError::new_err(format!("invalid EVTX file: {error}")))?;
        Ok(Self { path })
    }

    #[pyo3(signature = (limit=None, format="record"))]
    fn read(&self, py: Python<'_>, limit: Option<usize>, format: &str) -> PyResult<Vec<Py<PyAny>>> {
        read_path(py, &self.path, limit, format)
    }

    #[pyo3(signature = (format="record"))]
    fn iter(&self, format: &str) -> PyResult<EvtxRecordIterator> {
        iter_path(&self.path, format)
    }

    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __exit__(
        &self,
        _exception_type: Option<&Bound<'_, PyAny>>,
        _exception: Option<&Bound<'_, PyAny>>,
        _traceback: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        false
    }
}

#[pyfunction]
#[pyo3(signature = (path, limit=None, format="record"))]
pub fn read_evtx(
    py: Python<'_>,
    path: PathBuf,
    limit: Option<usize>,
    format: &str,
) -> PyResult<Vec<Py<PyAny>>> {
    read_path(py, &path, limit, format)
}

#[pyfunction]
#[pyo3(signature = (path, format="record"))]
pub fn iter_evtx(path: PathBuf, format: &str) -> PyResult<EvtxRecordIterator> {
    iter_path(&path, format)
}

fn iter_path(path: &Path, format: &str) -> PyResult<EvtxRecordIterator> {
    validate_format(format)?;
    let records = open_records(path)?;
    Ok(EvtxRecordIterator {
        records: Arc::new(Mutex::new(records)),
        format: format.to_owned(),
    })
}

fn read_path(
    py: Python<'_>,
    path: &Path,
    limit: Option<usize>,
    format: &str,
) -> PyResult<Vec<Py<PyAny>>> {
    validate_format(format)?;
    let mut iterator = iter_path(path, format)?;
    let mut records = Vec::new();
    while limit.is_none_or(|maximum| records.len() < maximum) {
        match iterator.next_formatted(py)? {
            Some(record) => records.push(record),
            None => break,
        }
    }
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::validate_format;

    #[test]
    fn accepts_supported_formats() {
        for format in ["record", "dict", "xml", "json"] {
            assert!(validate_format(format).is_ok());
        }
    }

    #[test]
    fn rejects_unknown_formats() {
        assert!(validate_format("yaml").is_err());
    }
}

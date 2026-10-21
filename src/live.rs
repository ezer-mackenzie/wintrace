#[cfg(windows)]
use crate::offline::formatted_record;
use crate::offline::validate_format;
#[cfg(not(windows))]
use pyo3::exceptions::PyNotImplementedError;
#[cfg(windows)]
use pyo3::exceptions::PyRuntimeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyclass(module = "wintrace")]
pub struct LiveEventIterator {
    #[cfg(windows)]
    query_handle: EvtHandle,
    #[cfg(windows)]
    finished: bool,
    #[cfg(windows)]
    format: String,
}

#[pyclass(module = "wintrace")]
pub struct LiveEventLog {
    channel: String,
    query: String,
    reverse: bool,
}

#[pymethods]
impl LiveEventIterator {
    fn __iter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        #[cfg(windows)]
        {
            if self.finished {
                return Ok(None);
            }
            let query_handle = self.query_handle.0;
            let (succeeded, event_handle, returned) = py.detach(|| {
                const EVENT_READ_TIMEOUT_MS: u32 = 1000;
                let mut event_handle = 0isize;
                let mut returned = 0u32;
                let succeeded = unsafe {
                    windows_sys::Win32::System::EventLog::EvtNext(
                        query_handle,
                        1,
                        &mut event_handle,
                        EVENT_READ_TIMEOUT_MS,
                        0,
                        &mut returned,
                    )
                };
                (succeeded, event_handle, returned)
            });
            if succeeded == 0 {
                let error = unsafe { windows_sys::Win32::Foundation::GetLastError() };
                if error == windows_sys::Win32::Foundation::ERROR_NO_MORE_ITEMS
                    || error == windows_sys::Win32::Foundation::ERROR_TIMEOUT
                {
                    self.finished = true;
                    return Ok(None);
                }
                return Err(PyRuntimeError::new_err(format!(
                    "Windows Event Log query failed with error {error}"
                )));
            }
            if returned == 0 || event_handle == 0 {
                self.finished = true;
                return Ok(None);
            }
            let event = EvtHandle(event_handle);
            let xml = render_event_xml(event.0)?;
            formatted_record(py, xml, &self.format).map(Some)
        }
        #[cfg(not(windows))]
        {
            let _ = py;
            Err(PyNotImplementedError::new_err(
                "live Windows Event Log queries are only available on Windows",
            ))
        }
    }
}

#[pymethods]
impl LiveEventLog {
    #[new]
    #[pyo3(signature = (channel="System", query="*", reverse=true))]
    fn new(channel: &str, query: &str, reverse: bool) -> PyResult<Self> {
        validate_query(channel, query)?;
        Ok(Self {
            channel: channel.to_owned(),
            query: query.to_owned(),
            reverse,
        })
    }

    #[pyo3(signature = (limit=None, format="record"))]
    fn read(&self, py: Python<'_>, limit: Option<usize>, format: &str) -> PyResult<Vec<Py<PyAny>>> {
        query_events(py, &self.channel, &self.query, limit, self.reverse, format)
    }

    #[pyo3(signature = (format="record"))]
    fn iter(&self, format: &str) -> PyResult<LiveEventIterator> {
        iter_events(&self.channel, &self.query, self.reverse, format)
    }
}

#[pyfunction]
#[pyo3(signature = (channel="System", query="*", limit=None, reverse=true, format="record"))]
pub fn query_events(
    py: Python<'_>,
    channel: &str,
    query: &str,
    limit: Option<usize>,
    reverse: bool,
    format: &str,
) -> PyResult<Vec<Py<PyAny>>> {
    validate_format(format)?;
    let mut iterator = iter_events(channel, query, reverse, format)?;
    let mut records = Vec::new();
    while limit.is_none_or(|maximum| records.len() < maximum) {
        match iterator.__next__(py)? {
            Some(record) => records.push(record),
            None => break,
        }
    }
    Ok(records)
}

#[pyfunction]
#[pyo3(signature = (channel="System", query="*", reverse=true, format="record"))]
pub fn iter_events(
    channel: &str,
    query: &str,
    reverse: bool,
    format: &str,
) -> PyResult<LiveEventIterator> {
    validate_format(format)?;
    validate_query(channel, query)?;
    #[cfg(windows)]
    {
        let channel_wide = to_wide(channel);
        let query_wide = to_wide(query);
        const EVT_QUERY_CHANNEL_PATH: u32 = 0x1;
        const EVT_QUERY_FORWARD_DIRECTION: u32 = 0x100;
        const EVT_QUERY_REVERSE_DIRECTION: u32 = 0x200;
        let direction = if reverse {
            EVT_QUERY_REVERSE_DIRECTION
        } else {
            EVT_QUERY_FORWARD_DIRECTION
        };
        let flags = EVT_QUERY_CHANNEL_PATH | direction;
        let handle = unsafe {
            windows_sys::Win32::System::EventLog::EvtQuery(
                0,
                channel_wide.as_ptr(),
                query_wide.as_ptr(),
                flags,
            )
        };
        if handle == 0 {
            let error = unsafe { windows_sys::Win32::Foundation::GetLastError() };
            return Err(PyRuntimeError::new_err(format!(
                "could not query Windows Event Log channel {channel:?} (error {error})"
            )));
        }
        Ok(LiveEventIterator {
            query_handle: EvtHandle(handle),
            finished: false,
            format: format.to_owned(),
        })
    }
    #[cfg(not(windows))]
    {
        let _ = reverse;
        Err(PyNotImplementedError::new_err(
            "live Windows Event Log queries are only available on Windows",
        ))
    }
}

fn validate_query(channel: &str, query: &str) -> PyResult<()> {
    if channel.trim().is_empty() {
        return Err(PyValueError::new_err("channel must not be empty"));
    }
    if query.trim().is_empty() {
        return Err(PyValueError::new_err("query must not be empty"));
    }
    Ok(())
}

#[cfg(windows)]
struct EvtHandle(windows_sys::Win32::System::EventLog::EVT_HANDLE);

#[cfg(windows)]
unsafe impl Send for EvtHandle {}

#[cfg(windows)]
impl Drop for EvtHandle {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::System::EventLog::EvtClose(self.0);
        }
    }
}

#[cfg(windows)]
fn to_wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(windows)]
fn render_event_xml(
    event_handle: windows_sys::Win32::System::EventLog::EVT_HANDLE,
) -> PyResult<String> {
    use std::ffi::c_void;
    use windows_sys::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
    use windows_sys::Win32::System::EventLog::EvtRender;

    let mut buffer_size = 0u32;
    let mut property_count = 0u32;
    const EVT_RENDER_EVENT_XML: u32 = 0x1;
    let first_call = unsafe {
        EvtRender(
            0,
            event_handle,
            EVT_RENDER_EVENT_XML,
            0,
            std::ptr::null_mut(),
            &mut buffer_size,
            &mut property_count,
        )
    };
    if first_call != 0 {
        return Err(PyRuntimeError::new_err(
            "Windows Event Log returned an unexpectedly empty XML buffer",
        ));
    }
    let error = unsafe { windows_sys::Win32::Foundation::GetLastError() };
    if error != ERROR_INSUFFICIENT_BUFFER || buffer_size == 0 {
        return Err(PyRuntimeError::new_err(format!(
            "could not size rendered event XML (error {error})"
        )));
    }
    let mut buffer = vec![0u16; (buffer_size as usize).div_ceil(2)];
    let succeeded = unsafe {
        EvtRender(
            0,
            event_handle,
            EVT_RENDER_EVENT_XML,
            buffer_size,
            buffer.as_mut_ptr().cast::<c_void>(),
            &mut buffer_size,
            &mut property_count,
        )
    };
    if succeeded == 0 {
        let error = unsafe { windows_sys::Win32::Foundation::GetLastError() };
        return Err(PyRuntimeError::new_err(format!(
            "could not render Windows event XML (error {error})"
        )));
    }
    let units = (buffer_size as usize / 2).min(buffer.len());
    let xml = String::from_utf16_lossy(&buffer[..units]);
    Ok(xml.trim_end_matches('\0').to_owned())
}

#[cfg(test)]
mod tests {
    use super::validate_query;

    #[test]
    fn rejects_empty_channel_and_query() {
        assert!(validate_query("", "*").is_err());
        assert!(validate_query("System", "").is_err());
    }
}

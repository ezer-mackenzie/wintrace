use crate::xml::parse_event_xml;
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use serde_json::Value;

#[pyclass(module = "wintrace")]
#[derive(Debug)]
pub struct EventRecord {
    #[pyo3(get)]
    pub xml: String,
    value: Value,
    #[pyo3(get)]
    pub event_id: Option<u64>,
    #[pyo3(get)]
    pub record_id: Option<u64>,
    #[pyo3(get)]
    pub channel: Option<String>,
    #[pyo3(get)]
    pub provider: Option<String>,
    #[pyo3(get)]
    pub level: Option<u64>,
    #[pyo3(get)]
    pub time_created: Option<String>,
    #[pyo3(get)]
    pub computer: Option<String>,
    system: Option<Value>,
    event_data: Option<Value>,
}

impl EventRecord {
    pub fn from_xml(xml: String) -> Result<Self, String> {
        let value = parse_event_xml(&xml)?;
        let system = value.get("Event").and_then(|event| event.get("System"));
        let event = value.get("Event");
        let event_data = event.and_then(|event| {
            event
                .get("EventData")
                .map(|data| event_data_value(data).unwrap_or_else(|| data.clone()))
                .or_else(|| event.get("UserData").cloned())
        });
        Ok(Self {
            xml,
            event_id: system
                .and_then(|system| system.get("EventID"))
                .and_then(value_as_u64),
            record_id: system
                .and_then(|system| system.get("EventRecordID"))
                .and_then(value_as_u64),
            channel: system
                .and_then(|system| system.get("Channel"))
                .and_then(value_as_string),
            provider: system
                .and_then(|system| system.get("Provider"))
                .and_then(|provider| provider.get("@Name").or(Some(provider)))
                .and_then(value_as_string),
            level: system
                .and_then(|system| system.get("Level"))
                .and_then(value_as_u64),
            time_created: system
                .and_then(|system| system.get("TimeCreated"))
                .and_then(|created| created.get("@SystemTime").or(Some(created)))
                .and_then(value_as_string),
            computer: system
                .and_then(|system| system.get("Computer"))
                .and_then(value_as_string),
            system: system.cloned(),
            event_data,
            value,
        })
    }

    fn event_root(&self) -> Option<&Value> {
        self.value.get("Event")
    }

    pub(crate) fn to_value(&self) -> Value {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::EventRecord;
    use serde_json::json;

    #[test]
    fn extracts_event_metadata_and_named_data() {
        let xml = r#"<Event><System><Provider Name="Sample"/><EventID>4624</EventID><EventRecordID>19</EventRecordID><Channel>Security</Channel><Level>4</Level><TimeCreated SystemTime="2026-01-02T03:04:05Z"/><Computer>host</Computer></System><EventData><Data Name="User">alice</Data></EventData></Event>"#;
        let record = EventRecord::from_xml(xml.to_owned()).expect("event XML should parse");
        assert_eq!(record.event_id, Some(4624));
        assert_eq!(record.record_id, Some(19));
        assert_eq!(record.channel.as_deref(), Some("Security"));
        assert_eq!(record.provider.as_deref(), Some("Sample"));
        assert_eq!(record.time_created.as_deref(), Some("2026-01-02T03:04:05Z"));
        assert_eq!(record.event_data, Some(json!({"User": "alice"})));
    }

    #[test]
    fn retains_user_data_sections() {
        let xml =
            r#"<Event><System/><UserData><Logon><User>alice</User></Logon></UserData></Event>"#;
        let record = EventRecord::from_xml(xml.to_owned()).expect("event XML should parse");
        assert_eq!(record.event_data, Some(json!({"Logon": {"User": "alice"}})));
    }

    #[test]
    fn preserves_empty_and_unnamed_event_data() {
        let xml = r#"<Event><System/><EventData><Data Name="Empty"/><Data>raw</Data></EventData></Event>"#;
        let record = EventRecord::from_xml(xml.to_owned()).expect("event XML should parse");
        assert_eq!(
            record.event_data,
            Some(json!({"Empty": "", "@unnamed": ["raw"]}))
        );
    }

    #[test]
    fn preserves_repeated_named_event_data() {
        let xml = r#"<Event><System/><EventData><Data Name="User">alice</Data><Data Name="User">bob</Data></EventData></Event>"#;
        let record = EventRecord::from_xml(xml.to_owned()).expect("event XML should parse");
        assert_eq!(record.event_data, Some(json!({"User": ["alice", "bob"]})));
    }
}

fn value_as_u64(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|text| text.trim().parse().ok()))
        .or_else(|| value.get("#text").and_then(value_as_u64))
}

fn value_as_string(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| value.as_u64().map(|number| number.to_string()))
        .or_else(|| value.get("#text").and_then(value_as_string))
}

fn event_data_value(event_data: &Value) -> Option<Value> {
    let data = event_data.get("Data")?;
    let entries = match data {
        Value::Array(values) => values.as_slice(),
        value => std::slice::from_ref(value),
    };
    let mut named = serde_json::Map::new();
    let mut unnamed = Vec::new();
    for item in entries {
        let value = item
            .get("#text")
            .cloned()
            .unwrap_or_else(|| match item.get("@Name") {
                Some(_) => Value::String(String::new()),
                None => item.clone(),
            });
        match item.get("@Name").and_then(Value::as_str) {
            Some(name) => insert_named_value(&mut named, name, value),
            None => unnamed.push(value),
        }
    }
    if !named.is_empty() {
        if !unnamed.is_empty() {
            named.insert("@unnamed".to_owned(), Value::Array(unnamed));
        }
        Some(Value::Object(named))
    } else if unnamed.len() == 1 {
        unnamed.pop()
    } else {
        Some(Value::Array(unnamed))
    }
}

fn insert_named_value(object: &mut serde_json::Map<String, Value>, name: &str, value: Value) {
    match object.get_mut(name) {
        Some(Value::Array(values)) => values.push(value),
        Some(previous) => {
            let first = std::mem::replace(previous, Value::Null);
            *previous = Value::Array(vec![first, value]);
        }
        None => {
            object.insert(name.to_owned(), value);
        }
    }
}

#[pymethods]
impl EventRecord {
    #[getter]
    fn system(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        Ok(self
            .system
            .as_ref()
            .map(|value| pythonize::pythonize(py, value).map(Bound::unbind))
            .transpose()?)
    }

    #[getter]
    fn event_data(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        Ok(self
            .event_data
            .as_ref()
            .map(|value| pythonize::pythonize(py, value).map(Bound::unbind))
            .transpose()?)
    }

    fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(pythonize::pythonize(py, &self.value)?.unbind())
    }

    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.value)
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    #[pyo3(signature = (key, default=None))]
    fn get(&self, py: Python<'_>, key: &str, default: Option<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        let value = self
            .event_root()
            .and_then(|event| event.get(key))
            .or_else(|| self.value.get(key));
        match value {
            Some(value) => Ok(pythonize::pythonize(py, value)?.unbind()),
            None => Ok(default.unwrap_or_else(|| py.None())),
        }
    }

    fn __getitem__(&self, py: Python<'_>, key: &str) -> PyResult<Py<PyAny>> {
        let value = self
            .event_root()
            .and_then(|event| event.get(key))
            .or_else(|| self.value.get(key))
            .ok_or_else(|| PyKeyError::new_err(key.to_owned()))?;
        Ok(pythonize::pythonize(py, value)?.unbind())
    }

    fn __repr__(&self) -> String {
        format!(
            "EventRecord(event_id={:?}, channel={:?}, provider={:?})",
            self.event_id, self.channel, self.provider
        )
    }
}

use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};

use crate::entity::{DivData, DivDataState, Entity};
use crate::field::FieldDescriptor;
use crate::value::DivValue;

use super::py_value::{json_to_py, py_to_divvalue};

/// Metadata for a registered entity type.
#[derive(Clone)]
pub struct EntityTypeMeta {
    pub type_name: Option<String>,
    pub class_name: String,
    pub field_names: Vec<String>,
    pub required_fields: Vec<String>,
}

/// Generic Python wrapper for any DivKit entity.
#[pyclass(subclass)]
#[derive(Clone)]
pub struct PyDivEntity {
    pub type_meta: EntityTypeMeta,
    pub fields: HashMap<String, DivValue>,
}

impl PyDivEntity {
    pub fn to_rust_entity(&self) -> Box<dyn Entity> {
        Box::new(DynamicEntity {
            type_name: self.type_meta.type_name.clone(),
            field_names: self.type_meta.field_names.clone(),
            required_fields: self.type_meta.required_fields.clone(),
            fields: self.fields.clone(),
        })
    }
}

#[pymethods]
impl PyDivEntity {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new_py(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let mut fields = HashMap::new();
        if let Some(kw) = kwargs {
            for (k, v) in kw.iter() {
                let key: String = k.extract()?;
                fields.insert(key, py_to_divvalue(&v)?);
            }
        }
        Ok(PyDivEntity {
            type_meta: EntityTypeMeta {
                type_name: None,
                class_name: "PyDivEntity".to_string(),
                field_names: vec![],
                required_fields: vec![],
            },
            fields,
        })
    }

    fn dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let entity = self.to_rust_entity();
        let json_val = entity.dict();
        json_to_py(py, &json_val)
    }

    fn build(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.dict(py)
    }

    #[pyo3(signature = (exclude_fields=None))]
    fn schema(&self, py: Python<'_>, exclude_fields: Option<Vec<String>>) -> PyResult<PyObject> {
        let entity = self.to_rust_entity();
        let exclude_refs: Option<Vec<&str>> =
            exclude_fields.as_ref().map(|v| v.iter().map(|s| s.as_str()).collect());
        let json_val = entity.schema(exclude_refs.as_deref());
        json_to_py(py, &json_val)
    }

    fn __repr__(&self) -> String {
        let fields: Vec<String> = self
            .fields
            .iter()
            .map(|(k, v)| format!("{}={:?}", k, v))
            .collect();
        format!("{}({})", self.type_meta.class_name, fields.join(", "))
    }
}

/// A dynamic Entity constructed from a HashMap at runtime.
#[derive(Debug, Clone)]
pub struct DynamicEntity {
    pub type_name: Option<String>,
    pub field_names: Vec<String>,
    pub required_fields: Vec<String>,
    pub fields: HashMap<String, DivValue>,
}

// Safety: DivValue is Send+Sync since Entity trait now requires it
unsafe impl Send for DynamicEntity {}
unsafe impl Sync for DynamicEntity {}

impl Entity for DynamicEntity {
    fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }

    fn field_descriptors(&self) -> Vec<FieldDescriptor> {
        let mut descs = Vec::new();
        if let Some(ref tn) = self.type_name {
            descs.push(
                FieldDescriptor::new("type")
                    .with_default(serde_json::Value::String(tn.clone()))
                    .required(),
            );
        }
        for name in &self.field_names {
            let mut d = FieldDescriptor::new(name);
            if self.required_fields.contains(name) {
                d = d.required();
            }
            descs.push(d);
        }
        descs
    }

    fn field_values(&self) -> Vec<(String, DivValue)> {
        let mut values = Vec::new();
        if let Some(ref tn) = self.type_name {
            values.push(("type".to_string(), DivValue::String(tn.clone())));
        }
        for (name, val) in &self.fields {
            if !val.is_null() {
                values.push((name.clone(), val.clone()));
            }
        }
        values
    }
}

/// Register a factory function for an entity type on the Python module.
pub fn register_entity_class(
    py: Python<'_>,
    module: &Bound<'_, pyo3::types::PyModule>,
    class_name: &str,
    type_name: Option<&str>,
    field_names: &[&str],
    required_fields: &[&str],
) -> PyResult<()> {
    let meta = EntityTypeMeta {
        type_name: type_name.map(|s| s.to_string()),
        class_name: class_name.to_string(),
        field_names: field_names.iter().map(|s| s.to_string()).collect(),
        required_fields: required_fields.iter().map(|s| s.to_string()).collect(),
    };

    // Create a closure that produces PyDivEntity with the right meta
    let constructor = pyo3::types::PyCFunction::new_closure_bound(
        py,
        None,
        None,
        move |args: &Bound<'_, PyTuple>, kwargs: Option<&Bound<'_, PyDict>>| -> PyResult<PyDivEntity> {
            let _ = args; // no positional args
            let mut fields = HashMap::new();
            if let Some(kw) = kwargs {
                for (k, v) in kw.iter() {
                    let key: String = k.extract()?;
                    fields.insert(key, py_to_divvalue(&v)?);
                }
            }
            Ok(PyDivEntity {
                type_meta: meta.clone(),
                fields,
            })
        },
    )?;

    module.setattr(class_name, constructor)?;
    Ok(())
}

/// Python wrapper for DivData.
#[pyclass]
pub struct PyDivData {
    pub log_id: String,
    pub states: Vec<PyDivDataState>,
}

#[derive(Clone)]
#[pyclass]
pub struct PyDivDataState {
    pub state_id: i64,
    pub div: PyDivEntity,
}

#[pymethods]
impl PyDivData {
    #[new]
    #[pyo3(signature = (log_id, states))]
    fn new_py(log_id: String, states: Vec<PyDivDataState>) -> Self {
        PyDivData { log_id, states }
    }

    fn dict(&self, py: Python<'_>) -> PyResult<PyObject> {
        let rust_states: Vec<DivDataState> = self
            .states
            .iter()
            .map(|s| DivDataState {
                state_id: s.state_id,
                div: s.div.to_rust_entity(),
            })
            .collect();
        let data = DivData {
            log_id: self.log_id.clone(),
            states: rust_states,
        };
        let json_val = <DivData as Entity>::dict(&data);
        json_to_py(py, &json_val)
    }

    fn build(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.dict(py)
    }
}

#[pymethods]
impl PyDivDataState {
    #[new]
    #[pyo3(signature = (state_id, div))]
    fn new_py(state_id: i64, div: PyDivEntity) -> Self {
        PyDivDataState { state_id, div }
    }
}

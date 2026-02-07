use std::collections::HashMap;

use serde_json::Value;

/// Constraints on a field, mirroring Python's Field constraints.
#[derive(Debug, Clone, Default)]
pub struct Constraints {
    pub format: Option<String>,
    pub exclusive_minimum: Option<f64>,
    pub minimum: Option<f64>,
    pub exclusive_maximum: Option<f64>,
    pub maximum: Option<f64>,
    pub multiple_of: Option<f64>,
    pub min_items: Option<usize>,
    pub max_items: Option<usize>,
    pub unique_items: Option<bool>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub regex: Option<String>,
}

impl Constraints {
    pub fn to_json_map(&self) -> HashMap<String, Value> {
        let mut m = HashMap::new();
        if let Some(ref f) = self.format {
            m.insert("format".into(), Value::String(f.clone()));
        }
        if let Some(v) = self.exclusive_minimum {
            m.insert("exclusiveMinimum".into(), serde_json::json!(v));
        }
        if let Some(v) = self.minimum {
            m.insert("minimum".into(), serde_json::json!(v));
        }
        if let Some(v) = self.exclusive_maximum {
            m.insert("exclusiveMaximum".into(), serde_json::json!(v));
        }
        if let Some(v) = self.maximum {
            m.insert("maximum".into(), serde_json::json!(v));
        }
        if let Some(v) = self.multiple_of {
            m.insert("multipleOf".into(), serde_json::json!(v));
        }
        if let Some(v) = self.min_items {
            m.insert("minItems".into(), serde_json::json!(v));
        }
        if let Some(v) = self.max_items {
            m.insert("maxItems".into(), serde_json::json!(v));
        }
        if let Some(v) = self.unique_items {
            m.insert("uniqueItems".into(), serde_json::json!(v));
        }
        if let Some(v) = self.min_length {
            m.insert("minLength".into(), serde_json::json!(v));
        }
        if let Some(v) = self.max_length {
            m.insert("maxLength".into(), serde_json::json!(v));
        }
        if let Some(ref v) = self.regex {
            m.insert("regex".into(), Value::String(v.clone()));
        }
        m
    }

    pub fn is_empty(&self) -> bool {
        self.format.is_none()
            && self.exclusive_minimum.is_none()
            && self.minimum.is_none()
            && self.exclusive_maximum.is_none()
            && self.maximum.is_none()
            && self.multiple_of.is_none()
            && self.min_items.is_none()
            && self.max_items.is_none()
            && self.unique_items.is_none()
            && self.min_length.is_none()
            && self.max_length.is_none()
            && self.regex.is_none()
    }
}

/// Field metadata descriptor.
/// Mirrors Python's `_Field` / `Field()`.
#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    pub name: String,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub constraints: Constraints,
    pub required: bool,
}

impl FieldDescriptor {
    pub fn new(name: &str) -> Self {
        FieldDescriptor {
            name: name.to_string(),
            description: None,
            default: None,
            constraints: Constraints::default(),
            required: false,
        }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_constraints(mut self, c: Constraints) -> Self {
        self.constraints = c;
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

/// Builder for creating a FieldDescriptor fluently.
/// Mirrors Python's `Field(name=..., description=..., default=..., ...)`.
pub struct FieldBuilder {
    name: Option<String>,
    description: Option<String>,
    default: Option<Value>,
    constraints: Constraints,
}

impl FieldBuilder {
    pub fn new() -> Self {
        FieldBuilder {
            name: None,
            description: None,
            default: None,
            constraints: Constraints::default(),
        }
    }

    pub fn name(mut self, n: &str) -> Self {
        self.name = Some(n.to_string());
        self
    }

    pub fn description(mut self, d: &str) -> Self {
        self.description = Some(d.to_string());
        self
    }

    pub fn default_value(mut self, v: Value) -> Self {
        self.default = Some(v);
        self
    }

    pub fn format(mut self, f: &str) -> Self {
        self.constraints.format = Some(f.to_string());
        self
    }

    pub fn gt(mut self, v: f64) -> Self {
        self.constraints.exclusive_minimum = Some(v);
        self
    }

    pub fn ge(mut self, v: f64) -> Self {
        self.constraints.minimum = Some(v);
        self
    }

    pub fn lt(mut self, v: f64) -> Self {
        self.constraints.exclusive_maximum = Some(v);
        self
    }

    pub fn le(mut self, v: f64) -> Self {
        self.constraints.maximum = Some(v);
        self
    }

    pub fn multiple_of(mut self, v: f64) -> Self {
        self.constraints.multiple_of = Some(v);
        self
    }

    pub fn min_items(mut self, v: usize) -> Self {
        self.constraints.min_items = Some(v);
        self
    }

    pub fn max_items(mut self, v: usize) -> Self {
        self.constraints.max_items = Some(v);
        self
    }

    pub fn unique_items(mut self, v: bool) -> Self {
        self.constraints.unique_items = Some(v);
        self
    }

    pub fn min_length(mut self, v: usize) -> Self {
        self.constraints.min_length = Some(v);
        self
    }

    pub fn max_length(mut self, v: usize) -> Self {
        self.constraints.max_length = Some(v);
        self
    }

    pub fn regex(mut self, v: &str) -> Self {
        self.constraints.regex = Some(v.to_string());
        self
    }

    pub fn build(self, field_name: &str) -> FieldDescriptor {
        FieldDescriptor {
            name: self.name.unwrap_or_else(|| field_name.to_string()),
            description: self.description,
            default: self.default,
            constraints: self.constraints,
            required: false,
        }
    }
}

impl Default for FieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

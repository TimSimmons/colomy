use std::collections::HashMap;
use std::fmt;

use serde_json::map::Map;
use serde_json::Value;

#[derive(Debug)]
pub struct Event {
    pub fields: HashMap<String, Value>,
}

impl Event {
    pub fn new(data: &str) -> Result<Event, EventError> {
        let v: Value = serde_json::from_str(data)?;

        let map = match v {
            Value::Object(o) => o,
            _ => {
                return Err(EventError::new(format!(
                    "looking for JSON map, got something else {:?}",
                    v
                )))
            }
        };

        let fields = get_values(String::from(""), map);

        Ok(Event { fields })
    }
}

#[derive(Debug)]
pub struct EventError {
    details: String,
}

impl EventError {
    fn new(msg: String) -> EventError {
        EventError { details: msg }
    }
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<serde_json::error::Error> for EventError {
    fn from(err: serde_json::error::Error) -> Self {
        EventError::new(err.to_string())
    }
}

fn get_values(prefix: String, m: Map<String, Value>) -> HashMap<String, Value> {
    let mut fields = HashMap::new();

    for (k, v) in m.iter() {
        let v_string = v.to_string();
        match v {
            Value::Null => {
                fields.insert(format!("{}{}", prefix, k), Value::Null);
            }
            Value::Bool(b) => {
                fields.insert(format!("{}{}", prefix, k), Value::Bool(*b));
            }
            Value::Number(n) => {
                fields.insert(format!("{}{}", prefix, k), Value::Number(n.clone()));
            }
            Value::String(s) => {
                fields.insert(format!("{}{}", prefix, k), Value::String(s.to_string()));
            }
            Value::Array(_a) => {
                fields.insert(format!("{}{}", prefix, k), Value::String(v_string));
            }
            Value::Object(o) => {
                let subfields = get_values(format!("{}{}.", prefix, k), o.clone());
                for (sk, sv) in subfields.iter() {
                    fields.insert(sk.to_string(), sv.clone());
                }
            }
        }
    }

    return fields;
}

use std::collections::HashMap;
use std::fmt;

use serde_json::map::Map;
use serde_json::Value;

#[derive(Debug)]
pub struct Event {
    // TODO: generate (sequential) ID
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

        let fields = Event::get_values(String::from(""), map);

        Ok(Event { fields })
    }

    pub fn get_values(prefix: String, m: Map<String, Value>) -> HashMap<String, Value> {
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
                    let subfields = Event::get_values(format!("{}{}.", prefix, k), o.clone());
                    for (sk, sv) in subfields.iter() {
                        fields.insert(sk.to_string(), sv.clone());
                    }
                }
            }
        }

        return fields;
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use serde_json::Value;

    #[test]
    fn parse_event() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "alive": true,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "job": {
                "type": "engineer",
                "firm": "Grunnings"
            }
        }"#;
        let e = Event::new(data).unwrap();
        assert_eq!(json!("John Doe"), e.fields["name"]);
        assert_eq!(json!(true), e.fields["alive"]);
        assert_eq!(json!(43), e.fields["age"]);
        assert_eq!(
            Value::String(r#"["+44 1234567","+44 2345678"]"#.to_string()),
            e.fields["phones"]
        );
        assert_eq!(json!("Grunnings"), e.fields["job.firm"]);
        assert_eq!(json!("engineer"), e.fields["job.type"]);

        for (k, v) in &e.fields {
            match v {
                Value::Bool(_b) => (),
                Value::Number(_n) => (),
                Value::String(_s) => (),
                _ => panic!("unknown value key: {} value {:?}", k, v),
            }
        }
    }

    #[test]
    fn parse_invalid_json() {
        let baddata = r#"
        [{
            "name": "John Doe",
            "age": 43
        }]"#;
        let err = Event::new(baddata);
        match err {
            Err(_) => (),
            Ok(_) => panic!("invalid json didn't return an error!"),
        }
    }
}

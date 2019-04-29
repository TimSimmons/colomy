use std::collections::HashMap;

use serde_json::map::Map;
use serde_json::Value;

fn main() {
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

    let v: Value = serde_json::from_str(data).unwrap();
    println!("{}", v);
    let fields = match v {
        Value::Null => panic!("looking for JSON map, got: null"),
        Value::Bool(b) => panic!("looking for JSON map, got: bool {}", b),
        Value::Number(n) => panic!("looking for JSON map, got: number {}", n),
        Value::String(s) => panic!("looking for JSON map, got: string {}", s),
        Value::Array(a) => panic!("looking for JSON map, got: array {:?}", a),
        Value::Object(o) => get_values(String::from(""), o),
    };
    println!("{:?}", fields);
    for (k, v) in fields {
        match v {
            Value::Bool(b) => println!("key {}  value {}", k, b),
            Value::Number(n) => println!("key {}  value {}", k, n),
            Value::String(s) => println!("key {}  value {}", k, s),
            _ => panic!("unknown value key: {} value {:?}", k, v),
        }
    }
}

fn get_values(prefix: String, m: Map<String, Value>) -> HashMap<String, Value> {
    let mut fields = HashMap::new();

    for (k, v) in m.iter() {
        // println!("key: {}{}   value {:?}", prefix, k, v);
        let v_string = v.to_string();
        match v {
            Value::Null => {
                // println!("null");
                fields.insert(format!("{}{}", prefix, k), Value::Null);
            }
            Value::Bool(b) => {
                // println!("bool {}", b);
                fields.insert(format!("{}{}", prefix, k), Value::Bool(*b));
            }
            Value::Number(n) => {
                // println!("number {}", n);
                fields.insert(format!("{}{}", prefix, k), Value::Number(n.clone()));
            }
            Value::String(s) => {
                // println!("string {}", s);
                fields.insert(format!("{}{}", prefix, k), Value::String(s.to_string()));
            }
            Value::Array(_a) => {
                // println!("array {:?}", a);
                fields.insert(format!("{}{}", prefix, k), Value::String(v_string));
            }
            Value::Object(o) => {
                // println!("map {:?}", o);
                let subfields = get_values(format!("{}{}.", prefix, k), o.clone());
                for (sk, sv) in subfields.iter() {
                    fields.insert(sk.to_string(), sv.clone());
                }
            }
        }
    }

    return fields;
}

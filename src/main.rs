extern crate colomy;

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
    let e = colomy::event::Event::new(data).unwrap();
    println!("{:#?}", e);
    for (k, v) in e.fields {
        match v {
            Value::Bool(b) => println!("key {}  value {}", k, b),
            Value::Number(n) => println!("key {}  value {}", k, n),
            Value::String(s) => println!("key {}  value {}", k, s),
            _ => panic!("unknown value key: {} value {:?}", k, v),
        }
    }
}

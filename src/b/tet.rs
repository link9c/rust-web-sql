use serde_json::{Result, Map, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
struct Info {
    name: String,
    info: Vec<HashMap<String, String>>,
}

fn main() -> Result<()> {
    // let d = HashMap::new()
    let json = r#"
  {
    "name": "琼台博客",
    "info": [{"age":"20",“time":"now"},{"age":"20",“time":"now"}]

  }"#;

    let json2 = r#"{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}"#;

    let mut ss = Info {
        name: "nn".to_string(),
        info: vec![],
    };

    let mut h = HashMap::new();
    h.insert("age".to_string(),"209".to_string());
    h.insert("time".to_string(),"now".to_string());

    ss.info.push(h.clone());
    ss.info.push(h.clone());

    let v = serde_json::to_string(&ss)?;
    // let u: Info = serde_json::from_str(json)?;
    println!("name = {:?}", v);

    Ok(())
}
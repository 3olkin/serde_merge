use serde::{Deserialize, Serialize};
use serde_json;
use serde_merge::*;
use std::collections::HashMap;

#[test]
pub fn test_mmerge() {
    let mut map1: HashMap<&str, &str> = HashMap::new();
    map1.insert("key1", "value1");
    map1.insert("key2", "value2");
    let mut map2: HashMap<&str, &str> = HashMap::new();
    map2.insert("key2", "value2_2");
    map2.insert("key3", "value3");
    let result = mmerge(map1, map2).unwrap();
    let mut target: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    let mut add_entry = |k: &str, v: &str| {
        target.insert(k.to_string(), serde_json::Value::String(v.to_string()));
    };
    add_entry("key1", "value1");
    add_entry("key2", "value2_2");
    add_entry("key3", "value3");
    assert_eq!(result, target);
}

#[derive(Debug, Serialize, Deserialize)]
struct Foo {
    pub field1: String,
    pub field2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bar {
    pub field2: i32,
    pub field3: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FooBar {
    pub field1: String,
    pub field2: i32,
    pub field3: Vec<i32>,
}

#[test]
fn test_tmerge() {
    let foo = Foo {
        field1: "field1".to_string(),
        field2: 15,
    };
    let bar = Bar {
        field2: 16,
        field3: vec![1, 2, 3],
    };
    // unwrap is not recommended, handle Result in your app
    let result: FooBar = tmerge(foo, bar).unwrap();
    let target = FooBar {
        field1: "field1".to_string(),
        field2: 16,
        field3: vec![1, 2, 3],
    };
    assert_eq!(result, target);
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct HasOptional {
    field1: i32,
    field2: Option<bool>,
    field3: Option<String>,
}

#[test]
pub fn test_omerge() {
    let s1 = HasOptional {
        field1: 1,
        field2: None,
        field3: Some("3".to_string()),
    };
    let s2 = HasOptional {
        field1: 2,
        field2: Some(true),
        field3: None,
    };
    let result: HasOptional = omerge(s1, s2).unwrap();
    let target = HasOptional {
        field1: 2,
        field2: Some(true),
        field3: Some("3".to_string()),
    };
    assert_eq!(result, target);
}

use serde_merge;
use std::collections::HashMap;

fn main() {
    let mut map1: HashMap<&str, &str> = HashMap::new();
    map1.insert("key1", "value1");
    map1.insert("key2", "value2");
    map1.insert("key3", "value3");
    let mut map2: HashMap<&str, &str> = HashMap::new();
    map2.insert("key4", "value4");
    map2.insert("key5", "value5");
    map2.insert("key6", "value6");
    let result = serde_merge::mmerge(map1, map2).unwrap();
    println!("{:#?}", result);
}

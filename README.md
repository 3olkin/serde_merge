# SERDE MERGE

Simple utility to merge some serializable data types based on [serde](https://github.com/serde-rs/serde).

## Quick Start

### HashMap with HashMap

```rust
use serde_merge::mmerge;
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
    let result = mmerge(map1, map2).unwrap();
    println!("{:#?}", result);
    // --Output--
    // {
    //     "key1": String(
    //         "value1",
    //     ),
    //     "key2": String(
    //         "value2",
    //     ),
    //     "key3": String(
    //         "value3",
    //     ),
    //     "key4": String(
    //         "value4",
    //     ),
    //     "key5": String(
    //         "value5",
    //     ),
    //     "key6": String(
    //         "value6",
    //     ),
    // }
}
```

### Struct with struct

```rust
use serde::{Deserialize, Serialize};
use serde_merge::tmerge;

#[derive(Serialize, Deserialize)]
struct Foo {
    pub field1: String,
    pub field2: i32,
}

#[derive(Serialize, Deserialize)]
struct Bar {
    pub field3: Vec<String>,
    pub field4: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Result {
    pub field1: String,
    pub field2: i32,
    pub field3: Vec<String>,
    pub field4: u32,
}

fn main() {
    let foo = Foo {
        field1: "field1".to_string(),
        field2: 15,
    };
    let bar = Bar {
        field3: Vec::from([
            "elem1".to_string(),
            "elem2".to_string(),
            "elem3".to_string(),
        ]),
        field4: 5,
    };
    // unwrap is not recommended, handle Result in your app
    let result: Result = tmerge(foo, bar).unwrap();
    println!("{:#?}", result);
    // --Output--
    // Result {
    //     field1: "field1",
    //     field2: 15,
    //     field3: [
    //         "elem1",
    //         "elem2",
    //         "elem3",
    //     ],
    //     field4: 5,
    // }
}
```

#### Note

Merging structs require `derive` feature for `serde`.

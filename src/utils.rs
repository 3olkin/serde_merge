use crate::Map;
use serde::{de::Deserialize, Serialize};

pub(crate) fn struct_to_map<'a, T: Serialize + Deserialize<'a>>(strct: T) -> anyhow::Result<Map> {
    let string = serde_json::to_string(&strct)?;
    Ok(serde_json::from_str(&string)?)
}

pub(crate) fn map_to_struct<'a, T: Serialize + Deserialize<'a>>(map: Map) -> anyhow::Result<T> {
    let string = serde_json::to_string(&map)?;
    Ok(serde_json::from_str(string_to_static_str(string))?)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

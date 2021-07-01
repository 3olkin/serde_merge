use crate::Map;
use serde::{de::Deserialize, Serialize};

pub(crate) fn to_map<'a, T: Serialize + Deserialize<'a>>(value: &T) -> anyhow::Result<Map> {
    let string = serde_json::to_string(value)?;
    Ok(serde_json::from_str(&string)?)
}

pub(crate) fn from_map<'a, T: Serialize + Deserialize<'a>>(map: &Map) -> anyhow::Result<T> {
    let string = serde_json::to_string(map)?;
    Ok(serde_json::from_str(string_to_static_str(string))?)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

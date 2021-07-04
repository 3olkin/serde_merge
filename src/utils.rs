use crate::Map;
use serde::{
    de::{Deserialize, DeserializeOwned},
    Serialize,
};

pub(crate) fn to_map<'a, T: Serialize + Deserialize<'a>>(value: &T) -> anyhow::Result<Map> {
    let serde_value = serde_json::to_value(value)?;
    Ok(serde_json::from_value(serde_value)?)
}

pub(crate) fn from_map<T: Serialize + DeserializeOwned>(map: &Map) -> anyhow::Result<T> {
    let serde_value = serde_json::to_value(map)?;
    Ok(serde_json::from_value(serde_value)?)
}

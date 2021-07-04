use crate::{
    error::{Error, Result},
    Map,
};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) fn to_map<T: Serialize>(value: &T) -> Result<Map> {
    match serde_json::to_value(value)?.as_object() {
        Some(map) => Ok(map.to_owned()),
        None => Err(Error::NotObject),
    }
}

pub(crate) fn from_map<T: Serialize + DeserializeOwned>(map: &Map) -> Result<T> {
    let value = serde_json::to_value(map)?;
    Ok(serde_json::from_value(value)?)
}

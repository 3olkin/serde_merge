use serde::{de::Deserialize, Serialize};

extern crate serde;
extern crate serde_json;

pub type Map = serde_json::Map<String, serde_json::Value>;

// TODO: Support HashMap args
pub fn merge_to_map<'a, L, R>(left: L, right: R) -> anyhow::Result<Map>
where
    L: Serialize + Deserialize<'a>,
    R: Serialize + Deserialize<'a>,
{
    let left_map = utils::to_map(&left)?;
    let mut right_map = utils::to_map(&right)?;

    for key in left_map.keys() {
        if !right_map.contains_key(key) {
            right_map.insert(
                key.to_string(),
                // unwrap is safe here because key exists
                left_map.get(key).unwrap().to_owned().take(),
            );
        };
    }

    Ok(right_map)
}

pub fn merge_to_struct<'a, L, R, T>(left: L, right: R) -> anyhow::Result<T>
where
    L: Serialize + Deserialize<'a>,
    R: Serialize + Deserialize<'a>,
    T: Serialize + Deserialize<'a>,
{
    let merged_map = merge_to_map(left, right)?;
    Ok(utils::from_map(&merged_map)?)
}

#[cfg(test)]
mod tests;
mod utils;

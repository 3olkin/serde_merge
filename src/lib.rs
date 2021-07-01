use serde::{de::Deserialize, Serialize};

extern crate serde;
extern crate serde_json;

/// Alias for a `serde_json::Map<String, serde_json::Value>`.
///
/// Represents default type used in this library.
pub type Map = serde_json::Map<String, serde_json::Value>;

/// Merge two types into `serde_merge::Map`, returns `anyhow::Result<serde_merge::Map>`.
///
/// Merge `R` into `L`. `R` override matching keys of `L`.
/// Both `L` and `R` have to implement `serde::Serialize` and `serde::de::Deserialize`.
pub fn mmerge<'a, L, R>(left: L, right: R) -> anyhow::Result<Map>
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
                // unwrap is safe here because this key exists
                left_map.get(key).unwrap().to_owned().take(),
            );
        };
    }

    Ok(right_map)
}

/// Merge two types into given type `T`, returns `anyhow::Result<T>`. ( *Recommended* )
///
/// Works the same as `serde_merge::mmerge` but convert result to given type `T`.
/// `T` has to implement `serde::Serialize` and `serde::de::Deserialize`.
pub fn tmerge<'a, L, R, T>(left: L, right: R) -> anyhow::Result<T>
where
    L: Serialize + Deserialize<'a>,
    R: Serialize + Deserialize<'a>,
    T: Serialize + Deserialize<'a>,
{
    let merged_map = mmerge(left, right)?;
    Ok(utils::from_map(&merged_map)?)
}

mod utils;

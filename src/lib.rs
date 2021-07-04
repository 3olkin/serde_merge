//! # Serde Merge
//!
//! `serde_merge` is a set of methods on top of [serde](https://github.com/serde-rs/serde)
//! for merging some serializable types.
use crate::error::Result;
use serde::{de::DeserializeOwned, Serialize};

pub mod error;
mod utils;

/// Alias for a `serde_json::Map<String, serde_json::Value>`.
///
/// Represents default type used in this library.
pub type Map = serde_json::Map<String, serde_json::Value>;

/// Merge two types into `serde_merge::Map`, returns `serde_merge::Result<serde_merge::Map>`.
///
/// Merge `R` into `L`. `R` override matching keys of `L`.
/// Both `L` and `R` have to implement `serde::Serialize`.
pub fn mmerge<L, R>(left: L, right: R) -> Result<Map>
where
    L: Serialize,
    R: Serialize,
{
    let mut left_map = utils::to_map(&left)?;
    let right_map = utils::to_map(&right)?;
    left_map.extend(right_map);

    Ok(left_map)
}

/// Merge two types into given type `T`, returns `serde_merge::Result<T>`. ( *Recommended* )
///
/// Works the same as `serde_merge::mmerge` but convert result to given type `T`.
/// `T` has to implement `serde::Serialize` and `serde::de::DeserializeOwned`.
pub fn tmerge<L, R, T>(left: L, right: R) -> Result<T>
where
    L: Serialize,
    R: Serialize,
    T: Serialize + DeserializeOwned,
{
    let merged_map = mmerge(left, right)?;
    Ok(utils::from_map(&merged_map)?)
}

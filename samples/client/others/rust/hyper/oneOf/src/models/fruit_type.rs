/*
 * ByRefOrValue
 *
 * This tests for a oneOf interface representation 
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FruitType {
    #[serde(rename = "APPLE")]
    Apple,
    #[serde(rename = "BANANA")]
    Banana,

}

impl std::fmt::Display for FruitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Apple => write!(f, "APPLE"),
            Self::Banana => write!(f, "BANANA"),
        }
    }
}

impl Default for FruitType {
    fn default() -> FruitType {
        Self::Apple
    }
}


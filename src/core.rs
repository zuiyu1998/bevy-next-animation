use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct AnimationName(String);

impl AnimationName {
    pub fn new(name: &str) -> Self {
        AnimationName(name.to_string())
    }
}

#[derive(
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    Clone,
    Deref,
    DerefMut,
    Deserialize,
    Serialize,
    Reflect,
    PartialOrd,
)]
pub struct ShortTypePath(String);

impl ShortTypePath {
    pub fn from_type_path<T: TypePath>() -> Self {
        Self(T::short_type_path().to_string())
    }
}

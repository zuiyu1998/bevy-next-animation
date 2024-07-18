use crate::prelude::ShortTypePath;

use super::{ReflectError, TrackValue};
use bevy::{
    asset::{Asset, AssetServer, Handle},
    prelude::Reflect,
    reflect::{FromType, TypePath},
};
use serde::{Deserialize, Serialize};

impl<A: AnimateValue> FromType<A> for AnimateValueFns {
    fn from_type() -> Self {
        AnimateValueFns {
            reflect: A::get_reflect_value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Reflect, Deserialize, Serialize, PartialOrd, Hash, Eq)]
pub struct AnimationValueAssetPath {
    pub path: String,
    pub type_path: ShortTypePath,
}

#[derive(Clone)]
pub struct AnimateValueFns {
    pub reflect:
        fn(&TrackValue, asset_server: &AssetServer) -> Result<Box<dyn Reflect>, ReflectError>,
}

impl AnimateValueFns {
    pub fn new<A: AnimateValue>() -> Self {
        AnimateValueFns {
            reflect: A::get_reflect_value,
        }
    }
}

pub trait AnimateValue: Reflect + TypePath {
    fn get_reflect_value(
        value: &TrackValue,
        asset_server: &AssetServer,
    ) -> Result<Box<dyn Reflect>, ReflectError>;
}

impl AnimateValue for bool {
    fn get_reflect_value(
        value: &TrackValue,
        _asset_server: &AssetServer,
    ) -> Result<Box<dyn Reflect>, ReflectError> {
        match value {
            TrackValue::Number(number) => return Ok(Box::new(number.ne(&0.0))),
            _ => {
                return Err(ReflectError::Kind(format!("TrackValue is not valid.")));
            }
        }
    }
}

impl AnimateValue for usize {
    fn get_reflect_value(
        value: &TrackValue,
        _asset_server: &AssetServer,
    ) -> Result<Box<dyn Reflect>, ReflectError> {
        match value {
            TrackValue::Number(number) => return Ok(Box::new(*number as usize)),
            _ => {
                return Err(ReflectError::Kind(format!("TrackValue is not valid.")));
            }
        }
    }
}

impl<A: Asset> AnimateValue for Handle<A> {
    fn get_reflect_value(
        value: &TrackValue,
        asset_server: &AssetServer,
    ) -> Result<Box<dyn Reflect>, ReflectError> {
        match value {
            TrackValue::Asset(asset) => {
                if asset.type_path != ShortTypePath::from_type_path::<Self>() {
                    return Err(ReflectError::Kind(format!("asset type mismatch.")));
                } else {
                    let handle: Self = asset_server.load(asset.path.clone());

                    return Ok(Box::new(handle));
                }
            }
            _ => {
                return Err(ReflectError::Kind(format!("TrackValue is not valid.")));
            }
        }
    }
}

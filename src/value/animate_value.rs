use super::TrackValue;
use bevy::{
    asset::AssetServer,
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
pub struct AnimationValueAssetPath(pub String);

#[derive(Clone)]
pub struct AnimateValueFns {
    pub reflect: fn(&TrackValue, asset_server: &AssetServer) -> Option<Box<dyn Reflect>>,
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
    ) -> Option<Box<dyn Reflect>>;
}

impl AnimateValue for bool {
    fn get_reflect_value(
        value: &TrackValue,
        _asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>> {
        match value {
            TrackValue::Number(number) => return Some(Box::new(number.ne(&0.0))),
            _ => {
                return None;
            }
        }
    }
}

impl AnimateValue for usize {
    fn get_reflect_value(
        value: &TrackValue,
        _asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>> {
        match value {
            TrackValue::Number(number) => return Some(Box::new(*number as usize)),
            _ => {
                return None;
            }
        }
    }
}

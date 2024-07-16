use super::TrackValue;
use bevy::{
    app::App,
    asset::AssetServer,
    prelude::Reflect,
    reflect::{FromType, TypePath},
};
use serde::{Deserialize, Serialize};

pub trait AnimationExt {
    fn register_animation<T: AnimatinValue>(&mut self);
}

impl AnimationExt for App {
    fn register_animation<T: AnimatinValue>(&mut self) {
        self.register_type_data::<T, AnimationComponentFns>();
    }
}

impl<A: AnimatinValue> FromType<A> for AnimationComponentFns {
    fn from_type() -> Self {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Reflect, Deserialize, Serialize, PartialOrd, Hash, Eq)]
pub struct AnimationValueAssetPath(pub String);

///可支持的关键帧数据类型
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ValueType {
    Bool,
    Usize,
    Asset,
}

#[derive(Clone)]
pub struct AnimationComponentFns {
    pub reflect: fn(&TrackValue, asset_server: &AssetServer) -> Option<Box<dyn Reflect>>,
}

impl AnimationComponentFns {
    pub fn new<A: AnimatinValue>() -> Self {
        AnimationComponentFns {
            reflect: A::get_reflect_value,
        }
    }
}

pub trait AnimatinValue: Reflect + TypePath {
    fn get_reflect_value(
        value: &TrackValue,
        asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>>;
}

impl AnimatinValue for bool {
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

impl AnimatinValue for usize {
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

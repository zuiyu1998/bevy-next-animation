use super::TrackValue;
use bevy::{asset::AssetServer, prelude::Reflect, render::texture::Image};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Reflect, Deserialize, Serialize, PartialOrd, Hash, Eq)]
pub struct AnimationValueAssetPath(pub String);

///可支持的关键帧数据类型
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ValueType {
    Bool,
    Usize,
    Asset,
}

pub trait AnimatinValue {
    fn get_reflect_value(
        value: &TrackValue,
        _asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>>;
}

impl AnimatinValue for AnimationValueAssetPath {
    fn get_reflect_value(
        value: &TrackValue,
        asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>> {
        match value {
            TrackValue::Asset(path) => {
                if let Some(handle) = asset_server.get_handle::<Image>(&path.0) {
                    let reflect: Box<dyn Reflect> = Box::new(handle);

                    return Some(reflect);
                } else {
                    None
                }
            }
            _ => {
                return None;
            }
        }
    }
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

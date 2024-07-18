mod animate_components;
mod animate_value;

pub use animate_components::*;
pub use animate_value::*;

use bevy::{
    asset::AssetServer,
    log::warn,
    reflect::{Reflect, TypeRegistry},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::ShortTypePath;

#[derive(Debug, Error)]
pub enum ReflectError {
    #[error("reflect error: {0}")]
    Kind(String),
}

#[derive(Clone)]
pub struct BoundComponentValue(pub Vec<BoundValue>);

impl BoundComponentValue {
    pub fn get_component_pose(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Option<ComponentPose> {
        let mut values = vec![];

        for bound_value in self.0.iter() {
            match bound_value.get_relect_value(registry, asset_server) {
                Ok(reflect) => {
                    values.push(ReflectBoundValue {
                        value: reflect,
                        path: bound_value.binding.path.clone(),
                    });
                }

                Err(e) => {
                    warn!("get_relect_value error: {}", e);
                }
            }
        }

        if values.is_empty() {
            return None;
        } else {
            Some(ComponentPose { values })
        }
    }
}

#[derive(Clone)]
pub struct ComponentPose {
    pub values: Vec<ReflectBoundValue>,
}

pub struct ReflectBoundValue {
    pub path: String,
    pub value: Box<dyn Reflect>,
}

impl Clone for ReflectBoundValue {
    fn clone(&self) -> Self {
        ReflectBoundValue {
            path: self.path.clone(),
            value: self.value.clone_value(),
        }
    }
}

///组件修改的字段路径和关键帧的数据类型
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueBinding {
    pub path: String,
    pub value_type: ShortTypePath,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, PartialOrd)]
pub struct AssetPath {
    pub path: String,
    pub type_path: ShortTypePath,
}

///原始的关键帧数据
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, PartialOrd)]
pub enum TrackValue {
    Number(f32),
    Asset(AssetPath),
}

impl TrackValue {
    pub fn blend_with(&mut self, _other: &Self, _weight: f32) {
        todo!()
    }
}

///用来修改组件的关键帧数据抽象
#[derive(Clone)]
pub struct BoundValue {
    pub binding: ValueBinding,
    pub value: TrackValue,
}

impl BoundValue {
    ///根据weight 混合
    pub fn blend_with(&mut self, other: &Self, weight: f32) {
        assert_eq!(self.binding.path, other.binding.path);
        self.value.blend_with(&other.value, weight);
    }

    pub fn get_relect_value(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Result<Box<dyn Reflect>, ReflectError> {
        let registraion = registry
            .get_with_short_type_path(&self.binding.value_type)
            .ok_or(ReflectError::Kind(format!(
                "{:?} not register type",
                self.binding.value_type
            )))?;
        if let Some(fns) = registraion.data::<AnimateValueFns>() {
            (fns.reflect)(&self.value, asset_server)
        } else {
            Err(ReflectError::Kind(format!(
                "{:?} not register animate value",
                self.binding.value_type
            )))
        }
    }
}

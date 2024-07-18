mod animate_components;
mod animate_value;

pub use animate_components::*;
pub use animate_value::*;

use bevy::{
    app::App,
    asset::AssetServer,
    log::warn,
    reflect::{Reflect, TypeRegistry},
};
use serde::{Deserialize, Serialize};

use crate::prelude::ShortTypePath;

pub trait AnimationExt {
    fn register_animate_value<T: AnimateValue>(&mut self) -> &mut Self;
    fn register_animate_component<T: AnimateComponent>(&mut self) -> &mut Self;
}

impl AnimationExt for App {
    fn register_animate_value<T: AnimateValue>(&mut self) -> &mut Self {
        self.register_type_data::<T, AnimateValueFns>();
        self
    }

    fn register_animate_component<T: AnimateComponent>(&mut self) -> &mut Self {
        self.register_type_data::<T, AnimateComponentFns>();
        self
    }
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
            if let Some(reflect) = bound_value.get_relect_value(registry, asset_server) {
                values.push(ReflectBoundValue {
                    value: reflect,
                    path: bound_value.binding.path.clone(),
                });
            }
        }

        Some(ComponentPose { values })
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
    pub component_type: ShortTypePath,
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
    ) -> Option<Box<dyn Reflect>> {
        if let Some(registraion) = registry.get_with_short_type_path(&self.binding.component_type) {
            if let Some(fns) = registraion.data::<AnimateValueFns>() {
                if let Some(field) = (fns.reflect)(&self.value, asset_server) {
                    return Some(field);
                } else {
                    warn!(
                        "{:?} not impl AnimatinValue trait.",
                        registraion.type_info().type_path_table().ident()
                    );

                    return None;
                }
            } else {
                warn!(
                    "{:?} not found AnimationValueFns.",
                    registraion.type_info().type_path_table().ident()
                );
                return None;
            }
        }

        return None;
    }
}

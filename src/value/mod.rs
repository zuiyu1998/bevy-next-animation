mod animate_components;
mod animate_value;

pub use animate_components::*;
pub use animate_value::*;

use bevy::{
    app::App,
    asset::AssetServer,
    log::warn,
    reflect::{DynamicStruct, Reflect, ReflectKind, TypeRegistry},
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
pub struct BoundValueCollection {
    pub values: Vec<BoundValue>,
    pub relect_kind: ReflectKind,
}

impl BoundValueCollection {
    pub fn get_dynamic(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Box<dyn Reflect> {
        match self.relect_kind {
            ReflectKind::Struct => self.get_dynamic_struct(registry, asset_server),

            _ => {
                todo!()
            }
        }
    }

    pub fn get_dynamic_struct(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Box<dyn Reflect> {
        let mut dynamic = DynamicStruct::default();

        for v in self.values.iter() {
            if let Some(registraion) = registry.get_with_short_type_path(&v.binding.value_type) {
                if let Some(fns) = registraion.data::<AnimateValueFns>() {
                    if let Some(field) = (fns.reflect)(&v.value, asset_server) {
                        dynamic.insert_boxed(v.binding.path.clone(), field);
                    } else {
                        warn!(
                            "{:?} not impl AnimatinValue trait.",
                            registraion.type_info().type_path_table().ident()
                        );
                    }
                } else {
                    warn!(
                        "{:?} not found AnimationValueFns.",
                        registraion.type_info().type_path_table().ident()
                    );
                }
            }
        }

        Box::new(dynamic)
    }
}

impl Default for BoundValueCollection {
    fn default() -> Self {
        Self {
            values: vec![],
            relect_kind: ReflectKind::Struct,
        }
    }
}

///组件修改的字段路径和关键帧的数据类型
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueBinding {
    pub path: String,
    pub value_type: ShortTypePath,
}

///原始的关键帧数据
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, PartialOrd)]
pub enum TrackValue {
    Number(f32),
    Asset(AnimationValueAssetPath),
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
}

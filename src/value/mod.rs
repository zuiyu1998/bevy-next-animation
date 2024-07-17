mod animate_components;
mod animate_value;

pub use animate_components::*;
pub use animate_value::*;

use bevy::{
    app::App,
    asset::AssetServer,
    log::warn,
    reflect::{DynamicStruct, Reflect, TypeRegistry},
};
use serde::{Deserialize, Serialize};

use crate::core::ComponentReflectKind;
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
pub struct BoundComponentValue {
    pub data: BoundValueData,
    pub relect_kind: ComponentReflectKind,
}

#[derive(Clone)]
pub enum BoundValueData {
    Single(BoundValue),
    Multiple(Vec<BoundValue>),
}

impl BoundComponentValue {
    pub fn get_dynamic(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>> {
        match self.relect_kind {
            ComponentReflectKind::Struct => self.get_dynamic_struct(registry, asset_server),
            ComponentReflectKind::Enum => self.get_dynamic_enum(registry, asset_server),

            _ => {
                todo!()
            }
        }
    }

    pub fn get_dynamic_enum(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>> {
        match &self.data {
            BoundValueData::Single(v) => v.get_relect_value(registry, asset_server),
            BoundValueData::Multiple(_) => {
                return None;
            }
        }
    }

    pub fn get_dynamic_struct(
        &self,
        registry: &TypeRegistry,
        asset_server: &AssetServer,
    ) -> Option<Box<dyn Reflect>> {
        match &self.data {
            BoundValueData::Single(v) => v.get_relect_value(registry, asset_server),
            BoundValueData::Multiple(values) => {
                let mut dynamic = DynamicStruct::default();
                for v in values.iter() {
                    if let Some(registraion) =
                        registry.get_with_short_type_path(&v.binding.component_type)
                    {
                        if let Some(fns) = registraion.data::<AnimateValueFns>() {
                            if let Some(field) = (fns.reflect)(&v.value, asset_server) {
                                dynamic.insert_boxed(v.binding.path.clone().unwrap(), field);
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

                Some(Box::new(dynamic))
            }
        }
    }
}

///组件修改的字段路径和关键帧的数据类型
#[derive(Clone, Serialize, Deserialize)]
pub struct ValueBinding {
    pub path: Option<String>,
    pub component_type: ShortTypePath,
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

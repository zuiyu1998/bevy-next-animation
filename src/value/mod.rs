mod value_type;

pub use value_type::*;

use bevy::{
    asset::AssetServer,
    reflect::{DynamicStruct, Reflect, ReflectKind},
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BoundValueCollection {
    pub values: Vec<BoundValue>,
    pub relect_kind: ReflectKind,
}

impl BoundValueCollection {
    pub fn get_dynamic(&self, asset_server: &AssetServer) -> Box<dyn Reflect> {
        match self.relect_kind {
            ReflectKind::Struct => self.get_dynamic_struct(asset_server),

            _ => {
                todo!()
            }
        }
    }

    pub fn get_dynamic_struct(&self, asset_server: &AssetServer) -> Box<dyn Reflect> {
        let mut dynamic = DynamicStruct::default();

        for value in self.values.iter() {
            if let Some(reflect) = value.get_reflect_value(asset_server) {
                dynamic.insert_boxed(value.binding.path.clone(), reflect);
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
    pub value_type: ValueType,
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

    pub fn get_reflect_value(&self, asset_server: &AssetServer) -> Option<Box<dyn Reflect>> {
        // match self.binding.value_type {
        //     ValueType::Bool => bool::get_reflect_value(&self.value, asset_server),
        //     ValueType::Usize => usize::get_reflect_value(&self.value, asset_server),
        //     ValueType::Asset => {
        //         AnimationValueAssetPath::get_reflect_value(&self.value, asset_server)
        //     }
        // }

        todo!()
    }
}

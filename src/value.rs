use bevy::{
    math::FloatExt,
    reflect::{DynamicStruct, Reflect, ReflectKind},
};

pub struct BoundValueCollection {
    pub values: Vec<BoundValue>,
    pub relect_kind: ReflectKind,
}

impl BoundValueCollection {
    pub fn get_dynamic(&self) -> Box<dyn Reflect> {
        match self.relect_kind {
            ReflectKind::Struct => self.get_dynamic_struct(),

            _ => {
                todo!()
            }
        }
    }

    pub fn get_dynamic_struct(&self) -> Box<dyn Reflect> {
        let mut dynamic = DynamicStruct::default();

        for value in self.values.iter() {
            if let Some(reflect) = value.get_reflect_value() {
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

///可支持的关键帧数据类型
#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Bool,
    Usize,
}

///组件修改的字段路径和关键帧的数据类型
#[derive(Clone)]
pub struct ValueBinding {
    pub path: String,
    pub value_type: ValueType,
}

///原始的关键帧数据
#[derive(Clone, Debug, PartialEq)]
pub struct TrackValue(pub f32);

impl TrackValue {
    pub fn blend_with(&mut self, other: &Self, weight: f32) {
        self.0 = self.0.lerp(other.0, weight);
    }

    pub fn get_number_type(&self, value: ValueType) -> Option<Box<dyn Reflect>> {
        match value {
            ValueType::Bool => Some(Box::new(self.0.ne(&0.0))),
            ValueType::Usize => Some(Box::new(self.0 as usize)),
        }
    }
}

///用来修改组件的关键帧数据抽象
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

    pub fn get_reflect_value(&self) -> Option<Box<dyn Reflect>> {
        self.value.get_number_type(self.binding.value_type)
    }
}

mod test {

    #[test]
    fn test_bound_value() {
        use bevy::prelude::*;

        use super::{BoundValue, TrackValue, ValueBinding, ValueType};

        #[derive(Component, Reflect, Default)]
        pub struct TestA {
            pub a: bool,
        }

        let mut test_a = TestA { a: false };

        let bound_value = BoundValue {
            binding: ValueBinding {
                path: "a".to_string(),
                value_type: ValueType::Bool,
            },
            value: TrackValue(1.0),
        };

        let reflect: &mut dyn Struct = &mut test_a;

        let mut value = bound_value.get_reflect_value().unwrap();

        if let Some(field) = reflect.field_mut(&bound_value.binding.path) {
            field.apply(&mut *value);
        }

        assert_eq!(test_a.a, true);
    }
}

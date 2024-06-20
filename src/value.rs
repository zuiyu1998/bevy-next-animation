use bevy::{
    math::FloatExt,
    reflect::{Reflect, ReflectMut},
};

#[derive(Default)]
pub struct BoundValueCollection {
    pub values: Vec<BoundValue>,
}

///可支持的关键帧数据类型
#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    //
    Bool,
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

    ///设置组件的数据
    pub fn apply_to_object(&self, object: &mut dyn Reflect) {
        if let Some(caset) = self.value.get_number_type(self.binding.value_type) {
            match object.reflect_mut() {
                ReflectMut::Struct(object) => {
                    if let Some(field) = object.field_mut(&self.binding.path) {
                        field.apply(&(*caset));
                    };
                }
                _ => {}
            }
        }
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

        let reflect: &mut dyn Reflect = &mut test_a;

        bound_value.apply_to_object(reflect);

        assert_eq!(test_a.a, true);
    }
}

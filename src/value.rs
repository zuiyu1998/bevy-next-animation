use bevy::{
    math::FloatExt,
    reflect::{DynamicStruct, Reflect, ReflectKind, Struct},
};

///可支持的关键帧数据类型
#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    //
    Bool,
}

///组件修改的字段路径和关键帧的数据类型
pub struct ValueBinding {
    path: String,
    value_type: ValueType,
}

///原始的关键帧数据
#[derive(Clone, Debug, PartialEq)]
pub struct TrackValue(f32);

impl TrackValue {
    pub fn blend_with(&mut self, other: &Self, weight: f32) {
        self.0 = self.0.lerp(other.0, weight);
    }

    pub fn get_number_type(&self, value: ValueType) -> Option<Box<dyn Reflect>> {
        match value {
            ValueType::Bool => Some(Box::new(self.0.eq(&0.0))),
        }
    }
}

///用来修改组件的关键帧数据抽象
pub struct BindValue {
    binding: ValueBinding,
    value: TrackValue,
}

impl BindValue {
    ///根据weight 混合
    pub fn blend_with(&mut self, other: &Self, weight: f32) {
        assert_eq!(self.binding.path, other.binding.path);
        self.value.blend_with(&other.value, weight);
    }

    ///设置组件的数据
    pub fn apply_to_object(&self, object: &mut dyn Reflect) {
        if let Some(caset) = self.value.get_number_type(self.binding.value_type) {
            match object.reflect_kind() {
                ReflectKind::Struct => {
                    let object = object.downcast_mut::<DynamicStruct>().unwrap();

                    if let Some(field) = object.field_mut(&self.binding.path) {
                        field.apply(&(*caset));
                    }
                }
                _ => {}
            }
        }
    }
}

use std::any;

use bevy::{
    prelude::Component,
    reflect::{Reflect, TypeInfo, TypePath, TypeRegistry},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    core::ComponentReflectKind,
    prelude::{AnimateValueFns, ShortTypePath},
    value::{BoundComponentValue, BoundValue, BoundValueData, TrackValue, ValueBinding},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct ComponentTrack {
    pub component_type: ShortTypePath,
    pub data: TrackData,
    pub relect_kind: ComponentReflectKind,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum TrackData {
    Single(Track),
    Multiple(HashMap<String, Track>),
}

fn get_relect_kind(tupe_info: &TypeInfo) -> ComponentReflectKind {
    match tupe_info {
        TypeInfo::Struct(_) => ComponentReflectKind::Struct,
        TypeInfo::Enum(_) => ComponentReflectKind::Enum,
        _ => ComponentReflectKind::None,
    }
}

impl ComponentTrack {
    pub fn add_track(&mut self, track: Track) {
        match &mut self.data {
            TrackData::Multiple(tracks) => {
                tracks.insert(track.binding.path.clone().unwrap(), track);
            }

            TrackData::Single(inner_track) => {
                let mut track = track;
                track.binding.path = None;

                *inner_track = track;
            }
        }
    }

    pub fn new<T: Reflect + TypePath + Component>(registry: &TypeRegistry) -> Option<Self> {
        let type_id = any::TypeId::of::<T>();

        match registry.get(type_id) {
            None => None,
            Some(registraion) => match get_relect_kind(registraion.type_info()) {
                ComponentReflectKind::Struct => {
                    if let Some(_) = registraion.data::<AnimateValueFns>() {
                        Some(ComponentTrack {
                            component_type: ShortTypePath::from_type_path::<T>(),
                            relect_kind: ComponentReflectKind::Struct,
                            data: TrackData::Single(Track::new(
                                ValueBinding {
                                    path: None,
                                    component_type: ShortTypePath::from_type_path::<T>(),
                                },
                                0.1,
                                10,
                            )),
                        })
                    } else {
                        Some(ComponentTrack {
                            component_type: ShortTypePath::from_type_path::<T>(),
                            relect_kind: ComponentReflectKind::Struct,
                            data: TrackData::Multiple(Default::default()),
                        })
                    }
                }
                ComponentReflectKind::Enum => {
                    if let Some(_) = registraion.data::<AnimateValueFns>() {
                        Some(ComponentTrack {
                            component_type: ShortTypePath::from_type_path::<T>(),
                            relect_kind: ComponentReflectKind::Enum,
                            data: TrackData::Single(Track::new(
                                ValueBinding {
                                    path: None,
                                    component_type: ShortTypePath::from_type_path::<T>(),
                                },
                                0.1,
                                10,
                            )),
                        })
                    } else {
                        Some(ComponentTrack {
                            component_type: ShortTypePath::from_type_path::<T>(),
                            relect_kind: ComponentReflectKind::Enum,
                            data: TrackData::Multiple(Default::default()),
                        })
                    }
                }
                _ => None,
            },
        }
    }

    pub(crate) fn fetch(&self, time: f32) -> Option<BoundComponentValue> {
        match &self.data {
            TrackData::Multiple(values) => {
                let mut bound_values = vec![];

                for track in values.values() {
                    if let Some(bound_value) = track.fetch(time) {
                        bound_values.push(bound_value);
                    }
                }

                Some(BoundComponentValue {
                    relect_kind: self.relect_kind,
                    data: BoundValueData::Multiple(bound_values),
                })
            }
            TrackData::Single(track) => {
                if let Some(bound_value) = track.fetch(time) {
                    Some(BoundComponentValue {
                        relect_kind: self.relect_kind,
                        data: BoundValueData::Single(bound_value),
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    enabled: bool,
    pub frames: TrackDataContainer,
    binding: ValueBinding,
}

impl Track {
    pub fn new(binding: ValueBinding, frame_duration: f32, frame_count: usize) -> Self {
        Self {
            enabled: true,
            frames: TrackDataContainer::new(frame_duration, frame_count),
            binding,
        }
    }
    pub fn add_keyframe(&mut self, key_frame: Keyframe) {
        self.frames.add_keyframe(key_frame);
    }

    pub fn path(&self) -> Option<&str> {
        self.binding.path.as_deref()
    }

    pub fn fetch(&self, time: f32) -> Option<BoundValue> {
        if !self.enabled {
            return None;
        }

        self.frames.fetch(time).map(|value| BoundValue {
            binding: self.binding.clone(),
            value,
        })
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TrackDataContainer {
    //关键帧数据
    pub keyframes: HashMap<Uuid, Keyframe>,

    //差值模式
    mode: InterpolationMode,

    //动画时间
    frame_duration: f32,
    //关键帧索引
    frame_indexs: Vec<Option<Uuid>>,
}

impl TrackDataContainer {
    fn add_keyframe(&mut self, key_frame: Keyframe) {
        if let Some(value) = self.frame_indexs.get_mut(key_frame.location) {
            *value = Some(key_frame.id.to_owned());

            self.keyframes.insert(key_frame.id.to_owned(), key_frame);
        }
    }

    fn new(frame_duration: f32, frame_count: usize) -> Self {
        TrackDataContainer {
            keyframes: Default::default(),
            mode: InterpolationMode::Constant,
            frame_duration,
            frame_indexs: vec![None; frame_count],
        }
    }

    fn fetch(&self, time: f32) -> Option<TrackValue> {
        let real_time = time % (self.frame_duration * self.frame_indexs.len() as f32);

        let index = real_time / self.frame_duration;

        let index_min = index.floor() as usize;
        // let index_max = index.ceil() as usize;

        match self.mode {
            InterpolationMode::Constant => {
                if let Some(uuid) = self.frame_indexs[index_min] {
                    Some(self.keyframes.get(&uuid).unwrap().value.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum InterpolationMode {
    Constant,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
pub struct Keyframe {
    pub id: Uuid,
    pub location: usize,
    pub value: TrackValue,
}

impl Keyframe {
    pub fn new(location: usize, value: TrackValue) -> Self {
        Self {
            location,
            value,
            id: Uuid::new_v4(),
        }
    }
}

mod test {

    #[test]
    fn test_track() {
        use super::{Keyframe, Track, ValueBinding};
        use crate::prelude::ShortTypePath;
        use crate::prelude::TrackValue;

        let mut track = Track::new(
            ValueBinding {
                path: Some("a".to_owned()),
                component_type: ShortTypePath::from_type_path::<bool>(),
            },
            0.5,
            2,
        );

        track.add_keyframe(Keyframe::new(1, TrackValue::Number(0.0)));
        track.add_keyframe(Keyframe::new(0, TrackValue::Number(1.0)));

        let bound_value = track.fetch(0.0);

        assert_eq!(true, bound_value.is_some());

        let bound_value = bound_value.unwrap();

        assert_eq!(bound_value.value, TrackValue::Number(1.0));

        let bound_value = track.fetch(0.5);

        let bound_value = bound_value.unwrap();

        assert_eq!(bound_value.value, TrackValue::Number(0.0));

        let bound_value = track.fetch(1.0);

        let bound_value = bound_value.unwrap();

        assert_eq!(bound_value.value, TrackValue::Number(1.0));
    }
}

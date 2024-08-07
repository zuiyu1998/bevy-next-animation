use bevy::{asset::AssetServer, log::warn, reflect::TypeRegistry, utils::HashMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value::{BoundValue, ReflectBoundValue, TrackValue, ValueBinding};

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

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct ComponentTrack {
    pub values: HashMap<String, Track>,
}

impl ComponentTrack {
    pub fn add_track(&mut self, track: Track) {
        self.values.insert(track.binding.path.clone(), track);
    }

    pub(crate) fn fetch(&self, time: f32) -> BoundComponentValue {
        let mut bound_values = vec![];

        for track in self.values.values() {
            if let Some(bound_value) = track.fetch(time) {
                bound_values.push(bound_value);
            }
        }

        BoundComponentValue(bound_values)
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
                path: ".a".to_owned(),
                value_type: ShortTypePath::from_type_path::<bool>(),
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

use bevy::utils::{HashMap, Uuid};

use crate::value::{BoundValue, BoundValueCollection, TrackValue};

use super::value::ValueBinding;

#[derive(Default)]
pub struct EntityTrack {
    pub values: HashMap<String, Track>,
}

impl EntityTrack {
    pub fn add_track(&mut self, track: Track) {
        self.values.insert(track.path(), track);
    }

    pub(crate) fn fetch(&self, time: f32) -> BoundValueCollection {
        let mut collection = BoundValueCollection::default();

        for track in self.values.values() {
            if let Some(bound_value) = track.fetch(time) {
                collection.values.push(bound_value);
            }
        }

        collection
    }
}

pub struct Track {
    enabled: bool,
    frames: TrackDataContainer,
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

    pub fn path(&self) -> String {
        self.binding.path.to_string()
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

pub struct TrackDataContainer {
    //关键帧数据
    keyframes: HashMap<Uuid, Keyframe>,

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
                    Some(TrackValue(self.keyframes.get(&uuid).unwrap().value))
                } else {
                    None
                }
            }
        }
    }
}

pub enum InterpolationMode {
    Constant,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Keyframe {
    pub id: Uuid,
    pub location: usize,
    pub value: f32,
}

impl Keyframe {
    pub fn new(location: usize, value: f32) -> Self {
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
        use crate::value::ValueType;

        let mut track = Track::new(
            ValueBinding {
                path: "a".to_owned(),
                value_type: ValueType::Bool,
            },
            0.5,
            2,
        );

        track.add_keyframe(Keyframe::new(1, 0.0));
        track.add_keyframe(Keyframe::new(0, 1.0));

        let bound_value = track.fetch(0.0);

        assert_eq!(true, bound_value.is_some());

        let bound_value = bound_value.unwrap();

        assert_eq!(bound_value.value.0, 1.0);

        let bound_value = track.fetch(0.5);

        let bound_value = bound_value.unwrap();

        assert_eq!(bound_value.value.0, 0.0);

        let bound_value = track.fetch(1.0);

        let bound_value = bound_value.unwrap();

        assert_eq!(bound_value.value.0, 1.0);
    }
}

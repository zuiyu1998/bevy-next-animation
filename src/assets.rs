use crate::entity::{AnimationName, EntityAnimation};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::HashMap,
};
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Default, Asset, TypePath, Clone, Deref, DerefMut, Deserialize, Serialize)]
pub struct EntityAnimations(HashMap<AnimationName, EntityAnimation>);

#[derive(Default)]
pub struct EntityAnimationsLoader;

// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum EntityAnimationsLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load asset: {0}")]
    Serde(#[from] serde_json::Error),
}

impl AssetLoader for EntityAnimationsLoader {
    type Asset = EntityAnimations;
    type Settings = ();
    type Error = EntityAnimationsLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let custom_asset = serde_json::from_slice::<EntityAnimations>(&bytes)?;

        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["entity_animations.json"]
    }
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "models/wizard.gltf#Scene0")]
    pub wizard: Handle<Scene>,
    #[asset(path = "models/old.gltf#Scene0")]
    pub old: Handle<Scene>,
    #[asset(path = "models/hat.gltf#Scene0")]
    pub hat: Handle<Scene>,
    #[asset(path = "models/young.gltf#Scene0")]
    pub young: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct AnimationAssets {
    #[asset(path = "models/wizard.gltf#Animation0")]
    pub wizard_01: Handle<AnimationClip>,
    #[asset(path = "models/wizard.gltf#Animation1")]
    pub wizard_02: Handle<AnimationClip>,
    #[asset(path = "models/wizard.gltf#Animation2")]
    pub wizard_03: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "dry_ground.png")]
    pub ground: Handle<Image>,
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::GameStart),
        )
        .add_collection_to_loading_state::<_, SceneAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, AnimationAssets>(GameState::AssetLoading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::AssetLoading);
    }
}

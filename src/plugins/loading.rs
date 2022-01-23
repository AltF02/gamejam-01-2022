use crate::plugins::interactables::InteractablesResource;
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;
use ron::de::from_bytes;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<InteractablesResource>(include_bytes!(
                "../../data/scenes/interactables.ron"
            ))
            .unwrap(),
        );

        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub walking: Handle<AudioSource>,
    #[asset(path = "audio/radio_static.ogg")]
    pub radio_static: Handle<AudioSource>,
    #[asset(path = "audio/computer_startup.ogg")]
    pub computer_startup: Handle<AudioSource>,
    #[asset(path = "audio/computer_idle.ogg")]
    pub computer_idle: Handle<AudioSource>,
    #[asset(path = "audio/music.ogg")]
    pub background_music: Handle<AudioSource>,
    #[asset(path = "audio/explosion.ogg")]
    pub explosion: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/player.png")]
    pub texture_player: Handle<Image>,
    #[asset(path = "textures/room.png")]
    pub texture_room: Handle<Image>,
    #[asset(path = "textures/e_key.png")]
    pub texture_e_key: Handle<Image>,
}

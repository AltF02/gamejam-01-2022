#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::enum_glob_use)]

mod plugins;

use plugins::actions::ActionsPlugin;
use plugins::audio::InternalAudioPlugin;
use plugins::loading::LoadingPlugin;
use plugins::menu::MenuPlugin;
use plugins::player::PlayerPlugin;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::interactables::InteractablePlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use plugins::world::WorldPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(InteractablePlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

        #[cfg(debug_assertions)]
        {
            use bevy_inspector_egui::WorldInspectorPlugin;

            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(WorldInspectorPlugin::new());
            // .add_plugin(RapierRenderPlugin);
        }
    }
}

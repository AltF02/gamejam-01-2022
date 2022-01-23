use crate::plugins::player::PlayerComponent;
use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.label("camera_setup"))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(follow_player));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Camera);
}

fn follow_player(
    player: Query<&Transform, (Without<Camera>, With<PlayerComponent>)>,
    mut camera: Query<&mut Transform, (Without<PlayerComponent>, With<Camera>)>,
) {
    for pt in player.iter() {
        for mut ct in camera.iter_mut() {
            ct.translation = pt.translation;
        }
    }
}

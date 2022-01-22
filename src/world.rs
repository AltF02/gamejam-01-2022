use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_startup_system(setup.label("world_setup"));
    }
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 10.0;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
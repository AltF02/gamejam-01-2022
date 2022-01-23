use crate::plugins::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::physics::{ColliderBundle, RigidBodyBundle, RigidBodyPositionSync};
use bevy_rapier2d::prelude::{
    ColliderDebugRender, ColliderMaterial, ColliderMaterialComponent, ColliderShape,
    ColliderShapeComponent, RigidBodyMassPropsFlags, RigidBodyType, RigidBodyTypeComponent,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerComponent {
    pub speed: f32,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player));
    }
}

fn spawn_player(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands
        .spawn()
        .insert(PlayerComponent { speed: 1.5 })
        .insert_bundle(SpriteBundle {
            texture: texture_assets.texture_player.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(71., 99.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(10.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle((
            RigidBodyPositionSync::Discrete,
            Name::new("Player"),
            PlayerComponent { speed: 1.5 },
        ))
        .insert(ColliderDebugRender::default())
        .with_children(|parent| {
            parent.spawn().insert_bundle(ColliderBundle {
                shape: ColliderShapeComponent(ColliderShape::cuboid(3., 1.)),
                position: Vec2::new(0.0, -3.8).into(),
                material: ColliderMaterialComponent(ColliderMaterial {
                    friction: 0.0,
                    restitution: 0.0,
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
}

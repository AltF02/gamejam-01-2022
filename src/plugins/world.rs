use crate::plugins::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelState {
    Home,
    Requiem,
    Utopia,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup.label("world_setup")),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    texture_assets: Res<TextureAssets>,
) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 10.0;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: texture_assets.texture_room.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(800., 800.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"));

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec2::new(0., -37.5).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(40., 1.)),
            material: ColliderMaterialComponent(ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Bottom Wall"));

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec2::new(0., 23.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(40., 1.)),
            material: ColliderMaterialComponent(ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Top Wall"));

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec2::new(37.5, 0.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(1., 40.)),
            material: ColliderMaterialComponent(ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Right Wall"));

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec2::new(-37., 1.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(1., 40.)),
            material: ColliderMaterialComponent(ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(ColliderDebugRender::default())
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Left Wall"));

    // let type_registry = commands.get
}

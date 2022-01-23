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
        app.add_startup_system(setup.label("world_setup"));
    }
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vector::zeros();
    rapier_config.scale = 10.0;

    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: Vec2::new(-8., -50.).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(27.5, 1.)),
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
            position: Vec2::new(-5.3, 16.).into(),
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
            position: Vec2::new(20.6, -9.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(1., 27.5)),
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
            position: Vec2::new(-31., -9.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(1., 27.5)),
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

    // let type_registry = commands.get
}

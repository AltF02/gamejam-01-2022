// SPDX-License-Identifier: MIT
// © TheRealTeamFReSh
// Mostly copied code and modified to fit from https://github.com/TheRealTeamFReSh/MurderUserDungeon/blob/master/src/apartment/interactable.rs

use crate::plugins::player::PlayerComponent;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Hash, Clone, Debug, PartialEq, Eq)]
pub enum InteractableType {
    Desk,
    Bed,
    Radio,
    Plant,
}

#[derive(Deserialize)]
pub struct InteractableData {
    pub position: Vec2,
    pub collider_size: Vec2,
    pub range: f32,
}

#[derive(Deserialize)]
pub struct InteractablesResource {
    pub interactables: HashMap<InteractableType, InteractableData>,
}

#[derive(Component)]
pub struct InteractableComponent {
    pub interactable_type: InteractableType,
    pub range: f32,
}

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup.label("interactables_setup")),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(check_interactables_system),
        );
    }
}

fn setup(mut commands: Commands, interactables_resource: Res<InteractablesResource>) {
    spawn_item(
        &mut commands,
        InteractableType::Desk,
        &interactables_resource,
    );

    spawn_item(
        &mut commands,
        InteractableType::Bed,
        &interactables_resource,
    );

    spawn_item(
        &mut commands,
        InteractableType::Plant,
        &interactables_resource,
    );
}

fn spawn_item(
    commands: &mut Commands,
    interactable_type: InteractableType,
    interactables_resource: &InteractablesResource,
) {
    let interactable_data = &interactables_resource.interactables[&interactable_type];

    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type,
            range: interactable_data.range,
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: interactable_data.position.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShapeComponent(ColliderShape::cuboid(
                interactable_data.collider_size.x,
                interactable_data.collider_size.y,
            )),
            material: ColliderMaterialComponent(ColliderMaterial {
                friction: 0.0,
                restitution: 0.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::default())
        .insert(Name::new("Desk"));
}

pub fn check_interactables_system(
    interactable_query: Query<(&InteractableComponent, &Transform)>,
    mut player_query: Query<(&Transform, &mut PlayerComponent)>,
) {
    for (player_transform, mut player_component) in player_query.iter_mut() {
        let mut interactable_in_range: Option<InteractableType> = None;
        for (interactable_component, interactable_transform) in interactable_query.iter() {
            let interactable_position = Vec2::new(
                interactable_transform.translation.x,
                interactable_transform.translation.y,
            );
            let player_position = Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );

            // get distance between player and interactable
            let distance = interactable_position.distance(player_position);

            // set interactable in range if within distance
            if distance < interactable_component.range {
                interactable_in_range = Some(interactable_component.interactable_type.clone());
            }
        }

        let old_interactable = player_component.interactable_in_range.clone();
        player_component.interactable_in_range = interactable_in_range.clone();
        // spawn interact icon
        if old_interactable != interactable_in_range {
            if let Some(interactable_type) = interactable_in_range {
                println!("Interacting with {:?}", interactable_type)
            }
        }
    }
}

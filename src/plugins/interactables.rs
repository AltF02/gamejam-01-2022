// SPDX-License-Identifier: MIT
// © TheRealTeamFReSh
// Mostly copied code and modified to fit from https://github.com/TheRealTeamFReSh/MurderUserDungeon/blob/master/src/apartment/interactable.rs

mod desk;
mod plant;

use crate::plugins::interactables::desk::ComputerState;
use crate::plugins::loading::TextureAssets;
use crate::plugins::player::PlayerComponent;
use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::fmt::Formatter;

const INTERACTABLE_ICON_Z: f32 = 11.0;
const INTERACTABLE_ICON_SPRITE_SCALE: f32 = 2.5;
const INTERACTABLE_ICON_Y_OFFSET: f32 = 6.0;

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

impl std::fmt::Display for InteractableType {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> std::fmt::Result {
        match self {
            InteractableType::Bed => write!(f, "Bed"),
            InteractableType::Desk => write!(f, "Desk"),
            InteractableType::Plant => write!(f, "Plant"),
            InteractableType::Radio => write!(f, "Radio"),
        }
    }
}

#[derive(Deserialize)]
pub struct InteractablesResource {
    pub interactables: HashMap<InteractableType, InteractableData>,
}

#[derive(Component, Debug)]
pub struct InteractableComponent {
    pub interactable_type: InteractableType,
    pub range: f32,
    pub transform: Transform,
}

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ComputerState::Off)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup.label("interactables_setup")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(check_interactables_system.label("check_interactables")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(desk::interact_desk_system.after("check_interactables")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(plant::interact_plant_system.after("check_interactables")),
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
    let type_string = format!("{}", interactable_type.clone());

    commands
        .spawn()
        .insert(InteractableComponent {
            interactable_type: interactable_type.clone(),
            range: interactable_data.range,
            transform: Transform::from_translation(interactable_data.position.extend(0.)),
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyTypeComponent(RigidBodyType::Static),
            position: RigidBodyPositionComponent(interactable_data.position.into()),
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
        .insert(Name::new(type_string));
}

#[derive(Component)]
pub struct InteractableIconComponent;

pub fn check_interactables_system(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    interactables_resource: Res<InteractablesResource>,
    interactable_query: Query<&InteractableComponent>,
    mut player_query: Query<(&Transform, &mut PlayerComponent)>,
    interactable_icon_query: Query<Entity, With<InteractableIconComponent>>,
    rapier_config: Res<RapierConfiguration>,
) {
    for (player_transform, mut player_component) in player_query.iter_mut() {
        let mut interactable_in_range: Option<InteractableType> = None;
        for interactable_component in interactable_query.iter() {
            let interactable_position = Vec2::new(
                interactable_component.transform.translation.x * 10.,
                interactable_component.transform.translation.y * 10.,
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
                // spawn interact icon
                spawn_interact_icon(
                    texture_assets.texture_e_key.clone(),
                    &interactable_type,
                    &mut commands,
                    &interactables_resource,
                );
            } else {
                for interactable_icon_entity in interactable_icon_query.iter() {
                    commands.entity(interactable_icon_entity).despawn();
                }
            }
        }
    }
}

/// Spawn an interactable icon
fn spawn_interact_icon(
    texture: Handle<Image>,
    interactable_type: &InteractableType,
    commands: &mut Commands,
    interactables_resource: &InteractablesResource,
) {
    let interactable_data = &interactables_resource.interactables[interactable_type];
    let transform = Transform::from_translation(Vec3::new(
        interactable_data.position.x * 10.,
        interactable_data.position.y * 10. + 50.,
        1.,
    ));

    commands
        .spawn()
        .insert(InteractableIconComponent)
        .insert_bundle(SpriteBundle {
            texture,
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Interactable Icon"));
}

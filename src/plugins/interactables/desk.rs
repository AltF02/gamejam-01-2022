use crate::plugins::interactables::InteractableType;
use crate::plugins::player::PlayerComponent;
use crate::GameState;
use bevy::prelude::*;

pub fn interact_desk_system(
    player_query: Query<&PlayerComponent>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: ResMut<State<GameState>>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Desk) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E) && app_state.current() == &GameState::Playing
            {
                println!("Interacting with desk....");
            }
        }
    }
}

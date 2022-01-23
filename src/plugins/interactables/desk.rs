use crate::plugins::interactables::InteractableType;
use crate::plugins::loading::AudioAssets;
use crate::plugins::player::PlayerComponent;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use std::time::Duration;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum ComputerState {
    On,
    Off,
}

pub fn interact_desk_system(
    player_query: Query<&PlayerComponent>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: ResMut<State<GameState>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    computer_state: Res<ComputerState>,
) {
    for player_component in player_query.iter() {
        if let Some(InteractableType::Desk) = player_component.interactable_in_range {
            if keyboard_input.just_pressed(KeyCode::E) && app_state.current() == &GameState::Playing
            {
                audio.play_in_channel(
                    audio_assets.explosion.clone(),
                    &AudioChannel::new("computer".to_string()),
                );

                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_secs(4));
                    std::process::exit(0);
                });

                // if computer_state == ComputerState::Off {
                //     audio.play_looped_with_intro_in_channel(
                //         audio_assets.computer_startup.clone(),
                //         audio_assets.computer_idle.clone(),
                //         &AudioChannel::new("computer".to_string()),
                //     );
                // } else {
                // }
            }
        }
    }
}

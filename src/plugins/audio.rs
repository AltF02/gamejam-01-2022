use crate::plugins::actions::Actions;
use crate::plugins::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(control_walking_sound),
            );
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume_in_channel(1., &AudioChannel::new("background_music".to_string()));
    audio.play_looped_in_channel(
        audio_assets.background_music.clone(),
        &AudioChannel::new("background_music".to_string()),
    );

    audio.set_volume_in_channel(1., &AudioChannel::new("computer".to_string()));

    audio.set_volume_in_channel(1., &AudioChannel::new("walking".to_string()));
    audio.play_looped_in_channel(
        audio_assets.walking.clone(),
        &AudioChannel::new("walking".to_string()),
    );
    audio.pause_channel(&AudioChannel::new("walking".to_string()));
}

fn control_walking_sound(actions: Res<Actions>, audio: Res<Audio>) {
    if actions.player_movement {
        audio.resume_channel(&AudioChannel::new("walking".to_string()));
    } else {
        audio.pause_channel(&AudioChannel::new("walking".to_string()))
    }
}

use crate::actions::Actions;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            flying: AudioChannel::new("flying".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(control_flying_sound.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_audio.system()));
    }
}

struct AudioChannels {
    flying: AudioChannel,
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.set_volume_in_channel(0.3, &channels.flying);
    audio.play_looped_in_channel(audio_assets.flying.clone(), &channels.flying);
    audio.pause_channel(&channels.flying);
}

fn stop_audio(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.flying);
}

fn control_flying_sound(actions: Res<Actions>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    if actions.player_movement.is_some() {
        audio.resume_channel(&channels.flying);
    } else {
        audio.pause_channel(&channels.flying)
    }
}

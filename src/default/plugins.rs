use bevy::app::{PluginGroup, PluginGroupBuilder};
use crate::default::loading::LoadingPlugin;
use crate::default::menu::MenuPlugin;
use crate::default::actions::ActionsPlugin;
use crate::default::audio::InternalAudioPlugin;
use crate::default::player::PlayerPlugin;
use crate::default::window::StarterWindowPlugin;

pub struct StarterPlugins;

impl PluginGroup for StarterPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(StarterWindowPlugin)
            .add(LoadingPlugin)
            .add(MenuPlugin)
            .add(ActionsPlugin)
            .add(InternalAudioPlugin)
            .add(PlayerPlugin);
    }
}


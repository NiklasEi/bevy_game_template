use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::winit::WinitSettings;
use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

#[unsafe(no_mangle)]
unsafe extern "C" fn main_rs() {
    main();
}

// this macro is a no-op on ios and only needed for anroid since bevy 0.16
// see https://github.com/bevyengine/bevy/pull/14780
#[bevy_main]
fn main() {
    #[cfg(target_os = "ios")]
    unsafe {
        // Sets our audio session to Ambient mode to prevent background music from stopping.
        // The default for iOS apps is SoloAmbient, which stops background music.
        // See apple docs: https://developer.apple.com/documentation/avfaudio/avaudiosession/category-swift.struct/ambient
        if let Err(e) = objc2_avf_audio::AVAudioSession::sharedInstance()
            .setCategory_error(objc2_avf_audio::AVAudioSessionCategoryAmbient.unwrap())
        {
            println!("Error setting audio session category: {:?}", e);
        }
    }

    App::new()
        .insert_resource(WinitSettings::mobile())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}

use bevy::app::{Plugin, App};
use bevy::core_pipeline::clear_color::ClearColor;
use bevy::window::{WindowDescriptor, WindowId};
use bevy::render::view::Msaa;
use bevy::render::color::Color;
use bevy::ecs::system::NonSend;
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

/// This will create the default window for you.
pub struct StarterWindowPlugin;

impl Plugin for StarterWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Msaa { samples: 1 })
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .insert_resource(WindowDescriptor {
                width: 800.,
                height: 600.,
                title: "Bevy game".to_string(), // ToDo
                canvas: Some("#bevy".to_owned()),
                ..Default::default()
            })
            .add_startup_system(set_window_icon);
    }
}

/// Set the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
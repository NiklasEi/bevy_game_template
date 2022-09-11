use bevy::app::{Plugin, App};
use bevy::ecs::system::Commands;
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::transform::components::Transform;
use bevy::math::Vec3;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera);
    }
}

/// spawn a camera into the world (and implicitly possess it)
fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
}

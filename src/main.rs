mod boid;
mod camera;
pub mod constants;
mod player;
mod trail;
mod window_resize;

use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.0, 0.03)))
        .insert_resource(boid::Settings::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(boid::BoidPlugin)
        .add_plugins(trail::TrailPlugin)
        .add_plugins(ResourceInspectorPlugin::<boid::Settings>::new())
        .run();
}

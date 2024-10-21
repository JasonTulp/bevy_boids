mod hello_plugin;
mod player;
mod camera;
pub mod constants;
mod enemy;
mod trail;
mod window_resize;

use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

fn main() {
    App::new()
        // .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.0, 0.03)))
        .insert_resource(enemy::Settings::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(PolylinePlugin)
        // .add_plugins(HanabiPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(trail::TrailPlugin)
        // .add_plugins(window_resize::WindowResizePlugin)
        .add_plugins(ResourceInspectorPlugin::<enemy::Settings>::new())
        .run();
}
//
//
// fn window_resize(resize_event: Res<Events<WindowResized>>, mut window: ResMut<Window>) {
//     let mut event_reader = resize_event.get_reader();
//     for event in event_reader.iter(&resize_event) {
//         window.width = event.width.try_into().unwrap();
//         window.height = event.height.try_into().unwrap();
//     }
// }
use bevy::{prelude::*, window::WindowResized};

#[derive(Default, Resource)]
pub struct WindowState {
    pub width: f32,
    pub height: f32,
}

fn update_window_size_system(
    mut state: ResMut<WindowState>,
    mut event_reader: EventReader<WindowResized>,
    // mut window_desc: ResMut<WindowState>,
) {
    for event in event_reader.read() {
        state.width = event.width;
        state.height = event.height;
        println!("Window resized to: {} x {}", state.width, state.height);
    }
}

pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowState::default())
            .add_systems(Update, update_window_size_system);
    }
}

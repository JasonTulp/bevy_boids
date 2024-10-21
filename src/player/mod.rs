pub mod components;
pub mod systems;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_player);
        app.add_systems(Update, systems::move_player);
    }
}
use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_spatial::{AutomaticUpdate, SpatialStructure};
use std::time::Duration;

mod boid;
mod camera;
pub mod constants;
mod player;
mod spatial_hash_map;
mod trail;
mod window_resize;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.0, 0.03)))
        .insert_resource(boid::Settings::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(boid::BoidPlugin)
        .add_plugins(trail::TrailPlugin)
        .add_plugins(ResourceInspectorPlugin::<boid::Settings>::new())
        .add_plugins(FpsCounterPlugin)
        // TODO Replace with a spatial hash grid
        .add_plugins(
            AutomaticUpdate::<crate::boid::Boid>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(100)),
        )
        .run();
}

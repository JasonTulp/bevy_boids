mod systems;

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::{FollowEntity, SimpleTrail2D, TrailBuilder, TrailColour, TrailPlugin};
}

pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, systems::update_trail);
    }
}

#[derive(Component)]
pub struct FollowEntity(Entity);

#[derive(Component)]
pub struct SimpleTrail2D {
    pub thickness: f32,
    pub local_offset: Vec2,
    pub points: Vec<Vec2>,
    pub taper_end: bool,
}

impl SimpleTrail2D {
    pub fn new(segments: u16, thickness: f32, spawn_pos: Vec2, local_offset: Vec2) -> Self {
        let vec = vec![spawn_pos; segments as usize];
        Self {
            thickness,
            local_offset,
            points: vec,
            taper_end: true,
        }
    }
}

/// Used to build a trail entity
pub struct TrailBuilder {
    /// The entity that this trail is attached to.
    /// It will follow this entity in PostUpdate
    follow_entity: Entity,
    /// How many segments the trail will have
    segments: u16,
    /// The thickness of the trail at the thickest point
    thickness: f32,
    /// Whether the trail should taper at the end (Reduce thickness to 0)
    taper_end: bool,
    /// The position of the trail when it is spawned
    spawn_pos: Vec2,
    /// The local offset from the follow entity, takes into account entity rotation
    local_offset: Vec2,
    /// The colour of the trail, can be single or gradient
    colour: TrailColour,
    /// The Z-depth of the trail
    depth: f32,
}

#[allow(dead_code)]
impl TrailBuilder {
    /// Create a new TrailBuilder with default values
    pub fn new(follow_entity: Entity, spawn_pos: Vec2) -> Self {
        Self {
            follow_entity,
            segments: 100,
            thickness: 1.0,
            taper_end: true,
            spawn_pos,
            local_offset: Vec2::ZERO,
            colour: TrailColour::single(Color::WHITE),
            depth: -1.0,
        }
    }

    /// Build the Trail Renderer and return the spawned entity
    pub fn build(
        self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Entity {
        let trail_renderer = SimpleTrail2D::new(
            self.segments,
            self.thickness,
            self.spawn_pos,
            self.local_offset,
        );
        let colours = self.colour.get_vertex_colours(self.segments);
        let trail_mesh = meshes.add(
            Mesh::new(
                PrimitiveTopology::TriangleStrip,
                RenderAssetUsages::default(),
            )
            // .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![])
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours),
        );
        let trail_mat = materials.add(ColorMaterial::from_color(Color::WHITE));
        let transform = Transform::from_xyz(0.0, 0.0, self.depth);
        commands.spawn((
            ColorMesh2dBundle {
                mesh: trail_mesh.clone().into(),
                material: trail_mat,
                transform,
                ..default()
            },
            trail_renderer,
            FollowEntity(self.follow_entity),
        )).id()
    }

    /// Set the number of segments in the trail
    pub fn with_segments(mut self, segments: u16) -> Self {
        self.segments = segments;
        self
    }

    /// Set the thickness of the trail
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set whether the trail should taper at the end
    pub fn with_taper_end(mut self, taper_end: bool) -> Self {
        self.taper_end = taper_end;
        self
    }

    /// Set the spawn position of the trail
    pub fn with_local_offset(mut self, local_offset: Vec2) -> Self {
        self.local_offset = local_offset;
        self
    }

    /// Set the colour of the trail
    pub fn with_colour(mut self, colour: TrailColour) -> Self {
        self.colour = colour;
        self
    }

    /// Set the Z-depth of the trail
    pub fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }
}

/// Colour of the trail, can be a single colour or gradient
pub enum TrailColour {
    Gradient { start: Color, end: Color },
    Single(Color),
}

impl TrailColour {
    pub fn gradient(start: Color, end: Color) -> Self {
        Self::Gradient { start, end }
    }

    pub fn single(colour: Color) -> Self {
        Self::Single(colour)
    }

    pub fn get_vertex_colours(&self, segments: u16) -> Vec<[f32; 4]> {
        match self {
            Self::Gradient { start, end } => Self::get_gradient(start, end, segments),
            Self::Single(colour) => vec![colour.to_srgba().to_f32_array(); segments as usize * 2],
        }
    }

    fn get_gradient(start: &Color, end: &Color, segments: u16) -> Vec<[f32; 4]> {
        let mut colours: Vec<[f32; 4]> = Vec::with_capacity(segments as usize * 2);
        for i in 0..segments {
            let c1 = end.to_srgba();
            let c2 = start.to_srgba();
            let red = c1.red + (c2.red - c1.red) * i as f32 / segments as f32;
            let green = c1.green + (c2.green - c1.green) * i as f32 / segments as f32;
            let blue = c1.blue + (c2.blue - c1.blue) * i as f32 / segments as f32;
            let alpha = c1.alpha + (c2.alpha - c1.alpha) * i as f32 / segments as f32;
            colours.push([red, green, blue, alpha]);
            colours.push([red, green, blue, alpha]);
        }
        colours
    }
}

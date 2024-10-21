
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::Mesh2dHandle;
use crate::enemy::Boid;
use crate::player::components::Player;


pub struct TrailPlugin;

impl Plugin for TrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_trail);
    }
}

#[derive(Component)]
pub struct TrailRenderer {
    pub segments: u16,
    pub thickness: f32,
    pub min_distance: f32,
    pub local_offset: Vec2,
    pub points: Vec<Vec2>,
    pub taper_end: bool,
}

#[derive(Component)]
pub struct FollowEntity(Entity);

impl TrailRenderer {
    pub fn new(segments: u16, thickness: f32, spawn_pos: Vec2, local_offset: Vec2) -> Self {
        let vec = vec![spawn_pos; segments as usize];
        Self {
            segments,
            thickness,
            min_distance: 2.0,
            local_offset,
            points: vec,
            taper_end: true,
        }
    }
}

/// Call this to create the trail entity, returns the entity ID to be parented
pub fn spawn_trail(
    follow_entity: Entity,
    segments: u16,
    thickness: f32,
    commands: &mut Commands,
    spawn_pos: Vec2,
    local_offset: Vec2,
    colour_2: Color,
    colour_1: Color,
    depth: f32,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    // TODO segments must be greater than 1
    let trail_renderer = TrailRenderer::new(segments, thickness, spawn_pos, local_offset);
    let vertices: Vec<Vec3> = vec![];

    let mut colours: Vec<[f32; 4]> = Vec::with_capacity(segments as usize * 2);
    for i in 0..segments {
        let c1 = colour_1.to_srgba();
        let c2 = colour_2.to_srgba();
        let red = c1.red + (c2.red - c1.red) * i as f32 / segments as f32;
        let green = c1.green + (c2.green - c1.green) * i as f32 / segments as f32;
        let blue = c1.blue + (c2.blue - c1.blue) * i as f32 / segments as f32;
        let alpha = c1.alpha + (c2.alpha - c1.alpha) * i as f32 / segments as f32;
        colours.push([red, green, blue, alpha]);
        colours.push([red, green, blue, alpha]);
    }

    let trail_mesh = meshes.add(
        Mesh::new(PrimitiveTopology::TriangleStrip, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    );
    let trail_mat = materials.add(ColorMaterial::from_color(Color::WHITE));
    let transform = Transform::from_xyz(0.0, 0.0, depth);
    commands
        .spawn((
            ColorMesh2dBundle {
                mesh: trail_mesh.clone().into(),
                material: trail_mat,
                transform,
                ..default()
            },
            trail_renderer,
            FollowEntity(follow_entity),
        ));
}

fn update_trail(
    boids: Query<&Transform>,
    mut query: Query<(&mut TrailRenderer, &mut Mesh2dHandle, &FollowEntity)>,
    mut assets: ResMut<Assets<Mesh>>
) {

    for (mut trail_renderer, mut mesh, follow_entity) in query.iter_mut() {
        let follow = boids.get(follow_entity.0).expect("Follow entity not found");
        let mut vertices: Vec<Vec3> = Vec::with_capacity(trail_renderer.points.len() * 2);
        // Get offset based on rotation of follow
        let offset = follow.rotation.mul_vec3(Vec3::new(trail_renderer.local_offset.x, trail_renderer.local_offset.y, 0.0));
        let offset = offset.xy();


        // Update the trail points from the end to the start
        for i in (1..trail_renderer.points.len()).rev() {
            let thickness = match trail_renderer.taper_end {
                true => {
                    // THe higher i is, the smaller the thickness is
                    let t = i as f32 / trail_renderer.points.len() as f32;
                    trail_renderer.thickness - trail_renderer.thickness * t
                }
                false => trail_renderer.thickness,
            };
            let dir = trail_renderer.points[i - 1] - trail_renderer.points[i];
            // let distance = dir.length();
            // if distance >= trail_renderer.min_distance {
            // }
            let perp = Vec3::new(dir.y, -dir.x, 0.0).normalize() * thickness;
            trail_renderer.points[i] = trail_renderer.points[i - 1];
            let point_vec3 = Vec3::new(trail_renderer.points[i].x, trail_renderer.points[i].y, 0.0);
            vertices.push(point_vec3 - perp);
            vertices.push(point_vec3 + perp);
        }
        // Add the new point at the start
        let new_pos = follow.translation.xy() + offset;
        let dir = new_pos - trail_renderer.points[0];
        let perp = Vec3::new(dir.y, -dir.x, 0.0).normalize() * trail_renderer.thickness / 2.0;
        trail_renderer.points[0] = new_pos;
        let point_vec3 = Vec3::new(trail_renderer.points[0].x, trail_renderer.points[0].y, 0.0);
        vertices.push(point_vec3 - perp);
        vertices.push(point_vec3 + perp);

        // Update the mesh
        let mesh = assets.get_mut(mesh.id()).unwrap();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    }
}

use bevy::sprite::Mesh2dHandle;
use bevy::prelude::*;
use super::*;

/// Update all the points in the trail based on the follow entity
/// If the FollowEntity does not exist, the trail will be destroyed
pub (crate) fn update_trail(
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut query: Query<(&mut SimpleTrail2D, &mut Mesh2dHandle, &FollowEntity, Entity)>,
    mut assets: ResMut<Assets<Mesh>>,
) {
    for (mut trail_renderer, mesh, follow_entity, entity) in query.iter_mut() {
        let Ok(follow) = transforms.get(follow_entity.0) else {
            // FollowEntity not found, remove the trail
            commands.entity(entity).despawn();
            continue
        };

        let mut vertices: Vec<Vec3> = Vec::with_capacity(trail_renderer.points.len() * 2);
        // Get offset based on rotation of follow
        let offset = follow.rotation.mul_vec3(Vec3::new(
            trail_renderer.local_offset.x,
            trail_renderer.local_offset.y,
            0.0,
        ));
        let offset = offset.xy();

        // Update the trail points from the end to the start
        for i in (1..trail_renderer.points.len()).rev() {
            let new_pos = trail_renderer.points[i - 1].xy();
            update_trail_point(i, new_pos, &mut trail_renderer, &mut vertices);
        }

        // Add the new point at the start
        let new_pos = follow.translation.xy() + offset;
        update_trail_point(0, new_pos, &mut trail_renderer, &mut vertices);

        // Update the mesh
        let mesh = assets.get_mut(mesh.id()).unwrap();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    }
}

/// Update a single points position and calculate the position of the corresponding vertices
/// Rotation is calculated based on the direction of the new position from the old position
fn update_trail_point(
    i: usize,
    new_pos: Vec2,
    trail_renderer: &mut SimpleTrail2D,
    vertices: &mut Vec<Vec3>,
) {
    let thickness = match trail_renderer.taper_end {
        true => {
            let t = i as f32 / trail_renderer.points.len() as f32;
            trail_renderer.thickness - trail_renderer.thickness * t
        }
        false => trail_renderer.thickness,
    };
    let dir = new_pos - trail_renderer.points[i];
    let perp = Vec3::new(dir.y, -dir.x, 0.0).normalize() * thickness / 2.0;
    trail_renderer.points[i as usize] = new_pos;
    let point_vec3 = Vec3::new(
        trail_renderer.points[i as usize].x,
        trail_renderer.points[i as usize].y,
        0.0,
    );
    vertices.push(point_vec3 - perp);
    vertices.push(point_vec3 + perp);
}

use crate::constants::{BOID_COUNT, ENEMY_SPEED};
use crate::player::components::Player;
use crate::trail::spawn_trail;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::prelude::*;
use rand::random;

pub struct BoidPlugin;

#[derive(Component, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone)]
pub struct MaxVelocity(pub f32);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

/// Simulation settings; everything can be updated through UI except the number of boids
#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Settings {
    /// Radius of the circle in which boids can see
    #[inspector(min = 0., max = 10000., speed = 100.)]
    visual_radius: f32,
    /// Radius of the circle in which boids wants to be alone
    #[inspector(min = 0., max = 1000., speed = 10.)]
    separation_radius: f32,
    /// Cohesion rule : boids move toward the center of mass of their neighbors
    #[inspector(min = 0., max = 1., speed = 0.01)]
    cohesion: f32,
    /// Separation rule: boids move away from other boids that are in protected range
    #[inspector(min = 0., max = 1., speed = 0.01)]
    separation: f32,
    /// Alignment rule: boids try to match the average velocity of boids located in its visual range
    #[inspector(min = 0., max = 1., speed = 0.01)]
    alignment: f32,
    /// Max boids speed
    #[inspector(min = 0., max = 1000., speed = 10.)]
    max_speed: f32,
    /// Min boids speed
    #[inspector(min = 0., max = 1000., speed = 10.)]
    max_force: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            visual_radius: 50.0,
            separation_radius: 30.0,
            cohesion: 0.6,
            separation: 0.8,
            alignment: 0.6,
            max_speed: 100.0,
            max_force: 50.0,
        }
    }
}

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boids);
        app.add_systems(Update, boid_update);
        app.add_systems(Update, boid_flock);
    }
}

#[derive(Component, Clone)]
pub struct Boid {
    // How many boids does this boid count for?
    pub(crate) weight: f32,
}

/// Spawn BOID_COUNT amount of boids
pub fn spawn_boids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().expect("No window found");
    for _ in 0..BOID_COUNT {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();
        let transform = Transform::from_xyz(x, y, 0.0);
        let velocity = Velocity(Vec2::new(
            ENEMY_SPEED * (random::<f32>() - 0.5),
            ENEMY_SPEED * (random::<f32>() - 0.5),
        ));
        let acceleration = Acceleration(Vec2::ZERO);
        let mesh = Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 4.0,
            Vec2::new(-3.5, -4.0),
            Vec2::new(3.5, -4.0),
        )));
        let material = materials.add(Color::srgb_u8(255, 221, 0));
        let boid = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh,
                    material,
                    transform,
                    ..default()
                },
                Boid { weight: 1. },
                velocity,
                acceleration,
            ))
            .id();

        spawn_trail(
            boid,
            100,
            2.5,
            &mut commands,
            transform.translation.xy(),
            Vec2::new(0.0, -4.0),
            Color::srgba_u8(255, 55, 0, 255),
            Color::srgba_u8(255, 0, 0, 0),
            -2.0 - random::<f32>() * 100.0,
            &mut materials,
            &mut meshes,
        );
    }
}

/// Update the boids position, velocity and rotation
pub fn boid_update(
    mut boid_query: Query<(
        &mut Transform,
        &mut Velocity,
        &Acceleration,
        &Boid,
        Option<&MaxVelocity>,
    )>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, acceleration, _, max_velocity) in boid_query.iter_mut() {
        // Update position and velocity
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
        velocity.0 += acceleration.0 * time.delta_seconds();

        // Limit velocity to max velocity if specified
        if let Some(max_velocity) = max_velocity {
            limit_vec(&mut velocity.0, max_velocity.0);
        }

        let direction = Vec2::new(velocity.0.x, velocity.0.y).normalize();
        if direction.length() > 0.0 {
            // Update rotation to facing direction
            transform.rotation = Quat::from_rotation_z(-direction.x.atan2(direction.y));
        }
    }
}

/// Flock the boids by following the alignment, cohesion and separation rules
pub fn boid_flock(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, With<Camera>>,
    mut boid_query: Query<(
        &mut Acceleration,
        &Transform,
        &Velocity,
        &Boid,
        Option<&Player>,
    )>,
    settings: Res<Settings>,
) {
    let boid_data: Vec<(Transform, Velocity, Boid)> = boid_query
        .iter()
        .map(|(_, t, v, b, _)| (t.clone(), v.clone(), b.clone()))
        .collect();
    let camera_transform = camera_query.get_single().expect("No camera found");
    let window = window_query.get_single().expect("No window found");

    for (mut acceleration, transform, velocity, _, player) in boid_query.iter_mut() {
        // we don't want to change the player force, but we want to incorporate it into the flocking
        // behaviour
        if player.is_some() {
            continue;
        }

        let mut alignment = Vec2::ZERO;
        let mut cohesion = Vec2::ZERO;
        let mut separation = Vec2::ZERO;
        let mut count = 0;
        let mut separation_count = 0;
        let position: Vec2 = transform.translation.truncate();

        // Iterate through every boid and calculate the alignment, cohesion and separation
        for (other_transform, other_velocity, boid) in boid_data.iter() {
            let other_position: Vec2 = other_transform.translation.truncate();
            let weight = boid.weight;
            if position == other_position {
                continue;
            }
            let distance = position.distance(other_position);
            if distance < settings.visual_radius {
                alignment += other_velocity.0 * weight;
                cohesion += other_position * weight;

                count += 1;
            }

            if distance < settings.separation_radius {
                let mut diff = position - other_position;
                diff /= distance.max(0.000001);
                separation += diff * weight;
                separation_count += 1;
            }
        }

        // Exit out if there are no boids within the radius, this will just keep the current velocity
        if count > 0 {
            // Set the alignment to a direction multiplied by max speed so we are always travelling at
            // max speed. This can be removed if you want the average velocity
            alignment /= count as f32;
            alignment = alignment.normalize() * settings.max_speed;
            alignment -= velocity.0;
            limit_vec(&mut alignment, settings.max_force);
            alignment *= settings.alignment;

            // Cohesion is the average position of all the boids within the set radius
            // Add the velocity to move towards the average position
            cohesion /= count as f32;
            cohesion -= position;
            cohesion = cohesion.normalize() * settings.max_speed;
            cohesion -= velocity.0;
            limit_vec(&mut cohesion, settings.max_force);
            cohesion *= settings.cohesion;
        }

        if separation_count > 0 {
            // separation is a force in the direction away from all neighbouring boids
            separation /= separation_count as f32;
            separation = separation.normalize() * settings.max_speed;
            separation -= velocity.0;
            limit_vec(&mut separation, settings.max_force);
            separation *= settings.separation;
        }

        // Check window bounds and apply force inwards if we are outside of them
        const MARGIN: f32 = -10.0;
        let min_x = camera_transform.translation.x - window.width() / 2.0 - MARGIN;
        let max_x = camera_transform.translation.x + window.width() / 2.0 + MARGIN;
        let min_y = camera_transform.translation.y - window.height() / 2.0 - MARGIN;
        let max_y = camera_transform.translation.y + window.height() / 2.0 + MARGIN;
        let mut border_adjustment = Vec2::new(
            if transform.translation.x < min_x {
                settings.max_speed
            } else {
                0.0
            } + if transform.translation.x > max_x {
                -settings.max_speed
            } else {
                0.0
            },
            if transform.translation.y < min_y {
                settings.max_speed
            } else {
                0.0
            } + if transform.translation.y > max_y {
                -settings.max_speed
            } else {
                0.0
            },
        );
        border_adjustment *= 2.0;

        *acceleration = Acceleration(alignment + cohesion + separation + border_adjustment);
    }
}

/// Limit a Vec2's magnitude to max
pub fn limit_vec(velocity: &mut Vec2, max: f32) {
    let speed = velocity.length();
    if speed > max {
        *velocity = velocity.normalize() * max;
    }
}

/// Wrap the boids around the screen so they don't go off screen
/// Note. using window bounds as barriers in flock function
#[allow(dead_code)]
pub fn wrap_around_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut boid_query: Query<(&mut Transform, &Boid)>,
) {
    const MARGIN: f32 = 10.0;
    let window = window_query.get_single().expect("No window found");
    for (mut transform, _) in boid_query.iter_mut() {
        if transform.translation.x < -MARGIN {
            transform.translation.x += window.width() + 2f32 * MARGIN;
        }
        if transform.translation.x > window.width() + MARGIN {
            transform.translation.x -= window.width() + 2f32 * MARGIN;
        }
        if transform.translation.y < -MARGIN {
            transform.translation.y += window.height() + 2f32 * MARGIN;
        }
        if transform.translation.y > window.height() + MARGIN {
            transform.translation.y -= window.height() + 2f32 * MARGIN;
        }
    }
}

/// Move boid towards the player Transform
#[allow(dead_code)]
pub fn move_boid_to_player(
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut boid_query: Query<(&mut Transform, &Boid), Without<Player>>,
    time: Res<Time>,
) {
    let player_position = player_query.get_single().unwrap().1.translation;
    for (mut transform, _) in boid_query.iter_mut() {
        let direction = (player_position - transform.translation).normalize();
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

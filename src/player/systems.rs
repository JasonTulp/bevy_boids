use crate::boid::{Acceleration, Boid, MaxVelocity, Velocity};
use crate::constants::{PLAYER_FORCE, PLAYER_MAX_SPEED};
use crate::player::components::Player;
use crate::trail::spawn_trail;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().expect("No window found");
    let transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 3.0);
    let mesh = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 8.0,
        Vec2::new(-8.0, -8.0),
        Vec2::new(8.0, -8.0),
    )));
    let material = materials.add(Color::srgb_u8(0, 200, 255));
    let acceleration = Acceleration(Vec2::ZERO);
    let velocity = Velocity(Vec2::ZERO);
    let player = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh,
                material,
                transform,
                ..default()
            },
            Player,
            acceleration,
            velocity,
            MaxVelocity(PLAYER_MAX_SPEED),
            Boid { weight: 10000.0 },
        ))
        .id();

    spawn_trail(
        player,
        100,
        1.0,
        &mut commands,
        transform.translation.xy(),
        Vec2::new(7.0, -8.0),
        Color::srgba_u8(0, 200, 255, 255),
        Color::srgba_u8(148, 22, 250, 0),
        -1.0,
        &mut materials,
        &mut meshes,
    );

    spawn_trail(
        player,
        100,
        1.0,
        &mut commands,
        transform.translation.xy(),
        Vec2::new(-7.0, -8.0),
        Color::srgba_u8(0, 200, 255, 255),
        Color::srgba_u8(148, 22, 250, 0),
        -1.0,
        &mut materials,
        &mut meshes,
    );
}

/// Move player based on WASD or Arrow Key input
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Acceleration, &Velocity, &Player)>,
    time: Res<Time>,
) {
    if let Ok((_, mut acceleration, velocity, _)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        let mut new_acceleration = direction * PLAYER_FORCE * time.delta_seconds() * 200.0;
        new_acceleration = new_acceleration - velocity.0;
        acceleration.0 = new_acceleration;
    }
}

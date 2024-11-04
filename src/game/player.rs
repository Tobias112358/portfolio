use std::marker;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub struct Speed(u8);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    camera: Camera3dBundle,
    speed: Speed,
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(
            PlayerBundle {
                marker: Player,
                camera: Camera3dBundle::default(),
                speed: Speed(3)
            }
        );
}

pub fn move_player(
    time: Res<Time>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut mouse_motion: EventReader<MouseMotion>, 
    mut speed: Query<&mut Speed, With<Player>>,
    mut camera: Query<&mut Transform, With<Player>>
) {
    let mut camera_transform = camera.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        camera_transform.rotate_y(yaw);
        camera_transform.rotate_local_x(pitch);
    }

    //camera move.
    if input.pressed(KeyCode::KeyW) {
        let forward_vec = camera_transform.forward().as_vec3();
        camera_transform.translation += forward_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    } else if input.pressed(KeyCode::KeyS) {
        let forward_vec = camera_transform.forward().as_vec3();
        camera_transform.translation -= forward_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    }
    if input.pressed(KeyCode::KeyD) {
        let right_vec = camera_transform.right().as_vec3();
        camera_transform.translation += right_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    } else if input.pressed(KeyCode::KeyA) {
        let right_vec = camera_transform.right().as_vec3();
        camera_transform.translation -= right_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    }
    if input.pressed(KeyCode::KeyE) {
        let up_vec = camera_transform.up().as_vec3();
        camera_transform.translation += up_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    } else if input.pressed(KeyCode::KeyQ) {
        let up_vec = camera_transform.up().as_vec3();
        camera_transform.translation -= up_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    }
}
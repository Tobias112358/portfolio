use std::marker;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::render::view::RenderLayers;
use bevy::color::palettes::tailwind;
use bevy::pbr::NotShadowCaster;


#[derive(Component)]
pub struct Speed(u8);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    space: SpatialBundle,
    speed: Speed,
}

//Camera markers

#[derive(Component)]
pub struct WorldModelCamera;

#[derive(Component)]
pub struct ViewModelCamera;

//Text Marker
#[derive(Component)]
pub struct ForwardText;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));
    
    commands.spawn(
        PlayerBundle {
            marker: Player,
            space: SpatialBundle {
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            },
            speed: Speed(3)
        }
    )
    .with_children(|parent| {
        
        //Spawn world model camera
        parent.spawn((
            WorldModelCamera,
            Camera3dBundle {
                
                camera: Camera {
                    // Bump the order to render on top of the world model.
                    order: 0,
                    ..default()
                },
                projection: PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }
                .into(),
                ..default()
            },
        ));

        //Spawn view model camera
        parent.spawn((
            ViewModelCamera,
            Camera3dBundle {
                camera: Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                projection: PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }
                .into(),
                ..default()
            },
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        ));

        // Spawn the player's right arm.
        parent.spawn((
            MaterialMeshBundle {
                mesh: arm,
                material: arm_material,
                transform: Transform::from_xyz(0.2, -0.1, -0.25),
                ..default()
            },
            // Ensure the arm is only rendered by the view model camera.
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            // The arm is free-floating, so shadows would look weird.
            NotShadowCaster,
        ));
    });
    
    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Forward: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                    ..default()
                }
            } else {
                // "default_font" feature is unavailable, load a font to use instead.
                TextStyle {
                    font_size: 60.0,
                    ..default()
                }
            }),
        ]),
        ForwardText,
    ));

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
        let mut forward_vec = camera_transform.forward().as_vec3().clone();

        forward_vec.y = 0.0;

        camera_transform.translation += forward_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    } else if input.pressed(KeyCode::KeyS) {
        let mut forward_vec = camera_transform.forward().as_vec3().clone();

        forward_vec.y = 0.0;
        camera_transform.translation -= forward_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    }
    if input.pressed(KeyCode::KeyD) {
        let mut right_vec = camera_transform.right().as_vec3().clone();

        right_vec.y = 0.0;
        camera_transform.translation += right_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    } else if input.pressed(KeyCode::KeyA) {
        let mut right_vec = camera_transform.right().as_vec3().clone();

        right_vec.y = 0.0;
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

pub fn forward_text_update(
    mut texts: Query<&mut Text, With<ForwardText>>,
    mut transform: Query<&mut Transform, With<Player>>
) {
    let player_transform = transform.single();
    for mut text in &mut texts {
        let forward_vector = player_transform.forward().as_vec3();
        text.sections[1].value = format!("x: {:.2}, y: {:.2}, z: {:.2}", forward_vector.x, forward_vector.y, forward_vector.z);
        
    }
}
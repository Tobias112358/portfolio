use std::marker;
use std::time::Duration;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::render::view::RenderLayers;
use bevy::color::palettes::tailwind;
use bevy::pbr::NotShadowCaster;
use bevy::animation::{AnimationTargetId, RepeatAnimation};
use bevy::scene::SceneInstanceReady;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};


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

#[derive(Component)]
pub struct Arm;

//Resouirces
#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    
    let arm = asset_server.load("Arm.glb#Scene0");


    //Animations
    let mut graph = AnimationGraph::new();
    let animations = graph.add_clips(
        [
            GltfAssetLabel::Animation(0).from_asset("Arm.glb"),
            GltfAssetLabel::Animation(1).from_asset("Arm.glb"),
        ]
        .into_iter()
        .map(|path| asset_server.load(path)),
        1.0,
        graph.root,
    )
    .collect();

    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });


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
                    fov: 73.0_f32.to_radians(),
                    ..default()
                }
                .into(),
                ..default()
            },
            RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        ));

        // Spawn the player's right arm.
        parent.spawn((
            Arm,
            HookedSceneBundle {
                scene: SceneBundle {
                    scene: arm,
                    transform: Transform::from_xyz(0.22, -0.22, -0.28)
                        .with_rotation(Quat::from_rotation_y(-0.4 * std::f32::consts::PI))
                        .with_scale(Vec3::new(0.1,0.1,0.1)),
                    ..default()
                },
                hook: SceneHook::new(|_, commands| {
                    commands.insert(RenderLayers::layer(VIEW_MODEL_RENDER_LAYER));
                    commands.insert(NotShadowCaster);
                }),
            },
            
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
            TextSection::new(
                "\nArmPos: ",
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
            TextSection::new(
                "\nFOV: ",
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

pub fn debug_move_arm(
    input: Res<ButtonInput<KeyCode>>, 
    mut arm: Query<&mut Transform, With<Arm>>,
    mut camera: Query<&mut Projection, With<ViewModelCamera>>
) {
    let mut transform = arm.single_mut();
    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 0.01;
    }
    if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 0.01;
    }
    if input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 0.01;
    }
    if input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 0.01;
    }
    if input.pressed(KeyCode::Equal) {
        transform.translation.z += 0.01;
    }
    if input.pressed(KeyCode::Minus) {
        transform.translation.z -= 0.01;
    }
    if input.pressed(KeyCode::PageUp) {
        let mut camera_perspective = camera.single_mut();
        let Projection::Perspective(ref mut perspective) = camera_perspective.as_mut() else {
            unreachable!(
                "The `Projection` component was explicitly built with `Projection::Perspective`"
            );
        };
    
        perspective.fov += 1.0_f32.to_radians();
    }
    if input.pressed(KeyCode::PageDown) {
        let mut camera_perspective = camera.single_mut();
        let Projection::Perspective(ref mut perspective) = camera_perspective.as_mut() else {
            unreachable!(
                "The `Projection` component was explicitly built with `Projection::Perspective`"
            );
        };
    
        perspective.fov -= 1.0_f32.to_radians();
    }
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

pub fn player_actions(
    input: Res<ButtonInput<KeyCode>>, 
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {

    for (mut player, mut transitions) in &mut animation_players {
        let Some((&playing_animation_index, &active_animation)) = player.playing_animations().next() else {
            continue;
        };
        if mouse_button.just_pressed(MouseButton::Left) {
            *current_animation = 1;

            transitions
                .play(
                    &mut player,
                    animations.animations[*current_animation],
                    Duration::from_millis(250),
                )
                .set_repeat(RepeatAnimation::Never);
        }

        if active_animation.is_finished() {
            *current_animation = 0;

            transitions
                .play(&mut player, animations.animations[*current_animation], Duration::ZERO)
                .repeat();
        }

    }

}

pub fn forward_text_update(
    mut texts: Query<&mut Text, With<ForwardText>>,
    transform: Query<&mut Transform, (With<Player>, Without<Arm>)>,
    arm: Query<&mut Transform, With<Arm>>,
    camera: Query<&mut Projection, With<ViewModelCamera>>
) {
    let player_transform = transform.single();
    for mut text in &mut texts {
        let forward_vector = player_transform.forward().as_vec3();
        text.sections[1].value = format!("x: {:.2}, y: {:.2}, z: {:.2}", forward_vector.x, forward_vector.y, forward_vector.z);
        
        let arm_vector = arm.single().translation;
        text.sections[3].value = format!("x: {:.2}, y: {:.2}, z: {:.2}", arm_vector.x, arm_vector.y, arm_vector.z);

        let projection = camera.single();
        let Projection::Perspective(ref perspective) = projection else {
            unreachable!(
                "The `Projection` component was explicitly built with `Projection::Perspective`"
            );
        };    
        
        text.sections[5].value = format!("{:.0}", perspective.fov.to_degrees());
        
    }
}

pub fn run_animations(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players_query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players_query {
        let mut transitions = AnimationTransitions::new();

        // Make sure to start the animation via the `AnimationTransitions`
        // component. The `AnimationTransitions` component wants to manage all
        // the animations and will get confused if the animations are started
        // directly via the `AnimationPlayer`.
        
        transitions
                .play(&mut player, animations.animations[0], Duration::ZERO)
                .repeat();

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions);


    }
}
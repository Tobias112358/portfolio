use std::marker;
use std::time::Duration;

use bevy::color::palettes::tailwind::{YELLOW_400, YELLOW_900};
use bevy::ecs::observer::TriggerTargets;
use bevy::render::camera::RenderTarget;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::{animation, prelude::*};
use bevy::input::mouse::MouseMotion;
use bevy::render::view::RenderLayers;
use bevy::color::palettes::{tailwind, css::*};
use bevy::pbr::{NotShadowCaster, VolumetricFogSettings};
use bevy::animation::{AnimationTargetId, RepeatAnimation};
use bevy::scene::SceneInstanceReady;
use bevy_rapier3d::na::{Vector, Vector3};
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::{Ccd, CharacterAutostep, CharacterLength, CoefficientCombineRule, Collider, ColliderMassProperties, CollisionEvent, ContactForceEvent, ExternalForce, ExternalImpulse, Friction, GravityScale, KinematicCharacterController, LockedAxes, QueryFilter, Restitution, RigidBody, Sensor, SoftCcd};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use bevy_tnua::prelude::*;
use bevy_tnua_rapier3d::*;

use crate::game::{
    scene::RoomScene,
    GameState,
};

use super::{
    enemy::{Enemy},
    combat_manager::{ Health, IsHit, AttackRange},
};
use super::AnimationEntityLink;

#[derive(Component)]
pub struct Speed(u8);

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct ColliderContactForce(Vec3);

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    rigidbody: RigidBody,
    speed: Speed,
    contact_force: ColliderContactForce,
    animations: Animations,
    health: Health,
}


//Camera markers

#[derive(Component)]
pub struct WorldModelCamera;

#[derive(Component)]
pub struct ViewModelCamera;

#[derive(Component)]
pub struct TextureCamera;

//Text Marker
#[derive(Component)]
pub struct ForwardText;

#[derive(Component)]
pub struct Arm;


//Resouirces
#[derive(Resource, Component)]
pub struct Animations {
    pub(crate) animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    pub(crate) graph: Handle<AnimationGraph>,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerState {
    #[default]
    Idle,
    Attacking,
}

const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins((
            TnuaControllerPlugin::default(),
            TnuaRapier3dPlugin::default(),
        ))
        .init_state::<PlayerState>()
        .add_systems(Startup, spawn_player)
        .add_event::<CollisionEvent>()
        .add_event::<ContactForceEvent>()
        .add_systems(Update, (move_player.in_set(TnuaUserControlsSystemSet), /*forward_text_update,*/ run_animations, debug_move_arm, player_actions, display_contact_info, change_rigidbody_params))
        .add_systems(OnEnter(PlayerState::Attacking), attack);
}

pub fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
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


    commands.spawn((
        PlayerBundle {
            marker: Player,
            rigidbody: RigidBody::Dynamic,
            speed: Speed(6),
            contact_force: ColliderContactForce(Vec3::ZERO),
            animations: Animations {
                animations,
                graph: graph.clone(),
            },
            health: Health {
                current_health: 100,
                max_health: 100,
            },
        },
        TnuaController::default()
    ))
    //.insert(GravityScale(0.125))
    .insert(Collider::cylinder(2.0, 0.5))
    //.insert(Restitution::coefficient(0.4))
    //.insert(ColliderMassProperties::Density(2.0))
    //.insert(Ccd::enabled())
    .insert(SoftCcd {
        prediction: 2.0
    })
    //.insert(Friction {
    //    coefficient: 0.7,
    //    combine_rule: CoefficientCombineRule::Multiply,
    //})
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)))
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

        let size = Extent3d {
            width: 512,
            height: 512,
            ..default()
        };

        let mut image = Image::new_fill(
            size,
            TextureDimension::D2,
            &[0, 0, 0, 0],
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::default(),
        );
        // You need to set these texture usage flags in order to use the image as a render target
        image.texture_descriptor.usage =
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    
    
        
        let image_handle = images.add(image);

        let texture_camera = parent.spawn((
                TextureCamera,
                Camera2d,
                Camera {
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
            ))
            .id();

            parent
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    
                    transform: Transform::from_translation(Vec3::ZERO),
                    visibility: Visibility::Visible,
                    background_color: BackgroundColor(Color::WHITE),
                    //z_index: ZIndex::Global(10),
                    
                    ..default()
                },
                TargetCamera(texture_camera),
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "This is a cube",  
                        TextStyle {
                            font_size: 40.0,
                            ..default()
                        }
                    )
                );
            });

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

        parent.spawn((
            SpotLightBundle {
                spot_light: SpotLight {
                    color: WHITE.into(),
                    intensity: 1_000_000.0,
                    range: 10_000.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            
        )
        );

        parent.spawn((
            AttackRange,
            Collider::ball(1.0),
            TransformBundle::from(Transform::from_xyz(0.0, 0.75, -5.0)),
            Sensor
        ));
    });
    /*
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
                "\nColForceVec: ",
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
    */
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
    mut camera: Query<&mut Transform, With<Player>>,
    contact_force_query: Query<&ColliderContactForce, (With<Player>, Without<Arm>)>,  
    mut controller_query: Query<&mut TnuaController, With<Player>>  
) {

    let Ok(mut controller) = controller_query.get_single_mut() else {
        println!("Failed to get: {:?}", controller_query);
        return;
    };


    let mut camera_transform = camera.single_mut();
    for motion in mouse_motion.read() {
        let yaw = -motion.delta.x * 0.003;
        let pitch = -motion.delta.y * 0.002;
        // Order of rotations is important, see <https://gamedev.stackexchange.com/a/136175/103059>
        camera_transform.rotate_y(yaw);
        camera_transform.rotate_local_x(pitch);
    }

    let mut movement_vector: Vec3 = camera_transform.translation;
    
    let mut direction = Vec3::ZERO;

    //camera move.
    if input.pressed(KeyCode::KeyW) {
        let mut forward_vec = camera_transform.forward().as_vec3().clone();

        forward_vec.y = 0.0;

        movement_vector += forward_vec * time.delta_seconds() * speed.single_mut().0 as f32;

        direction -= Vec3::Z;

    } else if input.pressed(KeyCode::KeyS) {
        let mut forward_vec = camera_transform.forward().as_vec3().clone();

        forward_vec.y = 0.0;
        movement_vector -= forward_vec * time.delta_seconds() * speed.single_mut().0 as f32;

        
        direction += Vec3::Z;
    }
    if input.pressed(KeyCode::KeyD) {
        let mut right_vec = camera_transform.right().as_vec3().clone();

        right_vec.y = 0.0;
        movement_vector += right_vec * time.delta_seconds() * speed.single_mut().0 as f32;

        
        direction -= Vec3::X;
    } else if input.pressed(KeyCode::KeyA) {
        let mut right_vec = camera_transform.right().as_vec3().clone();

        right_vec.y = 0.0;
        movement_vector -= right_vec * time.delta_seconds() * speed.single_mut().0 as f32;
        
        direction += Vec3::X;
    }
    if input.pressed(KeyCode::KeyE) {
        let up_vec = camera_transform.up().as_vec3();
        movement_vector += up_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    } else if input.pressed(KeyCode::KeyQ) {
        let up_vec = camera_transform.up().as_vec3();
        movement_vector -= up_vec * time.delta_seconds() * speed.single_mut().0 as f32;
    }

    //println!("{:?}", direction);

    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction * 10.0,
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 3.5,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });



    camera_transform.translation = movement_vector;// - contact_force_query.single().0;
}


fn player_actions(
    input: Res<ButtonInput<KeyCode>>, 
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations_query: Query<(&Animations, &AnimationEntityLink), With<Player>>,
    mut current_animation: Local<usize>,
    player_state: Res<State<PlayerState>>,
    mut player_next_state: ResMut<NextState<PlayerState>>,
) {
    
    if animation_players.is_empty() {
        return;
    }

    let (animations, animation_entity) = animations_query.single();

    if let Ok((mut player, mut transitions)) = animation_players.get_mut(animation_entity.0) {
        let Some((&_playing_animation_index, &active_animation)) = player.playing_animations().next() else {
            return;
        };
        if mouse_button.just_pressed(MouseButton::Left) && player_state.get() == &PlayerState::Idle {
            player_next_state.set(PlayerState::Attacking);
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
    camera: Query<&mut Projection, With<ViewModelCamera>>,
    contact_force_query: Query<&ColliderContactForce, (With<Player>, Without<Arm>)>,
) {
    let player_transform = transform.single();
    for mut text in &mut texts {
        let forward_vector = player_transform.forward().as_vec3();
        
        text.sections[1].value = format!("x: {:.2}, y: {:.2}, z: {:.2}", forward_vector.x, forward_vector.y, forward_vector.z);
        
        let arm_vector = contact_force_query.single().0;
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
    animations_query: Query<(&Animations, &AnimationEntityLink), Added<AnimationEntityLink>>,
    mut players_query: Query<(Entity, &mut AnimationPlayer), With<AnimationPlayer>>,
) {
    
    if players_query.is_empty() {
        return;
    }


    if animations_query.is_empty() {
        return;
    }

    let (animations, animation_entity) = animations_query.single();

    if let Ok((entity, mut player)) = players_query.get_mut(animation_entity.0) {        
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

/// Currently [`RenderLayers`] are not applied to children of a scene.
/// This [`SceneInstanceReady`] observer applies the [`RenderLayers`]
/// of a [`SceneRoot`] to all children with a [`Transform`] and without a [`RenderLayers`].
/// 
/// See [#12461](https://github.com/bevyengine/bevy/issues/12461) for current status.
pub fn apply_render_layers_to_children(
    mut commands: Commands,
    children: Query<&Children>,
    transforms: Query<&Transform, Without<RenderLayers>>,
    arm_query: Query<(Entity, &RenderLayers), With<Arm>>,
  ) {
    println!("Apply render layers to children");
    let Ok((parent, render_layers)) = arm_query.get_single() else {
        return;
    };
    children.iter_descendants(parent).for_each(|entity| {
      if transforms.contains(entity) {
        commands.entity(entity).insert(render_layers.clone());
      }
    });
  }

pub fn change_rigidbody_params(
    mut player_query: Query<(
        Entity, 
        &mut Friction, 
        &mut SoftCcd,
    ), With<Player>>,
    input: Res<ButtonInput<KeyCode>>, 
) {

    if player_query.is_empty() {
        //println!("{:?}",player_query);
        return;
    }

    let (player, mut friction, mut soft_ccd) = player_query.single_mut();

    if input.pressed(KeyCode::Numpad1) {
        friction.coefficient = friction.coefficient + 0.1;
        println!("friction.coefficient: {}", friction.coefficient);
    }
    if input.pressed(KeyCode::Numpad2) {
        friction.coefficient = friction.coefficient - 0.1;
        println!("friction.coefficient: {}", friction.coefficient);
    }
    if input.pressed(KeyCode::Numpad5) {
        soft_ccd.prediction = soft_ccd.prediction + 0.1;
        println!("soft_ccd.enabled: {}", soft_ccd.prediction);
    }
    if input.pressed(KeyCode::Numpad6) {
        soft_ccd.prediction = soft_ccd.prediction - 0.1;
        println!("soft_ccd.enabled: {}", soft_ccd.prediction);
    }
}

pub fn update_collider_params(
mut commands: Commands,
player_query: Query<Entity, With<Player>>,
) {
println!("update_collider_params: {:?}", player_query.single());

}

pub fn display_contact_info(rapier_context: Res<RapierContext>, 
    mut player_query: Query<(Entity, &mut ColliderContactForce), With<Player>>,
    room_query: Query<Entity, With<RoomScene>>,
    time: Res<Time>, 

) {
    if room_query.is_empty() || player_query.is_empty() {
        return;
    }


    let (player_entity, mut player_contact_force) = player_query.single_mut(); // A first entity with a collider attached.
    let room_entity = room_query.single(); // A second entity with a collider attached.

    player_contact_force.0 = Vec3::ZERO;
    /* Find the contact pair, if it exists, between two colliders. */
    if let Some(contact_pair) = rapier_context.contact_pair(player_entity, room_entity) {
        // The contact pair exists meaning that the broad-phase identified a potential contact.

        

        // We may also read the contact manifolds to access the contact geometry.
        for manifold in contact_pair.manifolds() {
            //println!("Local-space contact normal: {}", manifold.local_n1());
            //println!("Local-space contact normal: {}", manifold.local_n2());
            //println!("World-space contact normal: {}", manifold.normal());

            if player_contact_force.0 == Vec3::ZERO {
                player_contact_force.0 = manifold.local_n2();
            } else {
                player_contact_force.0 = (player_contact_force.0 + manifold.local_n2()).normalize();
            }

            // Read the geometric contacts.
            for contact_point in manifold.points() {
                // Keep in mind that all the geometric contact data are expressed in the local-space of the colliders.
                //println!(
                //    "Found local contact point 1: {:?}",
                //    contact_point.local_p1()
                //);
                //println!("Found contact distance: {:?}", contact_point.dist()); // Negative if there is a penetration.
                //println!("Found contact impulse: {}", contact_point.raw.data.impulse);
                //println!(
                //    "Found friction impulse: {}",
                //    contact_point.raw.data.tangent_impulse
                //);
            }

            // Read the solver contacts.
            for solver_contact in &manifold.raw.data.solver_contacts {
                // Keep in mind that all the solver contact data are expressed in world-space.
                //println!("Found solver contact point: {:?}", solver_contact.point);
                // The solver contact distance is negative if there is a penetration.
                //println!("Found solver contact distance: {:?}", solver_contact.dist);
            }
        }
    }
}


fn attack (
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    player_query: Query<(&Children, Entity, &Transform), With<Player>>,
    player_state: Res<State<PlayerState>>,
    mut player_next_state: ResMut<NextState<PlayerState>>,
    attack_range_query: Query<(&GlobalTransform, &Transform, &Collider), With<AttackRange>>,
    mut enemy_query: Query<Entity, With<Enemy>>,
    mut enemy_query_mut: Query< (&mut Health, Entity, &mut IsHit), With<Enemy>>,
    parent_query: Query<&Parent>,
) {
    //println!("ATTACK!!!");

    let Ok((player_children, player_entity, player_transform)) = player_query.get_single() else {
        println!("Faile");
        return;
    };

    let predicate = |handle| {
        // We can use a query to bevy inside the predicate.

        //println!("handle: {:?}", handle);

        let Ok(parent) = parent_query.get(handle) else {
            return false;
        };

        //println!("PARENT of  {:?}: {:?}, {:?}", handle, parent, parent.get());

        enemy_query
            .get(parent.get())
            .is_ok()
    };

    
    let filter = QueryFilter::default()
    .predicate(&predicate);

    /*let Ok(entity) = player_query.get_single() else {
        return;
    };

    commands.entity(entity).insert(shape.clone());
    */

    for player_child in player_children {
        let Ok((ar_global_transform, ar_transform, ar_collider)) = attack_range_query.get(*player_child) else {
            continue;
        };

        rapier_context.intersections_with_shape(ar_global_transform.translation(), ar_transform.rotation, ar_collider, filter, |entity| {
            //println!("TEST {:?} intersects our shape.", entity);
    
            let Ok(parent) = parent_query.get(entity) else {
                return false;
            };
    
            let Ok((mut health, entity, mut is_hit)) = enemy_query_mut.get_mut(parent.get()) else {
                return false;
            };
    
            let mut attack_vector = player_transform.forward().as_vec3();
    
            attack_vector.y = 1.25;
    
            
            //commands.entity(entity).remove::<LockedAxes>();
    
            commands.entity(entity).insert(ExternalImpulse {
                impulse: attack_vector * 200.0,
                torque_impulse: attack_vector * 500.0,
            });
    
            if health.current_health >= 10 {
                health.current_health -= 10;
            } else {
                health.current_health = 0;
            }
    
            is_hit.is_hit = true;
    
    
            true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        });
    
        player_next_state.set(PlayerState::Idle);
    }


}
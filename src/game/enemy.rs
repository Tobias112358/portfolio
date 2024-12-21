use std::time::Duration;

use bevy::{animation::RepeatAnimation, log::tracing_subscriber::fmt::time, prelude::*, render::view::RenderLayers};
use bevy_rapier3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::game::player::{Player, Animations};

use super::AnimationEntityLink;

#[derive(Default)]
pub struct SpawnCount(f32);


#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct IsHit {
    pub is_hit: bool,
    pub hit_timeout: f32,
}


#[derive(Component)]
pub struct Health(pub u32);

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    scene: HookedSceneBundle,
    rigidbody: RigidBody,
    animations: Animations,
    health: Health,
    is_hit: IsHit
}

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(Update, (spawn_enemy, enemy_movement, player_actions, enemy_die, after_hit));
    
}

pub fn spawn_enemy(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut spawn_count: Local<SpawnCount>,
    mut enemy_transform_query: Query<&mut Transform, With<Enemy>>,
) {
    spawn_count.0 += time.delta_seconds();
    
    if spawn_count.0 > 5.0 && enemy_transform_query.iter().len() <= 25 {

        println!("{}",time.elapsed_seconds());
        spawn_count.0 = 0.0;
        
        let enemy_scene = asset_server.load("Enemy.glb#Scene0");

        

        //Animations
        let mut graph = AnimationGraph::new();
        let animations = graph.add_clips(
            [
                GltfAssetLabel::Animation(0).from_asset("Enemy.glb"),
                GltfAssetLabel::Animation(1).from_asset("Enemy.glb"),
                GltfAssetLabel::Animation(2).from_asset("Enemy.glb"),
            ]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

        let graph = graphs.add(graph);

        commands.spawn(EnemyBundle {
            marker: Enemy,
            scene: HookedSceneBundle {
                scene: SceneBundle {
                    scene: enemy_scene,
                    transform: Transform::from_translation(Vec3::new(0.0, 10.0, -30.0)).with_rotation(Quat::from_rotation_y(-std::f32::consts::PI)),
                    ..default()
                },
                hook: SceneHook::new(|_, commands| {
                    commands.insert(RenderLayers::layer(0));
                }),
            },
            rigidbody: RigidBody::Dynamic,
            animations: Animations {
                animations,
                graph: graph.clone(),
            },
            health: Health(100),
            is_hit: IsHit { is_hit: false, hit_timeout: 0.0 },
        })
        //.insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(1.5, 1.25, 1.5))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.25, 0.0)));
        });
    }
}

pub fn enemy_movement(
    time: Res<Time>,
    mut enemy_transform_query: Query<(&mut Transform, &IsHit), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let mut player_translation = player_transform.translation.clone();

    player_translation.y = 1.0;

    //println!("{:?}", player_translation);

    for (mut transform, is_hit) in &mut enemy_transform_query {
        //let mut temp_transform = transform.clone();

        if !is_hit.is_hit {
            transform.look_at(player_translation, Vec3::Y);
        }
        

        let transform_forward = transform.clone().forward();

        transform.translation += transform_forward * 2.0 * time.delta_seconds();
    }
}

pub fn after_hit(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_transform_query: Query<(Entity, &mut Transform, &mut IsHit), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,

) {
    for (entity, mut transform,  mut is_hit) in enemy_transform_query.iter_mut() {
        if is_hit.is_hit {
            is_hit.hit_timeout += time.delta_seconds();
        }

        if is_hit.hit_timeout > 10.0 {
            
            transform.rotation = Quat::IDENTITY;
            //commands.entity(entity).insert(LockedAxes::ROTATION_LOCKED);
            is_hit.is_hit = false;
        }
    }
}

fn player_actions(
    input: Res<ButtonInput<KeyCode>>, 
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations_query: Query<(&Animations, &AnimationEntityLink), With<Enemy>>,
    mut current_animation: Local<usize>,
) {
    
    //println!("Anim Players:    {:?}", animation_players);
    if animation_players.is_empty() {
        return;
    }

    if animations_query.is_empty() {
        //println!("{:?}", animations_query);
        return;
    }


    for (animations, animation_entity) in animations_query.iter() {

        if let Ok((mut player, mut transitions)) = animation_players.get_mut(animation_entity.0) {
            
            let Some((&playing_animation_index, &active_animation)) = player.playing_animations().next() else {
                continue;
            };
            //If enemy state = attack, change animation.

            if active_animation.completions() > 0 {
                //println!("Anim Entity! {:?}", animation_entity.0);
    
                transitions
                    .play(&mut player, animations.animations[1], Duration::ZERO)
                    .repeat();
                *current_animation = (*current_animation + 1) % animations.animations.len();
            }
                    }
            
        /*for ( mut player, mut transitions) in &mut animation_players {

            
        
            let Some((&playing_animation_index, &active_animation)) = player.playing_animations().next() else {
                continue;
            };
            //If enemy state = attack, change animation.
    
            if active_animation.is_finished() {
    
                transitions
                    .play(&mut player, animations.animations[*current_animation], Duration::ZERO)
                    .repeat();
                *current_animation = (*current_animation + 1) % animations.animations.len();
            }
    
        }
         */
        
    }

}


/*fn run_animations(
    mut commands: Commands,
    animations_query: Query<(&Animations, &AnimationEntityLink), With<Enemy>>,
    mut players_query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    if players_query.is_empty() {
        return;
    }

    if animations_query.is_empty() {
        println!("{:?}", animations_query);
        return;
    }

    for (animations, animation_entity) in animations_query.iter() {
        if let Ok((entity, mut player)) = players_query.get_mut(animation_entity.0) {
            let mut transitions = AnimationTransitions::new();
    
            // Make sure to start the animation via the `AnimationTransitions`
            // component. The `AnimationTransitions` component wants to manage all
            // the animations and will get confused if the animations are started
            // directly via the `AnimationPlayer`.
            
            transitions
                    .play(&mut player, animations.animations[0], Duration::ZERO)
                    .set_repeat(RepeatAnimation::Never);
    
            commands
                .entity(entity)
                .insert(animations.graph.clone())
                .insert(transitions);
    
    
        }
    }
    
}
    */
    

fn enemy_die (
    mut commands: Commands,
    enemy_query: Query<(&Health, Entity, &Transform), With<Enemy>>,
) {
    for (health, entity, transform) in enemy_query.iter() {
        if health.0 <= 0 || transform.translation.y < -10.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
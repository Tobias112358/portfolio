use std::time::Duration;

use bevy::{color::palettes::tailwind::*, prelude::*, render::{camera::RenderTarget, render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}};
use bevy_rapier3d::{prelude::{ActiveEvents, ExternalImpulse}, pipeline::CollisionEvent};

use rand::prelude::*;

use crate::game::{
    player::{Player, TextureCamera},
    enemy::{Enemy, EnemyHealthBar, AttackMode, Target},
};

#[derive(Component)]
pub struct IsHit {
    pub is_hit: bool,
    pub hit_timeout: f32,
}


#[derive(Component)]
pub struct Health{
    pub current_health: u16,
    pub max_health: u16,
}

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Event)]
//Contains Entity being attacked (Player) and entity attacking.
struct EnemyAttackEvent(Entity, Entity);

#[derive(Component)]
pub struct AttackRange;

pub(super) fn plugin(app: &mut App) {
    app
        .add_event::<EnemyAttackEvent>()
        .add_systems(Startup, setup_player_health)
        .add_systems(Update, (after_hit, display_player_health, display_enemy_health, enemy_attack_event, display_events, enemy_attack_decider));
}

pub fn setup_player_health(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {

    commands.spawn((
        PlayerHealthBar,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                ..default()
            },
            
            transform: Transform::from_translation(Vec3::ZERO),
            visibility: Visibility::Visible,
            background_color: BackgroundColor(Color::WHITE),
            //z_index: ZIndex::Global(10),
            ..default()
        },
    
    ))
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(5.0),
                top: Val::Px(5.0),
                width: Val::Px(0.0),
                height: Val::Px(40.0),
                ..default()
            },
            
            transform: Transform::from_translation(Vec3::ZERO),
            visibility: Visibility::Visible,
            background_color: BackgroundColor(
                Color::Srgba(
                    Srgba { 
                        red: 0.0, 
                        green: 1.0, 
                        blue: 0.0, 
                        alpha: 1.0
                    }
                )
            ),
            //z_index: ZIndex::Global(10),
            ..default()
        });
    });
}

pub fn display_player_health(
    commands: Commands,
    player_query: Query<(Entity, &Health), With<Player>>,
    parent_healthbar_query: Query<&Children, With<PlayerHealthBar>>,
    mut healthbar_style_query: Query<&mut Style>,
) {
    let Ok((player, health)) = player_query.get_single() else {
        return;
    };

    for children in parent_healthbar_query.iter() {
        let Some(child) = children.get(0) else {
            return;
        };
        let Ok(mut healthbar_style) = healthbar_style_query.get_mut(*child) else {
            return;
        };

        healthbar_style.width = Val::Px((health.current_health as f32 / health.max_health as f32) * 190.0);
    }


    
}

pub fn display_enemy_health(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    enemy_query: Query<&Children, With<Enemy>>,
    mut enemy_healthbar_query: Query<&mut Handle<StandardMaterial>, With<EnemyHealthBar>>,
    texture_camera_query: Query<Entity, With<TextureCamera>>,
) {
    for enemy_children in enemy_query.iter() {

        for &child in enemy_children.iter() {
            let Ok(mut material) = enemy_healthbar_query.get_mut(child) else {
                continue;
            };

            //println!("Enemy healthbar found!");


    
            let Ok(texture_camera) = texture_camera_query.get_single() else {
                return;
            };
        }
    };

        
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
            commands.entity(entity).remove::<ExternalImpulse>();
            is_hit.is_hit = false;
            is_hit.hit_timeout = 0.0;
        }
    }
}


fn enemy_attack_event(
    mut commands: Commands,
    mut collision_events: EventReader<EnemyAttackEvent>,
    mut player_health_query: Query<&mut Health, With<Player>>,
) {
    for mut active_event in collision_events.read() {
        match active_event {
            EnemyAttackEvent(player_entity, enemy) => {
                println!("Player attacked by enemy: {:?}", enemy);
                let Ok(mut player_health) = player_health_query.get_mut(*player_entity) else {
                    return;
                };
                if player_health.current_health < 10 {
                    player_health.current_health = 0;
                    println!("Player is DEAD!");
                    commands.entity(*player_entity).despawn_recursive();
                } else {
                    player_health.current_health -= 10;
                }
            }
        }
        
    }
}

fn enemy_attack_initiator(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    mut attack_mode_query: Query<&mut AttackMode, With<Enemy>>,
) {
    for mut enemy_attack_mode in attack_mode_query.iter_mut() {
        if *enemy_attack_mode != AttackMode::InRange {
            continue;
        } else {
            let random_decision = rand::thread_rng().gen_range(0..10);
            if random_decision == 5 {
                *enemy_attack_mode = AttackMode::Attacking;
            }
        }
    }

}

fn enemy_attack_decider(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    mut attack_mode_query: Query<(&mut AttackMode, &Target), With<Enemy>>,
    mut player_health_query: Query<&mut Health, With<Player>>,
) {
    for (mut enemy_attack_mode, Target(player_entity_option)) in attack_mode_query.iter_mut() {
        
        match *enemy_attack_mode {
            AttackMode::Attacking => {
                println!("Attacking mode");
                *enemy_attack_mode = AttackMode::Hit;
            },
            AttackMode::InRange => {
                let random_decision = rand::thread_rng().gen_range(0..50);
                println!("Trying number: {}", random_decision);
                if random_decision == 5 {
                    *enemy_attack_mode = AttackMode::Attacking;
                }
            },
            AttackMode::Hit => {
                //println!("Player attacked by enemy: {:?}", enemy);

                let Some(player_entity) = player_entity_option else {
                    *enemy_attack_mode = AttackMode::NotInRange;
                    return;
                };
                println!("Hit mode: {:?}", player_entity);

                let Ok(mut player_health) = player_health_query.get_mut(*player_entity) else {
                    return;
                };
                println!("Hit mode 2");

                let attack_vector = Vec3::new(0.0, 0.5, 1.0);

                commands.entity(*player_entity).insert(ExternalImpulse {
                    impulse: attack_vector * 20.0,
                    torque_impulse: attack_vector * 50.0,
                });

                if player_health.current_health < 10 {
                    player_health.current_health = 0;
                    println!("Player is DEAD!");
                    commands.entity(*player_entity).despawn_recursive();
                } else {
                    player_health.current_health -= 10;
                }
                *enemy_attack_mode = AttackMode::NotInRange;
            },
            _ => {
                *enemy_attack_mode = AttackMode::NotInRange;
                continue;
            }
        }
    }

}


fn display_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    mut event_writer: EventWriter<EnemyAttackEvent>,
    mut attack_mode_query: Query<&mut AttackMode, With<Enemy>>,
    mut target_query: Query<&mut Target, With<Enemy>>,
    parent_query: Query<&Parent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(ent1, ent2, flags) => {
                
                
                let Ok(player) = player_query.get(*ent1) else {
                    return;
                };
                println!("entered collider: {:?}, {:?}", ent1, ent2);

                //get ent2 parent.
                let Ok(enemy) = parent_query.get(*ent2) else {
                    return;
                };

                let Ok(mut target) = target_query.get_mut(enemy.get()) else {
                    return;
                };
                println!("Got Target");
                let Ok(mut attack_mode) = attack_mode_query.get_mut(enemy.get()) else {
                    return;
                };
                println!("Get Attack Mode");
                *attack_mode = AttackMode::InRange;
                target.0 = Some(player);

                //For when I was using an event.
                //event_writer.send(EnemyAttackEvent(player, *ent2));
            },
            CollisionEvent::Stopped(ent1, ent2, flags) => {
                let Ok(player) = player_query.get(*ent1) else {
                    return;
                };
                
                let Ok(enemy) = parent_query.get(*ent2) else {
                    return;
                };
                let Ok(mut attack_mode) = attack_mode_query.get_mut(enemy.get()) else {
                    return;
                };
                *attack_mode = AttackMode::NotInRange;
                let Ok(mut target) = target_query.get_mut(enemy.get()) else {
                    return;
                };
                target.0 = None;
                
                //println!("Left collider: {:?}, {:?}", ent1, ent2);
            }
        }
    }
}


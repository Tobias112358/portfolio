use bevy::asset::{AssetContainer, LoadState};
use bevy::color::palettes::tailwind::LIME_400;
use bevy::gltf::{GltfMesh, GltfNode};
use bevy::log::tracing_subscriber::fmt::format::Full;
use bevy::prelude::*;
use bevy::color::palettes::basic::{LIME, WHITE};
use bevy::pbr::{CascadeShadowConfigBuilder, VolumetricLight};
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, ContactSkin};
use bevy_rapier3d::rapier::crossbeam::epoch::Pointable;
use bevy_rapier3d::rapier::prelude::ColliderBuilder;
use std::f32::consts::PI;
use bevy::utils::HashMap;

use crate::game::{
    file_system_interaction::{RoomAssets},
    GameState
};

#[derive(Component)]
pub struct Cube;


#[derive(Component)]
pub struct RoomScene;

pub(super) fn plugin(app: &mut App) {
    app
        //.add_systems(PreStartup, load_room)
        .add_systems(OnEnter(GameState::Playing), setup_scene)
        .add_systems(Update, change_collider_params.run_if(in_state(GameState::Playing)));
        //.add_systems(Update, update_scene);
}


pub fn setup_scene(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut nodes: ResMut<Assets<GltfNode>>, 
    models: Res<Assets<Gltf>>,
    room_assets: Res<RoomAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    
    let room = models.get(&room_assets.room_gltf);
    let room_mesh_handles = &room_assets.room_meshes;
    let room_node_handles = &room_assets.room_nodes;

    let mut room_meshes: Vec<Mesh> = vec!();
    for (key, mesh_handle) in room_mesh_handles.iter() {
        if let Some(mesh) = meshes.get(mesh_handle) {
            println!("{:?} is {:?}", key, mesh);
            room_meshes.push(mesh.clone());
        }
    }
    let mut room_mesh_transform = Transform {
        ..Default::default()
    };

    for (key, node_handle) in room_node_handles.iter() {
        if let Some(node) = nodes.get(node_handle) {
            println!("{:?} is node {}", key, node.name);
            if node.name == "Room" {
                room_mesh_transform = node.transform;
            }
        }
    }

    match room {
        Some(room) => {

            // if let (
            //     Some(meshes), 
            //     Some(nodes)
            // ) = (
            //     meshes.get(), 
            //     meshes.get()
            // ) {
            
                // let rotated_mesh_part_one = mesh_part_one.clone().rotated_by(Quat::from_rotation_z(PI));
                // let collider_mesh_part_one = Collider::from_bevy_mesh(&rotated_mesh_part_one, &ComputedColliderShape::TriMesh).unwrap();
                
                // let rotated_mesh_part_two = mesh_part_two.clone().rotated_by(Quat::from_rotation_z(PI));
                // let collider_mesh_part_two = Collider::from_bevy_mesh(&rotated_mesh_part_two, &ComputedColliderShape::TriMesh).unwrap();

                /*let room_meshes_slice: &[Mesh] = room_meshes.as_slice();

                let full_mesh = combine_meshes(
                    room_meshes_slice, 
                    &[
                        room_mesh_transform,
                        room_mesh_transform
                    ], 
                    false, 
                    false, 
                    false, 
                    false);
                    */

                    let full_mesh_option: Option<Mesh> = room_meshes.iter().fold(None, 
                        |acc:Option<Mesh>, mesh| 
                        {
                            match acc {
                                Some(accumulated_mesh) => {
                                    let mut temp = accumulated_mesh.clone();
                                    temp.merge(&mesh.clone().transformed_by(room_mesh_transform));
                                    
                                    Some(temp)
                                },
                                None => {
                                    Some(mesh.clone().transformed_by(room_mesh_transform))
                                }
                            }
                        }
                    );

                    //Attach outdoor mesh

                    let outdoor_mesh = meshes.get(&room_assets.outdoor_mesh).unwrap();

                    let mut full_mesh = full_mesh_option.unwrap();
                    
                    full_mesh.merge(outdoor_mesh);

                /*let room_colliders: Vec<Collider> = room_meshes.iter().map(|&mesh| 
                    Collider::from_bevy_mesh(
                        &mesh.clone().transformed_by(room_mesh_transform), 
                        &ComputedColliderShape::TriMesh
                    ).unwrap()
                )
                .collect();
            */

            // let mut full_mesh_2 = room_meshes_slice[0].clone();

            
            // full_mesh_2.merge(&room_meshes_slice[1]);
                    let room_collider = Collider::from_bevy_mesh(
                        &full_mesh,
                        &ComputedColliderShape::TriMesh
                    ).unwrap();

                commands.spawn((RoomScene,
                    SceneBundle {
                    scene: room.scenes[0].clone(),
                    transform: Transform::from_xyz(2.0, 0.0, -5.0),
                    ..Default::default()
                }))
                .insert(room_collider)
                .insert(ContactSkin(0.2));
                
        },
        None => {
            println!("No Room :'(")
        }
    }
        //Spawn cube
        commands
        .spawn((Cube,
            PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::from(LIME)),
            transform: Transform::from_xyz(0.0, 0.5, -5.0).with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
            ..default()
        }));

        // blue point light
        commands
            .spawn((PointLightBundle {
                // transform: Transform::from_xyz(5.0, 8.0, 2.0),
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                point_light: PointLight {
                    intensity: 10_000.0,
                    shadows_enabled: true,
                    color: LIME_400.into(),
                    radius: 100.0,
                    ..default()
                },
                ..default()
            },
            RenderLayers::from_layers(&[0,1]),

        ));

        commands
            .spawn((
                DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        illuminance: 5.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                RenderLayers::from_layers(&[0,1]),
            ));

}

pub fn update_scene(mut cube: Query<&mut Transform, With<Cube>>) {
    if !cube.is_empty() {
        let mut cube_transform = cube.single_mut();
        cube_transform.rotate_x(0.001 * std::f32::consts::PI);
    }
}


pub fn change_collider_params(
    mut player_query: Query<(
        Entity, 
        &mut ContactSkin, 
    ), With<RoomScene>>,
    input: Res<ButtonInput<KeyCode>>, 
) {
    if player_query.is_empty() {
        return;
    }
    let (room, mut contact_skin) = player_query.single_mut();

    if input.pressed(KeyCode::Numpad7) {
        contact_skin.0 = contact_skin.0 + 0.1;
        println!("friction.coefficient: {}", contact_skin.0);
    }
    if input.pressed(KeyCode::Numpad8) {
        contact_skin.0 = contact_skin.0 - 0.1;
        println!("friction.coefficient: {}", contact_skin.0);
    }
}

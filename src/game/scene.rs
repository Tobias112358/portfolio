use bevy::prelude::*;
use bevy::color::palettes::basic::{LIME, WHITE};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::render::view::RenderLayers;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Cube;

pub fn setup_scene(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>) {
    
    let room = asset_server.load("Room1.glb#Scene0");

    /*//Spawn ground
    commands
        .spawn(MaterialMeshBundle  {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
            material: materials.add(Color::from(WHITE)),
            ..default()
        });*/

        commands.spawn(SceneBundle {
            scene: room,
            transform: Transform::from_xyz(2.0, 0.0, -5.0),
            ..Default::default()
        });
    


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
                intensity: 1_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
        RenderLayers::from_layers(&[0,1]),

    ));


}

pub fn update_scene(mut cube: Query<&mut Transform, With<Cube>>) {
    let mut cube_transform = cube.single_mut();
    cube_transform.rotate_x(0.001 * std::f32::consts::PI);
}
use bevy::prelude::*;
use bevy::color::palettes::basic::{LIME, WHITE};

#[derive(Component)]
pub struct Cube;

pub fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    
    //Spawn ground
    commands
        .spawn(MaterialMeshBundle  {
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0))),
            material: materials.add(Color::from(WHITE)),
            ..default()
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
}

pub fn update_scene(mut cube: Query<&mut Transform, With<Cube>>) {
    let mut cube_transform = cube.single_mut();
    cube_transform.rotate_x(0.001 * std::f32::consts::PI);
}
use bevy::prelude::*;
use bevy::color::palettes::basic::LIME;

pub fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::from(LIME)),
            transform: Transform::from_xyz(0.0, 0.5, -5.0).with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
            ..default()
        });
}
use bevy::prelude::*;

#[derive(Component)]
struct MainGameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn((
            Camera3dBundle::default(), 
            MainGameCamera
        ));
}
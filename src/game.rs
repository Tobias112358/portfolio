use wasm_bindgen::prelude::*;
use bevy::prelude::*;

mod camera;
mod scene;
mod player;

pub use camera::{setup_camera, update_camera};
pub use scene::{setup_scene, update_scene};
pub use player::{spawn_player, move_player};

//Initialize 3D bevy game
#[wasm_bindgen]
pub fn build_game(element_id: &str) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // provide the ID selector string here
                canvas: Some(element_id.into()),
                // ... any other window properties ...
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (spawn_player, setup_scene))
        .add_systems(Update, (update_scene, move_player))
        .run();
}
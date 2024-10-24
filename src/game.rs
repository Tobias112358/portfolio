use wasm_bindgen::prelude::*;
use bevy::prelude::*;

mod camera;
mod scene;

pub use camera::setup_camera;
pub use scene::setup_scene;

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
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_scene)
        .run();
}
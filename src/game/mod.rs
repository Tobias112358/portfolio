use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod camera;
mod scene;
mod player;

use bevy_scene_hook::HookPlugin;
pub use camera::{setup_camera, update_camera};
pub use scene::{setup_scene, update_scene};
pub use player::{spawn_player, move_player, forward_text_update, run_animations, debug_move_arm, player_actions};

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
        .add_plugins(HookPlugin)
        .add_systems(Startup, (spawn_player, setup_scene))
        .add_systems(Update, (move_player, update_scene, forward_text_update, run_animations, debug_move_arm, player_actions))
        .run();
}
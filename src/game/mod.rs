use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use wasm_bindgen::prelude::*;

mod camera;
pub(crate) mod player;
pub(crate) mod scene;
pub(crate) mod file_system_interaction;
pub(crate) mod enemy;
pub(crate) mod combat_manager;

use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_scene_hook::HookPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);


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
        .insert_resource(AmbientLight::NONE)
        .init_state::<GameState>()
        .add_plugins((HookPlugin, RapierDebugRenderPlugin::default(), RapierPhysicsPlugin::<NoUserData>::default()))
        .add_plugins((scene::plugin, player::plugin, file_system_interaction::plugin, enemy::plugin, combat_manager::plugin))
        //.add_systems(PreStartup, load_room)
        .add_systems(Update, link_animations)
        .run();
}

//Pinkponk's cool code: https://github.com/bevyengine/bevy/discussions/5564#discussioncomment-3333257
fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

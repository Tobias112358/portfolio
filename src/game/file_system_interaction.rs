use bevy::{gltf::GltfNode, prelude::*};
use bevy_asset_loader::prelude::*;
use crate::game::GameState;
use iyes_progress::ProgressPlugin;
use bevy::utils::HashMap;

pub(super) fn plugin(app: &mut App) {
    app
        .add_plugins(ProgressPlugin::<GameState>::new(GameState::Loading).continue_to(GameState::Playing))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<RoomAssets>()
        );
}


#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct RoomAssets {

    #[asset(path = "Room1.glb")]
    pub(crate) room_gltf: Handle<Gltf>,

    #[asset(
        paths(
            "Room1.glb#Node0",
            "Room1.glb#Node1",
            "Room1.glb#Node2",
            "Room1.glb#Node3",
            "Room1.glb#Node4",
            "Room1.glb#Node5",
        ),
        collection(mapped, typed)
    )]
    pub(crate) room_nodes: HashMap<AssetLabel, Handle<GltfNode>>,

    #[asset(
        paths(
            "Room1.glb#Mesh3/Primitive0",
            "Room1.glb#Mesh3/Primitive1"
        ),
        collection(mapped, typed)
    )]
    pub(crate) room_meshes: HashMap<AssetLabel, Handle<Mesh>>,
    #[asset(path = "Room1.glb#Mesh5/Primitive0")]
    pub(crate) outdoor_mesh: Handle<Mesh>
}
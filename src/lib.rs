use wasm_bindgen::prelude::*;
use bevy::prelude::*;


mod karplus_strong;
mod random;

pub use karplus_strong::KarplusStrong;


#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to WASM!", name)
}

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


//Initialize bevy game
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
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greet_test() {
        let result = greet("Tobias");
        assert_eq!(result, "Hello, Tobias! Welcome to WASM!");
    }
}
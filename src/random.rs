use wasm_bindgen::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[wasm_bindgen]
pub struct Random {
    rng: StdRng,
}

#[wasm_bindgen]
impl Random {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let seed = generate_seed();
        Self {
            rng: StdRng::from_seed(seed),
        }
    }

    pub fn gen_range(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min..max)
    }
}

fn generate_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    getrandom::getrandom(&mut seed).expect("Failed to generate random seed");
    seed
}
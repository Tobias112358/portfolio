use crate::random::Random;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KarplusStrong {
    buffer: Vec<f32>,
    index: usize,
    decay: f32,
    random: Random,
}

#[wasm_bindgen]
impl KarplusStrong {
    #[wasm_bindgen(constructor)]
    pub fn new(frequency: f32, sample_rate: f32) -> Self {
        let buffer_size = (sample_rate / frequency).round() as usize;
        let mut buffer = vec![0.0; buffer_size];
        let mut random = Random::new();

        // Initialize buffer with white noise
        for sample in buffer.iter_mut() {
            *sample = random.gen_range(-1.0, 1.0) as f32;
        }

        KarplusStrong {
            buffer,
            index: 0,
            decay: 0.99, // Adjust this value to change the decay rate
            random,
        }
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> f32 {
        let current_sample = self.buffer[self.index];
        let next_index = (self.index + 1) % self.buffer.len();
        let next_sample = self.buffer[next_index];

        // Apply low-pass filter and decay
        let new_sample = self.decay * 0.5 * (current_sample + next_sample);
        self.buffer[self.index] = new_sample;

        self.index = next_index;
        current_sample
    }

    #[wasm_bindgen]
    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay;
    }

}

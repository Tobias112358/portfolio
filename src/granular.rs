use wasm_bindgen::prelude::*;


mod circular_buffer;

pub use circular_buffer::CircularBuffer;

#[wasm_bindgen]
extern "C" {
    fn console_log(s: &str);
}

#[wasm_bindgen]
pub struct GranularSynth {
    sample: Vec<f32>, // The audio sample data
    sample_rate: f32,
    grains: Vec<Grain>,
    is_playing: bool,
    circular_buffer: CircularBuffer,
}

#[wasm_bindgen]
impl GranularSynth {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: f32, buffer_size: usize) -> Self {
        console_error_panic_hook::set_once();
        GranularSynth {
            sample: Vec::new(),
            sample_rate,
            grains: Vec::new(),
            is_playing: false,
            circular_buffer: CircularBuffer::new(buffer_size),
        }
    }

    #[wasm_bindgen]
    pub fn load_sample(&mut self, sample_data: &[f32]) {
        self.sample = sample_data.to_vec(); // Load the sample data
        console_log(&format!("Sample Loaded. Sample length is {}", self.sample.len()));
    }

    #[wasm_bindgen]
    pub fn add_grain(&mut self, position: f32, duration: f32) {
        let grain = Grain::new(position, duration, self.sample.clone(), self.sample_rate);
        
        self.grains.push(grain);
    }

    #[wasm_bindgen]
    pub fn generate(&mut self) -> Vec<f32> {
        let mut output = Vec::new();
        for grain in &self.grains {
            let grain_sample = grain.sample();
            if grain_sample.is_empty() {
                console_log("failed to generate this grain sample.");
            } else {
                output.extend(grain_sample);
            }
        }

        // Write generated samples to the circular buffer
        self.circular_buffer.write(&output);
        output
    }

    #[wasm_bindgen]
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn update(&mut self) {
        if self.is_playing {
            // Logic to update the current position or generate samples
            //self.current_position += 1; // Increment position or handle looping
            // You can also call self.generate() here if needed
        }
    }

    // Method to read from the circular buffer
    pub fn read_buffer(&self) -> Vec<f32> {
        self.circular_buffer.read().to_vec()
    }
}

struct Grain {
    position: f32,
    duration: f32,
    sample: Vec<f32>, // Store owned data
    sample_rate: f32,
}

impl Grain {
    fn new(position: f32, duration: f32, sample: Vec<f32>, sample_rate: f32) -> Self {
        Grain { position, duration, sample, sample_rate }
    }

    fn sample(&self) -> Vec<f32> {
        let sample_count = (self.duration * self.sample_rate) as usize; // Number of samples to generate
        let start_index = (self.position * self.sample_rate) as usize; // Starting index in the sample

        // Generate samples for the grain
        let mut output = Vec::new();
        for i in 0..sample_count {
            let index = start_index + i;
            if index < self.sample.len() {
                output.push(self.sample[index]);
            } else {
                break; // Stop if we exceed the sample length
            }
        }
        output
    }
}

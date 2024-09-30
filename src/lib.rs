use wasm_bindgen::prelude::*;

mod karplus_strong;
mod random;

pub use karplus_strong::KarplusStrong;


#[wasm_bindgen]
pub fn generate_karplus_strong() {
    let sample_rate = 44100.0;
    let frequency = 440.0; // A4 note
    let duration = 2.0; // seconds

    let mut synth = karplus_strong::KarplusStrong::new(frequency, sample_rate);

    for _ in 0..(sample_rate * duration) as usize {
        let sample = synth.tick();
        // Here you would typically send the sample to your audio output
        println!("{}", sample);
    }
}


#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to WASM!", name)
}

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
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

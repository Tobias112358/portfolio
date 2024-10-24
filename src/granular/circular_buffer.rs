pub struct CircularBuffer {
    buffer: Vec<f32>,
    size: usize,
    write_index: usize,
}

impl CircularBuffer {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: vec![0.0; size],
            size,
            write_index: 0,
        }
    }

    // Method to write samples to the buffer
    pub fn write(&mut self, samples: &[f32]) {
        for &sample in samples {
            self.buffer[self.write_index] = sample;
            self.write_index = (self.write_index + 1) % self.size; // Wrap around
        }
    }

    // Method to read samples from the buffer
    pub fn read(&self) -> &[f32] {
        &self.buffer
    }
}


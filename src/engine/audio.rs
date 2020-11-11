
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use cpal::traits::{StreamTrait, HostTrait, DeviceTrait};
use cpal::{StreamConfig, SampleRate, SampleFormat, BufferSize};

const SAMPLE_RATE: usize = 44_100;

/// Manages the audio.
#[derive(Default)]
pub struct Audio {
    mixer: Arc<Mutex<usfx::Mixer>>,
}

impl Audio {
    /// Instantiate a new audio object without a generator.
    pub fn new() -> Self {
        Self {
            mixer: Arc::new(Mutex::new(usfx::Mixer::new(SAMPLE_RATE))),
        }
    }

    /// Play samples.
    pub fn play(&mut self, sample: usfx::Sample) {
        // Add the sample to the mixer
        self.mixer.lock().unwrap().play(sample);
    }

    /// Start a thread which will emit the audio.
    pub fn run(&mut self) {
        let mixer = self.mixer.clone();

        // Setup the audio system
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = StreamConfig{
            channels: 1,
            sample_rate: SampleRate(SAMPLE_RATE as u32),
            buffer_size: BufferSize::Default
        };

        let stream = device.build_output_stream(&config,||{},||{})
            .expect("could not build output stream");

        stream.play()
            .expect("could not play stream");
    }
}
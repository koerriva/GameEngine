#[warn(unused_imports)]
use gl::types::*;
use crate::engine::window::Window;
use crate::engine::core::GameEngine;
use crate::engine::logic::IGameLogic;
use crate::game::dummy_game::DummyGame;
use crate::engine::resource::ResourceLoader;
use crate::engine::font::Font;
use std::fmt::Error;
use crate::engine::graph::mesh::{Mesh,VertexAttr};
use crate::game::model_game::ModelGame;
use gltf::Semantic;
use std::process::id;
use crate::engine::audio::Audio;
use std::thread;
use std::time::Duration;

mod engine;
mod game;

fn main(){
    // Create a simple blip sound
//     let mut sample = usfx::Sample::default();
//     sample.volume(1.0);
//
// // Use a sine wave oscillator at 500 hz
//     sample.osc_type(usfx::OscillatorType::Sine);
//     sample.osc_frequency(500);
//
// // Set the envelope
//     sample.env_attack(0.02);
//     sample.env_decay(0.05);
//     sample.env_sustain(0.2);
//     sample.env_release(0.5);
//
// // Add some distortion
//     sample.dis_crunch(0.5);
//     sample.dis_drive(0.9);
//
// // Create a mixer so we can play the sound
//     let mut mixer = usfx::Mixer::default();
// // Play our sample
//     mixer.play(sample);
//     mixer.generate(&mut audio_device_buffer);

    let mut audio = Audio::new();

    let mut sample = usfx::Sample::default();
    sample.osc_frequency(1000);
    sample.osc_type(usfx::OscillatorType::Sine);
    sample.env_attack(0.1);
    sample.env_decay(0.1);
    sample.env_sustain(0.5);
    sample.env_release(0.5);
    sample.dis_crunch(0.2);

    // Play a low sample with a square wave
    audio.play(sample);

    // Spawn a background thread where an audio device is opened with cpal
    audio.run();

    thread::sleep(Duration::from_millis(3_000));

    // ResourceLoader::init();
    // let game = Box::new(ModelGame::new());
    // let mut engine = GameEngine::new(1280,720,game);
    // engine.run();
}

#[test]
fn test_font(){
    let font = Font::new("NotoSansSC-Thin.otf",18);
    let text = String::from("生当作人杰，死亦为鬼雄。至今思项羽，不肯过江东。");
    for char in text.chars() {
        let c = font.read(char as usize);
        println!("char {},{:?}",char,c);
    }
}

#[test]
fn test_resource_load(){
    let shader_code = ResourceLoader::load_shader("terrain.vert");
    println!("shader code : {:?}",shader_code);
}

#[test]
fn test_usfx(){
    // Create a simple blip sound
    let mut sample = usfx::Sample::default();
    sample.volume(0.5);

// Use a sine wave oscillator at 500 hz
    sample.osc_type(usfx::OscillatorType::Sine);
    sample.osc_frequency(500);

// Set the envelope
    sample.env_attack(0.02);
    sample.env_decay(0.05);
    sample.env_sustain(0.2);
    sample.env_release(0.5);

// Add some distortion
    sample.dis_crunch(0.5);
    sample.dis_drive(0.9);

// Create a mixer so we can play the sound
    let mut mixer = usfx::Mixer::default();

// Play our sample
    mixer.play(sample);
}
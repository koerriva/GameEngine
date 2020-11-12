
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use cpal::traits::{StreamTrait, HostTrait, DeviceTrait};
use cpal::{StreamConfig, SampleRate, SampleFormat, BufferSize, OutputCallbackInfo, Sample};
use usfx::Mixer;
use noise::{Worley, NoiseFn};

const SAMPLE_RATE: usize = 48_000;

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
            .expect("没有音频输出设备");
        println!("音频设备 {}",device.name().unwrap());

        let mut supported_configs_range = device.supported_output_configs()
            .expect("查询音频输出配置出错");
        let supported_config = supported_configs_range.next()
            .expect("没有支持的配置")
            .with_max_sample_rate();

        let sample_format = supported_config.sample_format();
        println!("采样器 {:?}",sample_format);
        let config = supported_config.into();
        println!("音频配置 {:?}",config);

        let error_fn = |err| eprintln!("播放音频是出现一个错误{}", err);

        let write_data = move|buffer:&mut [f32],_:&OutputCallbackInfo|{
            println!("播放");
            mixer.lock().unwrap().generate(buffer)
        };

        let stream = device.build_output_stream(&config,write_data,error_fn)
            .expect("could not build output stream");

        println!("播放序列...");
        stream.play()
            .expect("could not play stream");
    }
}

fn test_audio(){
    let mut sample = usfx::Sample::default();
    sample.volume(1.0);
    sample.osc_frequency(100);
    sample.osc_type(usfx::OscillatorType::Sine);
    sample.env_attack(0.1);
    sample.env_decay(0.1);
    sample.env_sustain(0.5);
    sample.env_release(0.5);
    sample.dis_crunch(0.2);

    let mut mixer = Mixer::new(48_000);


    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let supported_config = device.supported_output_configs().unwrap().next()
        .unwrap()
        .with_max_sample_rate();
    let config = supported_config.into();
    let noise = Worley::default();
    let mut x = 1.0;
    let mut y = 3.0;
    let steam = device.build_output_stream(&config,move|buffer:&mut [f32],_:&OutputCallbackInfo|{
        let value = (noise.get([x, y])+1.0)*1000.0;
        println!("steam ... {},{},{}",x,y,value);
        sample.osc_frequency(value as usize);
        mixer.play(sample);
        mixer.generate(buffer);
        x+=1.0;
        y+=1.0;
    },|err|{
        eprintln!("error {:?}",err)
    }).expect("ddd");
    steam.play();
}
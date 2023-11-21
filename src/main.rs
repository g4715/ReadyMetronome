use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Data, StreamConfig};
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Set up host and default audio device
    let host = cpal::default_host();

    // TODO: Allow changing output device, enumerate with devices()
    let device = host
        .default_output_device()
        .expect("no output device available");

    // Query device supported output configs
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = StreamConfig::from(supported_config);

    // load the click.wav file
    let mut click_file = File::open("./src/assets/EmeryBoardClick.wav")?;
    let mut buf = vec![];
    let result = click_file.read_to_end(&mut buf).expect("Error: Unable to load file into buffer");

    // Create output stream
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // react to stream events and read or write stream data here.

        },
        move |err| {
            // react to errors here.
            println!("Output Steam Error: {}", err);
        },
        None, // None=blocking, Some(Duration)=timeout
    );
    Ok(())
}

// use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use cpal::{Data, FromSample, Sample, SampleFormat, StreamConfig};
// use std::fs::File;
// use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() -> std::io::Result<()> {
    
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("./src/assets/EmeryBoardClick.wav").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
    
    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));

    // // Set up host and default audio device
    // let host = cpal::default_host();

    // // TODO: Allow changing output device, enumerate with devices()
    // let device = host
    //     .default_output_device()
    //     .expect("no output device available");

    // // Query device supported output configs
    // let mut supported_configs_range = device
    //     .supported_output_configs()
    //     .expect("error while querying configs");
    // let supported_config = supported_configs_range
    //     .next()
    //     .expect("no supported config?!")
    //     .with_max_sample_rate();

    // let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    // // Load click file into buffer
    // let mut click_file = File::open("./src/assets/EmeryBoardClick.wav")?;
    // let mut buf = vec![];
    // click_file.read_to_end(&mut buf).expect("Error: Unable to load file into buffer");


    // let sample_format = supported_config.sample_format();
    // let config = supported_config.into();
    // let stream = match sample_format {
    //     SampleFormat::F32 => {
    //         device.build_output_stream(&config, write_buffer::<f32>, err_fn, None)
    //     }
    //     SampleFormat::I16 => {
    //         device.build_output_stream(&config, write_buffer::<i16>, err_fn, None)
    //     }
    //     SampleFormat::U16 => {
    //         device.build_output_stream(&config, write_buffer::<u16>, err_fn, None)
    //     }
    //     sample_format => panic!("Unsupported sample format '{sample_format}'"),
    // }.unwrap();


    // fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    //     for sample in data.iter_mut() {
    //         *sample = Sample::EQUILIBRIUM;
    //     }
    // }

    // stream.play().unwrap();

    Ok(())
}

// // Set up host and default audio device
// let host = cpal::default_host();

// // TODO: Allow changing output device, enumerate with devices()
// let device = host
//     .default_output_device()
//     .expect("no output device available");

// // Query device supported output configs
// let mut supported_configs_range = device
//     .supported_output_configs()
//     .expect("error while querying configs");
// let supported_config = supported_configs_range
//     .next()
//     .expect("no supported config?!")
//     .with_max_sample_rate();
// let config = StreamConfig::from(supported_config);

// // load the click.wav file into buffer
// let mut click_file = File::open("./src/assets/EmeryBoardClick.wav")?;
// let mut buf = vec![];
// let result = click_file.read_to_end(&mut buf).expect("Error: Unable to load file into buffer");

// let &mut data :cpal::Data;
// data.

// // Create output stream
// let stream = device.build_output_stream(
//     &config,
//     move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
//         // react to stream events and read or write stream data here.

//     },
//     move |err| {
//         // react to errors here.
//         println!("Output Steam Error: {}", err);
//     },
//     None, // None=blocking, Some(Duration)=timeout
// );

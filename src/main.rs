use std::{thread, time};
use std::io;
use std::fs::File;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use metronome::Metronome;

mod app;
mod ui;
mod metronome;

fn main() {
    // let mut program_running = true;
    // let default_bpm = get_ms_from_bpm(120);
    
    // // Set up Atomics for metronome thread
    // let bpm = Arc::new(AtomicU64::new(default_bpm));
    // let bpm_clone = Arc::clone(&bpm);
    // let metronome_running = Arc::new(AtomicBool::new(false));
    // let metronome_running_clone = Arc::clone(&metronome_running);

    // let metronome_thread = thread::spawn(move || {
    //     let mut running = false;
    //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //     loop {
    //         if running {
    //             // TODO: Don't load the sample every time, if possible load once and replay. Convert to Sink
    //             let file = io::BufReader::new(File::open("./src/assets/EmeryBoardClick.wav").unwrap());
    //             let source = Decoder::new(file).unwrap();
    //             let _ = stream_handle.play_raw(source.convert_samples());
    //             spin_sleep::sleep(time::Duration::from_millis(bpm_clone.load(Ordering::Relaxed)));
    //         }
    //         running = metronome_running_clone.load(Ordering::Relaxed);
    //     }
    // });

    // while program_running {     
    //     let choice = get_input("q to quit, w to toggle metronome, r to change bpm");
    //     if choice == "q" {
    //         program_running = false;
    //     } else if choice == "w" {
    //         let currently_running = metronome_running.load(Ordering::Relaxed);
    //         metronome_running.swap(!currently_running, Ordering::Relaxed);
    //     } else if choice == "r" {
    //         let mut new_bpm = get_input("Input the new bpm:").parse().unwrap();
    //         new_bpm = get_ms_from_bpm(new_bpm);
    //         bpm.swap(new_bpm, Ordering::Relaxed);
    //     }
    // }
    // drop(metronome_thread);

    let mut metronome = Metronome::new();
    let bpm = Arc::clone(&metronome.settings.bpm);
    let volume = Arc::clone(&metronome.settings.volume);
    let is_running = Arc::clone(&metronome.settings.is_running);
    
    metronome.init();

}

// Convert a bpm value to the millisecond delay
fn get_ms_from_bpm(bpm :u64) -> u64 {
    let result :u64 = (60_000.0_f64 / bpm as f64).round() as u64;
    result
}

// Adapted from this: https://users.rust-lang.org/t/how-to-get-user-input/5176/8
// Taken verbatim from my implementation in HW2
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }
    input.trim().to_string()
}

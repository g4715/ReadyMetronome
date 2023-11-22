use std::fs::File;
use std::io::BufReader;
use std::{thread, time};
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{Source};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use std::io;

fn main() {
    let default_bpm = get_ms_from_bpm(120);
    let mut program_running = true;
    
    // Set up Atomics for metronome thread
    let bpm = Arc::new(AtomicU64::new(default_bpm));
    let bpm_clone = Arc::clone(&bpm);
    let metronome_running = Arc::new(AtomicBool::new(true));
    let metronome_running_clone = Arc::clone(&metronome_running);

    let metronome_thread = thread::spawn(move || {
        let mut looping = true;
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        loop {
            if looping {
                let file = BufReader::new(File::open("./src/assets/EmeryBoardClick.wav").unwrap());
                let source = Decoder::new(file).unwrap();
                stream_handle.play_raw(source.convert_samples());
                let now = time::Instant::now();
                spin_sleep::sleep(std::time::Duration::from_millis(bpm_clone.load(Ordering::Relaxed)));
                println!("{:?}", now.elapsed());
            }
            looping = metronome_running_clone.load(Ordering::Relaxed);
        }
    });

    while program_running {     
        let choice = get_input("q to quit, w to toggle metronome, r to change bpm");
        if choice == "q" {
            program_running = false;
        } else if choice == "w" {
            let currently_running = metronome_running.load(Ordering::Relaxed);
            metronome_running.swap(!currently_running, Ordering::Relaxed);
        } else if choice == "r" {
            let mut new_bpm = get_input("Input the new bpm:").parse().unwrap();
            new_bpm = get_ms_from_bpm(new_bpm);
            bpm.swap(new_bpm, Ordering::Relaxed);
        }
    }
    drop(metronome_thread);
}

// Convert a bpm value to the millisecond delay
fn get_ms_from_bpm(bpm :u64) -> u64 {
    let result :u64 = (60_000.0 as f64 / bpm as f64).round() as u64;
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

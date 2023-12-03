use std::{thread, time};
use std::io;
use std::fs::File;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use atomic_float::AtomicF64;
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use metronome::Metronome;

mod app;
mod ui;
mod metronome;

fn main() {
    let mut program_running = true;
    
    let bpm = Arc::new(AtomicU64::new(500));
    let volume = Arc::new(AtomicF64::new(1.0));
    let is_running = Arc::new(AtomicBool::new(true));
    let mut metronome = Metronome::new(&bpm, &volume, &is_running);
    
    let metronome_thread = thread::spawn(move || {
        metronome.init();
    });

    while program_running {     
        let choice = get_input("q to quit, w to toggle metronome, r to change bpm");
        if choice == "q" {
            program_running = false;
        } else if choice == "w" {
            let currently_running = is_running.load(Ordering::Relaxed);
            is_running.swap(!currently_running, Ordering::Relaxed);
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

use app::App;
use atomic_float::AtomicF64;
use metronome::Metronome;
use rodio::source::Source;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::{thread, time};

mod app;
mod metronome;
mod ui;

fn main() {
    let mut program_running = true;

    // let bpm = Arc::new(AtomicU64::new(500));
    // let volume = Arc::new(AtomicF64::new(1.0));
    // let is_running = Arc::new(AtomicBool::new(true));

    let mut app = App::new(500, 1.0, true);
    app.init();

    while program_running {
        let choice = get_input("q to quit, w to toggle metronome, r to change bpm");
        if choice == "q" {
            program_running = false;
        } else if choice == "w" {
            app.toggle_metronome();
        } else if choice == "r" {
            let mut new_bpm = get_input("Input the new bpm:").parse().unwrap();
            app.change_bpm(new_bpm);
        }
    }
    // app.cleanup();
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

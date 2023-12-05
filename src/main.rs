// Ratatui portions of this are taken from https://ratatui.rs/tutorials/json-editor/main/
// I have added comments about each piece of code from there to illuminate what it does

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::{error::Error, io};

mod app;
mod ui;
use crate::{
    app::App,
    events::run_app,
};

mod menu;
mod metronome;
mod events;

fn main() -> Result<(), Box<dyn Error>> {
    // This is neccessary Ratatui boilerplate, enables Ratatui to have control over the keyboard inputs as well as mouse
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // This sets up Crossterm for our backend and gives it a terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Initialize the app
    let mut app = App::new(120, 500, 100.0, false);
    app.init();

    let res = run_app(&mut terminal, &mut app);

    // This begins the clean up phase after the app quits
    // Restores the terminal to its original state after exiting the program
    disable_raw_mode()?;

    // Leave the alternate screen created by ratatui
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Check to see if the app errored out and print that to terminal
    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

// TODO: Reimplement Commandline Mode, select it by passing -c flag
// let mut program_running = true;
// while program_running {
//     let choice = get_input("q to quit, w to toggle metronome, r to change bpm");
//     if choice == "q" {
//         program_running = false;
//     } else if choice == "w" {
//         app.toggle_metronome();
//     } else if choice == "r" {
//         let mut new_bpm = get_input("Input the new bpm:").parse().unwrap();
//         app.change_bpm(new_bpm);
//     }
// }
// app.cleanup();

// Adapted from this: https://users.rust-lang.org/t/how-to-get-user-input/5176/8
// Taken verbatim from my implementation in HW2
// fn get_input(prompt: &str) -> String {
//     println!("{}", prompt);
//     let mut input = String::new();
//     match io::stdin().read_line(&mut input) {
//         Ok(_goes_into_input_above) => {}
//         Err(_no_updates_is_fine) => {}
//     }
//     input.trim().to_string()
// }

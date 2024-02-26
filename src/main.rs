use clap::Parser;
/// This file is the main entrypoint and handles starting the app as well as initializing
/// and cleaning up the ratatui interface.
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io};

mod app;
mod ui;
use crate::{app::App, events::run_app, metronome::InitMetronomeSettings};

mod event_handler;
mod events;
mod menu;
mod metronome;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // This is neccessary Ratatui boilerplate, enables Ratatui to have control over the keyboard inputs as well as mouse
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // This sets up Crossterm for our backend and gives it a terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Initialize the app
    const TICK_RATE: u64 = 7;
    let init_settings: InitMetronomeSettings = InitMetronomeSettings {
        bpm: 120,
        ms_delay: 500,
        ts_note: 4,
        ts_value: 4,
        volume: 100.0,
        is_running: false,
        debug: args.debug,
    };

    let mut app = App::new(init_settings, TICK_RATE);
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether or not we are in debug mode
    #[arg(short, long)]
    debug: bool,
}

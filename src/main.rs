// Ratatui portions of this are taken from https://ratatui.rs/tutorials/json-editor/main/
// I have added comments about each piece of code from there to illuminate what it does

use std::{error::Error, io};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
        KeyEventKind,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
    widgets::{ListState, ListItem},
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

mod metronome;

// TODO: Create a command-line version of the app that can be accessed with -c flag
fn main() -> Result<(), Box<dyn Error>>{
    // This is neccessary Ratatui boilerplate, enables Ratatui to have control over the keyboard inputs as well as mouse
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // This sets up Crossterm for our backend and gives it a terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // This is my code for setting up the application
    let mut program_running = true;
    let mut app = App::new(500, 1.0, true);
    app.init();

    let res = run_app(&mut terminal, &mut app);

    // This restores the terminal to its original state after exiting the program
    disable_raw_mode()?;            // Gives keyboard control back

    // Leaves the alternate screen created by ratatui
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
}

// This function controls the application in Ratatui mode, the generic Backend is to allow for support for
// more backends than just Crossterm
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    // This will populate the main menu list and give it a selectable state for arrow key navigation
    let mut list_state = ListState::default().with_selected(Some(0));
    let items = [ListItem::new("Start / Stop Metronome"), ListItem::new("Change BPM"), ListItem::new("Quit")];
    // This is the main UI loop
    // Reference https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html
    loop {
        terminal.draw(|f| ui(f, app, &list_state, &items))?;         // Draw a frame to the terminal by passing it to our ui function in ui.rs

        // Crossterm: Poll for keyboard events and make choices based on app's current screen
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            // global and menu navigation controls
            match key.code { 
                KeyCode::Up => {
                    // list_state.offset()
                }
                KeyCode::Down => {
                    // list_state.offset()
                }
                KeyCode::Enter => {
                    if app.current_screen != CurrentScreen::Exiting {
                        // list_state.selected();
                        continue;
                    }
                }
                KeyCode::Char('t') => {
                        app.toggle_metronome();
                }
                _ => {}
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    // KeyCode::Char('b') => {
                    //     // change bpm
                    // }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                }   
                CurrentScreen::Editing => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Backspace | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }   
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') | KeyCode::Enter => {
                        return Ok(());
                    }
                    KeyCode::Char('n') | KeyCode::Backspace | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                }   
            }
        }

    }
}





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


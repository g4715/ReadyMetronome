// Ratatui portions of this are taken from https://ratatui.rs/tutorials/json-editor/main/
// I have added comments about each piece of code from there to illuminate what it does

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
    menu::Menu,
};

mod metronome;
mod menu;

fn main() -> Result<(), Box<dyn Error>> {
    // This is neccessary Ratatui boilerplate, enables Ratatui to have control over the keyboard inputs as well as mouse
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // This sets up Crossterm for our backend and gives it a terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Initialize the app
    let mut app = App::new(120, 500, 1.0, false);
    app.init();

    let res = run_app(&mut terminal, &mut app);

    // This begins the clean up phase after the app quits
    // Restores the terminal to its original state after exiting the program
    disable_raw_mode()?; // Gives keyboard control back

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

// This function controls the application in Ratatui mode, the generic Backend is to allow for support for
// more backends than just Crossterm
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let main_menu_vec = vec!(
        "Start / Stop Metronome".to_string(),
        "Edit Metronome Settings".to_string(),
        "Quit".to_string(),
    );
    let editing_menu_vec = vec!(
        "Change BPM".to_string(),
        "Back to Main Menu".to_string(),
    );

    let mut control_menu = Menu::new(main_menu_vec.clone());
    control_menu.select(0);

    // This is the main UI loop
    loop {
        terminal.draw(|f| ui(f, app, &mut control_menu.state, &control_menu.items))?; // Draw a frame to the terminal by passing it to our ui function in ui.rs

        // Crossterm: Poll for keyboard events and make choices based on app's current screen
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            // global keyboard shortcuts and menu navigation controls
            match key.code {
                KeyCode::Up | KeyCode::Left | KeyCode::BackTab => {
                    if app.current_screen != CurrentScreen::Exiting {
                        control_menu.previous();
                        continue;
                    }
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Tab => {
                    if app.current_screen != CurrentScreen::Exiting {
                        control_menu.next();
                        continue;
                    }
                }
                KeyCode::Enter => {
                    if app.current_screen == CurrentScreen::Main {
                        // TODO: This is messy and currently this uses magic numbers, replace that with behavior based on selected item by name, not index
                        let current_selection = control_menu.state.selected().unwrap();
                        match current_selection {
                            0 => {
                                app.toggle_metronome();
                            }
                            1 => {
                                app.current_screen = CurrentScreen::Editing;
                                control_menu.set_items(editing_menu_vec.clone());
                            }
                            2 => {
                                app.current_screen = CurrentScreen::Exiting;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    if app.current_screen == CurrentScreen::Editing {
                        let current_selection = control_menu.state.selected().unwrap();
                        match current_selection {
                            0 => {
                                // app.toggle_metronome();
                                // Open change bpm box
                            }
                            1 => {
                                app.current_screen = CurrentScreen::Main;
                                control_menu.set_items(main_menu_vec.clone());
                            }
                            _ => {}
                        }
                        continue;
                    }
                }
                KeyCode::Char('t') => {
                    app.toggle_metronome();
                }
                KeyCode::Char('q') => {
                    if app.current_screen != CurrentScreen::Exiting {
                        app.current_screen = CurrentScreen::Exiting;
                        continue;
                    }
                }
                _ => {}
            }

            // Screen specific keyboard shortcuts
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('b') => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Backspace | KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('q') | KeyCode::Enter => {
                        return Ok(());
                    }
                    KeyCode::Char('n') | KeyCode::Backspace | KeyCode::Esc | KeyCode::Tab => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
            }
        }
    }
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

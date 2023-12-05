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
    menu::Menu,
    ui::ui,
};

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

mod menu;
mod metronome;

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
    let main_menu_vec = vec![
        "Start / Stop Metronome".to_string(),
        "Edit Metronome Settings".to_string(),
        "Quit".to_string(),
    ];
    // let edit_menu_vec = vec!["Change BPM".to_string(), "Back to Main Menu".to_string()];
    let mut is_playing;

    if app.settings.is_running.load(Ordering::Relaxed) == true {
        is_playing = "yes".to_string();
    } else {
        is_playing = "no".to_string();
    }  
    let mut current_edit_menu_selection: Option<usize>;
    let mut edit_menu_vec = vec![
        "playing: ".to_owned() + &is_playing,
        "bpm: ".to_owned() + &app.settings.bpm.load(Ordering::Relaxed).to_string(),
        "volume: ".to_owned() + &app.settings.volume.load(Ordering::Relaxed).to_string(),
        "ms_delay: ".to_owned() + &app.settings.ms_delay.load(Ordering::Relaxed).to_string(),        
    ];
    let mut edit_menu = Menu::new(edit_menu_vec.clone());
    let mut main_menu = Menu::new(main_menu_vec.clone());
    main_menu.select(0);

    // This is the main UI loop
    loop {
        // Refresh Status/Edit menu
        if app.settings.is_running.load(Ordering::Relaxed) == true {
            is_playing = "yes".to_string();
        } else {
            is_playing = "no".to_string();
        }
        current_edit_menu_selection = edit_menu.state.selected();
        edit_menu_vec = vec![
            "playing: ".to_owned() + &is_playing,
            "bpm: ".to_owned() + &app.settings.bpm.load(Ordering::Relaxed).to_string(),
            "volume: ".to_owned() + &app.settings.volume.load(Ordering::Relaxed).to_string(),
            "Back to main menu".to_owned(),   
        ];
        edit_menu.set_items(edit_menu_vec.clone());
        if current_edit_menu_selection.is_some() {
            edit_menu.select(current_edit_menu_selection.unwrap());
        }
        terminal.draw(|f| ui(f, app, &mut main_menu, &mut edit_menu))?; // Draw a frame to the terminal by passing it to our ui function in ui.rs
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
                        if app.current_screen == CurrentScreen::Main {
                            main_menu.previous();
                        } else if app.current_screen == CurrentScreen::Editing {
                            edit_menu.previous();
                        }
                        continue;
                    }
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Tab => {
                    if app.current_screen != CurrentScreen::Exiting {
                        if app.current_screen == CurrentScreen::Main {
                            main_menu.next();
                        } else if app.current_screen == CurrentScreen::Editing {
                            edit_menu.next();
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
                    KeyCode::Enter => {
                        let current_selection = main_menu.state.selected().unwrap();
                        // TODO: This is messy and bad, magic numbers are not scalable
                        match current_selection {
                            0 => {
                                // start / stop metronome
                                app.toggle_metronome();
                            }
                            1 => {
                                // enter edit menu
                                main_menu.deselect();
                                edit_menu.select(0);
                                app.current_screen = CurrentScreen::Editing;
                            }
                            2 => {
                                // enter quit menu
                                app.current_screen = CurrentScreen::Exiting;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Enter => {
                        // TODO: This is messy and bad, magic numbers are not scalable
                        let current_selection = edit_menu.state.selected().unwrap();
                        match current_selection {
                            0 => {
                                // start / stop metronome
                                app.toggle_metronome()
                            }
                            1 => {
                                // edit bpm
                            }
                            2 => {
                                // edit volume
                            }
                            3 => {
                                // back to main menu
                                edit_menu.deselect();
                                main_menu.select(0);
                                app.current_screen = CurrentScreen::Main;
                            }
                            _ => {}
                        }
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

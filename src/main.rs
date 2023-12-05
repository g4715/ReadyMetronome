// Ratatui portions of this are taken from https://ratatui.rs/tutorials/json-editor/main/
// I have added comments about each piece of code from there to illuminate what it does

// References
// List state / Menu reference: https://docs.rs/ratatui/latest/ratatui/widgets/trait.StatefulWidget.html
// List: https://docs.rs/ratatui/latest/ratatui/widgets/struct.List.html

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::{ListItem, ListState},
    Terminal,
};
use std::{error::Error, io, thread::current};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

mod metronome;

fn main() -> Result<(), Box<dyn Error>> {
    // This is neccessary Ratatui boilerplate, enables Ratatui to have control over the keyboard inputs as well as mouse
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    // This sets up Crossterm for our backend and gives it a terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Initialize the app
    let mut app = App::new(500, 1.0, false);
    app.init();

    let res = run_app(&mut terminal, &mut app);

    // This restores the terminal to its original state after exiting the program
    disable_raw_mode()?; // Gives keyboard control back

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
}

pub struct Menu {
    pub items: Vec<String>,
    pub state: ListState,
}

// This provides a struct to hold a selectable menu state. See stateful widget reference above.
impl Menu {
    fn new(items: Vec<String>) -> Menu {
        Menu {
            items,
            state: ListState::default(),
        }
    }
    // Resets the menu items and selects the first on the list
    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
        self.state.select(Some(0));
    }
    // Select the next item in the list
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    // Select the previous item in the list
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    // Deselect (unused for now)
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}


// This function controls the application in Ratatui mode, the generic Backend is to allow for support for
// more backends than just Crossterm
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut current_menu = Menu::new(vec!(
        "Start / Stop Metronome".to_string(),
        "Change BPM".to_string(),
        "Quit".to_string(),
    ));

    // This is the main UI loop
    loop {
        terminal.draw(|f| ui(f, app, &mut current_menu.state, &current_menu.items))?; // Draw a frame to the terminal by passing it to our ui function in ui.rs

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
                        current_menu.previous();
                        // let i = match list_state.selected() {
                        //     Some(i) => {
                        //         if i == 0 {
                        //             items.len() - 1
                        //         } else {
                        //             i - 1
                        //         }
                        //     }
                        //     None => 0,
                        // };
                        // list_state.select(Some(i));
                    }
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Tab => {
                    if app.current_screen != CurrentScreen::Exiting {
                        // let i = match list_state.selected() {
                        //     Some(i) => {
                        //         if i >= items.len() - 1 {
                        //             0
                        //         } else {
                        //             i + 1
                        //         }
                        //     }
                        //     None => 0,
                        // };
                        // list_state.select(Some(i));
                        current_menu.next();
                    }
                }
                KeyCode::Enter => {
                    if app.current_screen != CurrentScreen::Exiting {
                        // TODO: Currently this uses magic numbers, replace that with behavior based on selected item by name, not index
                        let current_selection = current_menu.state.selected().unwrap();
                        match current_selection {
                            0 => {
                                app.toggle_metronome();
                            }
                            1 => {
                                app.current_screen = CurrentScreen::Editing;
                            }
                            2 => {
                                app.current_screen = CurrentScreen::Exiting;
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
                    KeyCode::Char('n') | KeyCode::Backspace | KeyCode::Esc => {
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

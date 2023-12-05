/// Events.rs: This file houses the event loop and ui draw calls for the ratatui application
use crossterm::event::{self, Event, KeyCode};

use ratatui::{backend::Backend, Terminal};
use std::{error::Error, io};

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    menu::Menu,
    ui::ui,
};

// This function controls the application in Ratatui mode, the generic Backend is to allow for support for
// more backends than just Crossterm
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    // Initialize Main Menu
    let main_menu_vec = vec![
        "Start / Stop Metronome".to_string(),
        "Edit Metronome Settings".to_string(),
        "Quit".to_string(),
    ];
    let mut main_menu = Menu::new(main_menu_vec);
    main_menu.select(0);

    // Create edit menu (it gets initialized in the loop when refreshed)
    let mut edit_menu = Menu::new(vec![]);
    let mut first_edit = true; // this is used to overwrite the original metronome setting text upon editing

    // This is the main UI loop
    loop {
        refresh_edit_list(&mut edit_menu, app);

        // Draw a frame to the terminal by passing it to our ui function in ui.rs
        terminal.draw(|f| ui(f, app, &mut main_menu, &mut edit_menu))?;

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
                        } else if app.current_screen == CurrentScreen::Editing
                            && app.currently_editing.is_none()
                        {
                            edit_menu.previous();
                        }
                        continue;
                    }
                }
                KeyCode::Down | KeyCode::Right | KeyCode::Tab => {
                    if app.current_screen != CurrentScreen::Exiting {
                        if app.current_screen == CurrentScreen::Main {
                            main_menu.next();
                        } else if app.current_screen == CurrentScreen::Editing
                            && app.currently_editing.is_none()
                        {
                            edit_menu.next();
                        }
                        continue;
                    }
                }
                KeyCode::Char('t') => {
                    if app.currently_editing.is_none() {
                        app.toggle_metronome();
                    }
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
            // Main screen ---------------------------------------------------------------------------------------------
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
                // Edit screen -----------------------------------------------------------------------------------------
                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc => {
                        if app.currently_editing.is_some() {
                            edit_menu.select(0);
                            app.currently_editing = None;
                            app.clear_edit_strs();
                        } else {
                            app.current_screen = CurrentScreen::Main;
                            edit_menu.deselect();
                            main_menu.select(1);
                        }
                    }
                    KeyCode::Char(value) => {
                        if app.currently_editing.is_some() {
                            if first_edit {
                                app.edit_string.clear();
                                first_edit = false;
                            }
                            app.edit_string.push(value);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.currently_editing.is_some() {
                            app.edit_string.pop();
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Bpm => {
                                    if app.change_bpm() {
                                        edit_menu.select(1);
                                        first_edit = true;
                                    } else {
                                        app.alert_string =
                                            "Please input a value between 20 and 500".to_owned();
                                    }
                                }
                                CurrentlyEditing::Volume => {
                                    if app.change_volume() {
                                        edit_menu.select(2);
                                        first_edit = true;
                                    } else {
                                        app.alert_string =
                                            "Please input a value between 1.0 and 200.0".to_owned();
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            // Main edit menu --------------------------------------------
                            // TODO: This is messy and bad, magic numbers are not scalable
                            let current_selection = edit_menu.state.selected().unwrap();
                            match current_selection {
                                0 => {
                                    // start / stop metronome
                                    app.toggle_metronome()
                                }
                                1 => {
                                    // edit bpm
                                    app.edit_string = app.get_bpm().to_string();
                                    app.currently_editing = Some(CurrentlyEditing::Bpm);
                                    edit_menu.deselect();
                                }
                                2 => {
                                    // edit volume
                                    app.edit_string = app.get_volume().to_string();
                                    app.currently_editing = Some(CurrentlyEditing::Volume);
                                    edit_menu.deselect();
                                }
                                3 => {
                                    // back to main menu
                                    edit_menu.deselect();
                                    main_menu.select(1);
                                    app.current_screen = CurrentScreen::Main;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                // Exit screen -----------------------------------------------------------------------------------------
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

// Refresh Status/Edit menu
fn refresh_edit_list(edit_menu: &mut Menu, app: &mut App) {
    let edit_menu_selection = edit_menu.state.selected();
    let is_playing = if app.get_is_running() { "yes" } else { "no" };
    let edit_menu_vec = vec![
        "playing: ".to_owned() + is_playing,
        "bpm: ".to_owned() + &app.get_bpm().to_string(),
        "volume: ".to_owned() + &app.get_volume().to_string(),
        "Back to main menu".to_owned(),
    ];
    edit_menu.set_items(edit_menu_vec);
    if let Some(..) = edit_menu_selection {
        edit_menu.select(edit_menu_selection.unwrap());
    }
}

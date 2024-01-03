/// Events.rs: This file handles the user keyboard interaction event loop and ui draw calls for the ratatui application
// This event handling is based off of the example found here: https://ratatui.rs/tutorials/counter-app/multiple-files/event/
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, MouseEvent};

use color_eyre::Result;
use ratatui::{backend::Backend, Terminal};
use std::io::{self, Error as IOError, ErrorKind};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::event_handler::Event;
use crate::{app::App, event_handler::EventHandler, ui::ui};

// This function controls the application in Ratatui mode, It polls for user input and updates the various menus / app.state appropriately
// the generic Backend parameter is to allow for support for more backends than just Crossterm.
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<String> {
    // Initialize Main Menu
    // let main_menu_vec = vec![
    //     "Start / Stop Metronome".to_string(),
    //     "Edit Metronome Settings".to_string(),
    //     "Quit".to_string(),
    // ];
    // let mut main_menu = Menu::new(main_menu_vec);
    // main_menu.select(0);

    // // Create edit menu (it gets initialized in the loop when refreshed)
    // let mut edit_menu = Menu::new(vec![]);

    let mut first_edit = true; // this is used to overwrite the original metronome setting text upon opening the edit window

    let events = EventHandler::new(120);
    loop {
        app.check_error_status();
        app.refresh_edit_menu();
        if app.should_quit {
            break;
        }

        terminal.draw(|f| ui(f, app))?;

        match events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => match app.update(key_event, first_edit) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    Ok("exited successfully".to_string())
}

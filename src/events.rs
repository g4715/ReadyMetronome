/// Events.rs: This file handles the user keyboard interaction event loop and ui draw calls for the ratatui application
// This event handling is based off of the example found here: https://ratatui.rs/tutorials/counter-app/multiple-files/event/
use crate::{
    app::App,
    event_handler::{Event, EventHandler},
    ui::ui,
};
use color_eyre::Result;
use ratatui::{backend::Backend, Terminal};

// This function controls the application in Ratatui mode, It polls for user input and updates the various menus / app.state appropriately
// the generic Backend parameter is to allow for support for more backends than just Crossterm.
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<String> {
    let events = EventHandler::new(7);
    loop {
        app.check_error_status();
        app.refresh_edit_menu();
        if app.should_quit {
            break;
        }

        terminal.draw(|f| ui(f, app))?;

        match events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => match app.update(key_event) {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::FocusChange(_) => {}
        }
    }

    Ok("exited successfully".to_string())
}

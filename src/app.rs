// App will hold the current application state of Ready Metronome. It keeps track of the current screen, quitting,
// and various settings on the metronome like the bpm, volume and whether or not it is playing.

// This is loosely based on the ratatui JSON editor tutorial found here: https://ratatui.rs/tutorials/json-editor/app/

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Bpm,
    Volume,
    IsPlaying,
}

pub struct App {
    pub bpm: i32,
    pub volume: f32,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            bpm: i32::new(),
            volume: f32::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Bpm => {
                    self.currently_editing = Some(CurrentlyEditing::Bpm)
                }
                CurrentlyEditing::Volume => {
                    self.currently_editing = Some(CurrentlyEditing::Volume)
                }
                CurrentlyEditing::IsPlaying => {
                    self.currently_editing = Some(CurrentlyEditing::IsPlaying)
                }
            }
        }
    }
}
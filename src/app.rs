// App will hold the current application state of Ready Metronome. It keeps track of the current screen, quitting,
// and various settings on the metronome like the bpm, volume and whether or not it is playing.

// This is loosely based on the ratatui JSON editor tutorial found here: https://ratatui.rs/tutorials/json-editor/app/
use crate::metronome::{MetronomeSettings, Metronome};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use atomic_float::AtomicF64;
use std::sync::Arc;
use std::thread;

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
    pub settings: MetronomeSettings,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub metronome_handle: Option<thread::JoinHandle<()>>,
}

impl App {
    pub fn new(set_bpm: &Arc<AtomicU64>, set_volume :&Arc<AtomicF64>, set_is_running :&Arc<AtomicBool>) -> App {
        App {
            settings: MetronomeSettings {
                bpm: Arc::clone(set_bpm),
                volume: Arc::clone(set_volume),
                is_running: Arc::clone(set_is_running),
            },
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            metronome_handle: None,
        }
    }

    pub fn spawn_metronome_thread(&mut self) {
        let mut metronome = Metronome::new(&self.settings);
        self.metronome_handle = Some(thread::spawn(move || {
            metronome.start();
        }));
    }

    pub fn init(&mut self) {
        self.spawn_metronome_thread();
    }

    // pub fn cleanup(&mut self) {
    //     drop(self.metronome_handle);
    // }

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
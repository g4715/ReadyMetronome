// App will hold the current application state of Ready Metronome. It keeps track of the current screen, quitting,
// and various settings on the metronome like the bpm, volume and whether or not it is playing. It is additionally
// in charge of starting the metronome thread and keeping a reference to it's handle

// This is loosely based on the ratatui JSON editor tutorial found here: https://ratatui.rs/tutorials/json-editor/app/
use crate::metronome::{Metronome, MetronomeSettings};
use atomic_float::AtomicF64;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

#[derive(PartialEq)]
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
    pub fn new(set_bpm: u64, set_ms_delay: u64, set_volume: f64, set_is_running: bool) -> App {
        App {
            settings: MetronomeSettings {
                bpm: Arc::new(AtomicU64::new(set_bpm)),
                ms_delay: Arc::new(AtomicU64::new(set_ms_delay)),
                volume: Arc::new(AtomicF64::new(set_volume)),
                is_running: Arc::new(AtomicBool::new(set_is_running)),
            },
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            metronome_handle: None,
        }
    }

    pub fn init(&mut self) {
        self.spawn_metronome_thread();
    }

    pub fn cleanup(&mut self) {
        // TODO: Find out how to do this gracefully
        // drop(self.metronome_handle);
    }

    fn spawn_metronome_thread(&mut self) {
        let mut metronome = Metronome::new(&self.settings);
        self.metronome_handle = Some(thread::spawn(move || {
            metronome.start();
        }));
    }

    // Metronome settings change functions
    pub fn change_bpm(&mut self, new_bpm: u64) {
        self.settings.bpm.swap(new_bpm, Ordering::Relaxed);
        let new_ms_delay = self.get_ms_from_bpm(new_bpm);
        self.settings.ms_delay.swap(new_ms_delay, Ordering::Relaxed);
    }

    pub fn toggle_metronome(&mut self) {
        let currently_playing = self.settings.is_running.load(Ordering::Relaxed);
        self.settings
            .is_running
            .swap(!currently_playing, Ordering::Relaxed);
    }

    // Convert a bpm value to the millisecond delay
    fn get_ms_from_bpm(&mut self, bpm: u64) -> u64 {
        let result: u64 = (60_000.0_f64 / bpm as f64).round() as u64;
        result
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Bpm => self.currently_editing = Some(CurrentlyEditing::Bpm),
                CurrentlyEditing::Volume => self.currently_editing = Some(CurrentlyEditing::Volume),
                CurrentlyEditing::IsPlaying => {
                    self.currently_editing = Some(CurrentlyEditing::IsPlaying)
                }
            }
        }
    }
}

/// App.rs holds the current application state of Ready Metronome. It keeps track of the current screen, quitting,
/// and various settings on the metronome like the bpm, volume and whether or not it is playing. It is additionally
/// in charge of starting the metronome thread and keeping a reference to it's handle
// App.rs is loosely based on the ratatui JSON editor tutorial found here: https://ratatui.rs/tutorials/json-editor/app/
use crate::{
    metronome::{Metronome, MetronomeSettings, InitMetronomeSettings},
    menu::Menu,
};
use atomic_float::AtomicF64;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

// These two enums are used extensively in events.rs and ui.rs to render the correct state and
// select the right value when editing
#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
    Error,
}

#[derive(Clone, Copy)]
pub enum CurrentlyEditing {
    Bpm,
    Volume,
}

pub struct App {
    pub settings: MetronomeSettings,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub metronome_handle: Option<thread::JoinHandle<()>>,
    pub edit_string: String,
    pub alert_string: String,
    pub main_menu: Menu,
    pub edit_menu: Menu,
}

impl App {
    pub fn new(init_settings :InitMetronomeSettings) -> App {
        App {
            settings: MetronomeSettings {
                bpm: Arc::new(AtomicU64::new(init_settings.bpm)),
                ms_delay: Arc::new(AtomicU64::new(init_settings.ms_delay)),
                ts_note: Arc::new(AtomicU64::new(init_settings.ts_note)),
                ts_value: Arc::new(AtomicU64::new(init_settings.ts_value)),
                volume: Arc::new(AtomicF64::new(init_settings.volume)),
                is_running: Arc::new(AtomicBool::new(init_settings.is_running)),
                bar_count: Arc::new(AtomicU64::new(0)),
                current_beat_count: Arc::new(AtomicU64::new(0)),
                error: Arc::new(AtomicBool::new(false)),
            },
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            metronome_handle: None,
            edit_string: String::new(),
            alert_string: String::new(),
            main_menu: Menu::new(vec![
                "Start / Stop Metronome".to_string(),
                "Edit Metronome Settings".to_string(),
                "Quit".to_string(),
            ]),
            edit_menu: Menu::new(vec![]),
        }
    }

    pub fn init(&mut self) {
        self.spawn_metronome_thread();
        self.main_menu.select(0);
    }

    // Spawns a metronome on its own thread
    fn spawn_metronome_thread(&mut self) {
        let mut metronome = Metronome::new(&self.settings);
        self.metronome_handle = Some(thread::spawn(move || {
            metronome.start();
        }));
        self.check_error_status();
    }

    // Added these helper functions so app is in charge of its own atomics
    pub fn get_bpm(&mut self) -> u64 {
        self.settings.bpm.load(Ordering::Relaxed)
    }
    pub fn get_volume(&mut self) -> f64 {
        self.settings.volume.load(Ordering::Relaxed)
    }
    pub fn get_is_running(&mut self) -> bool {
        self.settings.is_running.load(Ordering::Relaxed)
    }
    pub fn get_time_sig_string(&mut self) -> String {
        let note = self.settings.ts_note.load(Ordering::Relaxed).to_string();
        let value = self.settings.ts_note.load(Ordering::Relaxed).to_string();
        note + "/" + &value
    }
    pub fn get_bar_count_string(&mut self) -> String {
        self.settings.bar_count.load(Ordering::Relaxed).to_string()
    }

    // Metronome settings change functions
    pub fn change_bpm(&mut self, new_bpm: u64) {
        if !(self.verify_bpm(new_bpm)) {
            return;
        }
        self.settings.bpm.swap(new_bpm, Ordering::Relaxed);
        let new_ms = self.get_ms_from_bpm(new_bpm);
        self.settings.ms_delay.swap(new_ms, Ordering::Relaxed);
    }

    fn verify_bpm(&mut self, test_bpm: u64) -> bool {
        if (20..=500).contains(&test_bpm) {
            return true;
        }
        false
    }

    fn verify_volume(&mut self, test_vol: f64) -> bool {
        if (1.0..=200.0).contains(&test_vol) {
            return true;
        }
        false
    }

    pub fn change_bpm_editor(&mut self) -> bool {
        if self.edit_string.is_empty() {
            false
        } else {
            let new_bpm: u64 = match self.edit_string.parse() {
                Ok(new_value) => new_value,
                Err(_) => return false,
            };
            if self.verify_bpm(new_bpm) {
                self.settings.bpm.swap(new_bpm, Ordering::Relaxed);
                let new_ms_delay = self.get_ms_from_bpm(new_bpm);
                self.settings.ms_delay.swap(new_ms_delay, Ordering::Relaxed);
                self.clear_strings();
                self.currently_editing = None;
                true
            } else {
                self.edit_string.clear();
                false
            }
        }
    }

    pub fn change_volume_editor(&mut self) -> bool {
        if self.edit_string.is_empty() {
            false
        } else {
            let new_volume: f64 = match self.edit_string.parse() {
                Ok(new_value) => new_value,
                Err(_) => return false,
            };
            if self.verify_volume(new_volume) {
                self.settings.volume.swap(new_volume, Ordering::Relaxed);
                self.clear_strings();
                self.currently_editing = None;
                true
            } else {
                self.edit_string.clear();
                false
            }
        }
    }

    pub fn toggle_metronome(&mut self) {
        let currently_playing = self.settings.is_running.load(Ordering::Relaxed);
        self.settings
            .is_running
            .swap(!currently_playing, Ordering::Relaxed);
        // This will trigger if the metronome fails to load a file
        self.check_error_status();
    }

    // Convert a bpm value to the millisecond delay
    fn get_ms_from_bpm(&mut self, bpm: u64) -> u64 {
        (60_000.0_f64 / bpm as f64).round() as u64
    }

    pub fn clear_strings(&mut self) {
        self.alert_string.clear();
        self.edit_string.clear();
    }

    pub fn check_error_status(&mut self) {
        if self.settings.error.load(Ordering::Relaxed) {
            self.current_screen = CurrentScreen::Error;
        }
    }

    pub fn refresh_edit_menu(&mut self) {
        let edit_menu_selection = self.edit_menu.state.selected();
        let is_playing = if self.get_is_running() { "yes" } else { "no" };
        let edit_menu_vec = vec![
            "playing: ".to_owned() + is_playing,
            "bpm: ".to_owned() + &self.get_bpm().to_string(),
            "volume: ".to_owned() + &self.get_volume().to_string(),
            "Time signature: ".to_owned() + &self.get_time_sig_string(),
            "Bar count: ".to_owned() + &self.get_bar_count_string(),
            "Back to main menu".to_owned(),
        ];
        self.edit_menu.set_items(edit_menu_vec);
        if let Some(..) = edit_menu_selection {
            self.edit_menu.select(edit_menu_selection.unwrap());
        }
    }
    
}

// Tests ---------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SETTINGS: InitMetronomeSettings = InitMetronomeSettings {
        bpm: 120,
        ms_delay: 500,
        ts_note: 4,
        ts_value: 4,
        volume: 100.0,
        is_running: false,
    };

    // helper functions should return their values
    #[test]
    fn app_get_bpm() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.get_bpm(), 120);
    }

    #[test]
    fn app_get_volume() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.get_volume(), 100.0);
    }

    #[test]
    fn app_get_is_running() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.get_is_running(), false);
    }

    // change functions should change the internal state of app based on edit_string
    #[test]
    fn app_change_bpm_editor() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "200".to_string();
        test_app.change_bpm_editor();
        assert_eq!(test_app.get_bpm(), 200);
    }

    // app::change_bpm should not change bpm with invalid input
    #[test]
    fn app_change_bpm_bad_input() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "hey this isn't a number is it?".to_string();
        assert_eq!(test_app.change_bpm_editor(), false);
        assert_eq!(test_app.get_bpm(), 120);
    }

    #[test]
    fn app_change_bpm_value_too_big() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "500000".to_string();
        assert_eq!(test_app.change_bpm_editor(), false);
        assert_eq!(test_app.get_bpm(), 120);
    }

    #[test]
    fn app_change_bpm_value_too_small() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "19".to_string();
        assert_eq!(test_app.change_bpm_editor(), false);
        assert_eq!(test_app.get_bpm(), 120);
    }

    #[test]
    fn app_change_bpm_value_negative() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "-120".to_string();
        assert_eq!(test_app.change_bpm_editor(), false);
        assert_eq!(test_app.get_bpm(), 120);
    }

    #[test]
    fn app_change_bpm_value_is_float() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "120.5".to_string();
        assert_eq!(test_app.change_bpm_editor(), false);
        assert_eq!(test_app.get_bpm(), 120);
    }

    // app::change_volume should not change volume with bad input
    #[test]
    fn app_change_volume_editor_bad_input() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "hey this isn't a number is it?".to_string();
        assert_eq!(test_app.change_volume_editor(), false);
        assert_eq!(test_app.get_volume(), 100.0);
    }

    #[test]
    fn app_change_volume_editor_value_too_big() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "500000".to_string();
        assert_eq!(test_app.change_volume_editor(), false);
        assert_eq!(test_app.get_volume(), 100.0);
    }

    #[test]
    fn app_change_volume_editor_value_too_small() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "0".to_string();
        assert_eq!(test_app.change_volume_editor(), false);
        assert_eq!(test_app.get_volume(), 100.0);
    }

    #[test]
    fn app_change_volume_editor_value_negative() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "-120".to_string();
        assert_eq!(test_app.change_volume_editor(), false);
        assert_eq!(test_app.get_volume(), 100.0);
    }

    // app::toggle_metronome should toggle metronome
    #[test]
    fn app_toggle_metronome() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.get_is_running(), false);
        test_app.toggle_metronome();
        assert_eq!(test_app.get_is_running(), true);
        test_app.toggle_metronome();
        assert_eq!(test_app.get_is_running(), false);
    }

    // app::get_ms_from_bpm should correctly calculate the millisecond offset from bpm
    #[test]
    fn app_get_ms_from_bpm() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.get_ms_from_bpm(120), 500);
    }

    // app::clear_strings should clear it's edit and notification strings when told to
    #[test]
    fn app_clear_strings() {
        let mut test_app = App::new(TEST_SETTINGS);
        test_app.edit_string = "Don't forget a towel!".to_string();
        test_app.alert_string = "I mean it, don't forget a towel!".to_string();

        assert!(!test_app.edit_string.is_empty());
        assert!(!test_app.alert_string.is_empty());

        test_app.clear_strings();

        assert!(test_app.edit_string.is_empty());
        assert!(test_app.alert_string.is_empty());
    }

    // app::verify_bpm should correctly determine which values are in range
    #[test]
    fn app_verify_bpm() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.verify_bpm(19), false);
        assert_eq!(test_app.verify_bpm(501), false);
        assert_eq!(test_app.verify_bpm(120), true);
        assert_eq!(test_app.verify_bpm(500), true);
        assert_eq!(test_app.verify_bpm(20), true);
    }

    // app::verify_volume should correctly determine which values are in range
    #[test]
    fn app_verify_volume() {
        let mut test_app = App::new(TEST_SETTINGS);
        assert_eq!(test_app.verify_volume(0.0), false);
        assert_eq!(test_app.verify_volume(201.0), false);
        assert_eq!(test_app.verify_volume(120.0), true);
        assert_eq!(test_app.verify_volume(200.0), true);
        assert_eq!(test_app.verify_volume(1.0), true);
    }
}

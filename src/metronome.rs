/// This file houses the Metronome code which has the audio event loop for running the click
/// It is started on a new thread by App and also shares state with it via Arc variables
use atomic_float::AtomicF64;
use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamHandle};
use std::error;
use std::{
    io,
    fs::File,
    thread,
    time::{Duration, Instant},
    sync::{
        Arc,
        atomic::{
            AtomicBool, AtomicU64, AtomicUsize, Ordering
        },
        mpsc,
    }
    
};
use color_eyre::{eyre::eyre, Report, Result};

pub struct Metronome {
    pub settings: MetronomeSettings,
}

// These settings are also shared with an instance of App to update the metronome after it has been
// moved to a new thread
#[derive(Clone)]
pub struct MetronomeSettings {
    pub bpm: Arc<AtomicU64>,
    pub ms_delay: Arc<AtomicU64>,
    pub ts_note: Arc<AtomicU64>,
    pub ts_value: Arc<AtomicU64>,
    pub volume: Arc<AtomicF64>,
    pub is_running: Arc<AtomicBool>,
    pub bar_count: Arc<AtomicU64>,
    pub current_beat_count: Arc<AtomicU64>,
    pub error: Arc<AtomicBool>,
    pub selected_sound: Arc<AtomicUsize>,
    pub sound_list: Vec<String>,
}

#[derive(Clone, Copy)]
pub struct InitMetronomeSettings {
    pub bpm: u64,
    pub ms_delay: u64,
    pub ts_note: u64,
    pub ts_value: u64,
    pub volume: f64,
    pub is_running: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum MetronomeEvent {
    Tick,
    TickCompleted,
}

impl Metronome {
    pub fn new(new_settings: &MetronomeSettings) -> Metronome {
        Metronome {
            settings: MetronomeSettings {
                bpm: Arc::clone(&new_settings.bpm),
                ms_delay: Arc::clone(&new_settings.ms_delay),
                ts_note: Arc::clone(&new_settings.ts_note),
                ts_value: Arc::clone(&new_settings.ts_value),
                volume: Arc::clone(&new_settings.volume),
                is_running: Arc::clone(&new_settings.is_running),
                bar_count: Arc::clone(&new_settings.bar_count),
                current_beat_count: Arc::clone(&new_settings.current_beat_count),
                error: Arc::clone(&new_settings.error),
                selected_sound: Arc::clone(&new_settings.selected_sound),
                sound_list: new_settings.sound_list.clone(),
            },
        }
    }

    pub fn start(&mut self, tick_rate :u64) {
        let tick_rate = Duration::from_millis(tick_rate);
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut running = self.settings.is_running.load(Ordering::Relaxed);
        let (sender, receiver) = mpsc::channel();
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(tick_rate);
            spin_sleep::sleep(timeout);

            if running {
                // Exit the loop if there was an error
                if self.settings.error.load(Ordering::Relaxed) {
                    break;
                }
                // Check to see if we have completed a tick and prepare another one if so
                match receiver.recv() {
                    Ok(typey) => {
                        match typey {
                            MetronomeEvent::TickCompleted => {
                                // Load the tick function into a new thread for execution (that way this isn't tied to bpm anymore)
                                let sender = sender.clone();
                                let handle_clone = stream_handle.clone();
                                let settings_clone = self.settings.clone();
                                let error_clone = self.settings.error.clone();
                                thread::spawn(move || {
                                    match tick(handle_clone, settings_clone, sender) {
                                        Ok(_) => {}
                                        Err(_) => {error_clone.swap(true, Ordering::Relaxed);} 
                                    }
                                });
                            }
                            _ => {}
                        }

                    }
                    _ => {}
                }               
            }
            running = self.settings.is_running.load(Ordering::Relaxed);
            if !running {
                self.settings.bar_count.swap(0, Ordering::Relaxed);
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    // I am leaving this here as it might be useful in the future, though it is currently dead code
    #[allow(dead_code)]
    pub fn update_settings(
        &self,
        bpm: u64,
        ms_delay: u64,
        volume: f64,
        is_running: bool,
        error: bool,
    ) {
        self.settings.bpm.swap(bpm, Ordering::Relaxed);
        self.settings.ms_delay.swap(ms_delay, Ordering::Relaxed);
        self.settings.volume.swap(volume, Ordering::Relaxed);
        self.settings.is_running.swap(is_running, Ordering::Relaxed);
        self.settings.error.swap(error, Ordering::Relaxed);
    }
}

fn tick(stream_handle: OutputStreamHandle, settings_clone :MetronomeSettings, sender :mpsc::Sender<MetronomeEvent>) -> Result<(), Report> {
    // TODO: Don't load the sample every time, if possible load once and replay.
    // TODO: add functionality for loading different samples, possibly with atomic string crate
    let selected_sound_name = settings_clone.sound_list
            [settings_clone.selected_sound.load(Ordering::Relaxed)]
            .clone();
    let file = io::BufReader::new(
        match File::open("./assets/".to_owned() + &selected_sound_name) {
            Ok(value) => value,
            Err(_) => {
                return Err(eyre!("Error: Problem loading sound"))
            }
        },
    );

    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(
        source
            .amplify((settings_clone.volume.load(Ordering::Relaxed) / 100.0) as f32)
            .convert_samples(),
    );

    // Bar/Beat count
    let mut current_beat_count =
        settings_clone.current_beat_count.load(Ordering::Relaxed);
    if current_beat_count == settings_clone.ts_note.load(Ordering::Relaxed) {
        settings_clone.current_beat_count.swap(1, Ordering::Relaxed);
        let new_bar_count = settings_clone.bar_count.load(Ordering::Relaxed) + 1;
        settings_clone
            .bar_count
            .swap(new_bar_count, Ordering::Relaxed);
    } else {
        current_beat_count += 1;
        settings_clone
            .current_beat_count
            .swap(current_beat_count, Ordering::Relaxed);
    }

    // Wait
    spin_sleep::sleep(Duration::from_millis(
        settings_clone.ms_delay.load(Ordering::Relaxed),
    ));
    sender.send(MetronomeEvent::TickCompleted);
    Ok(())
}
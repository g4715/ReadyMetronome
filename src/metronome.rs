/// This file houses the Metronome code which has the audio event loop for running the click
/// It is started on a new thread by App and also shares state with it via Arc variables
use atomic_float::AtomicF64;
use rodio::source::Source;
use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time;

pub struct Metronome {
    pub settings: MetronomeSettings,
}

// These settings are also shared with an instance of App to update the metronome after it has been
// moved to a new thread
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
}

pub struct InitMetronomeSettings {
    pub bpm: u64,
    pub ms_delay: u64,
    pub ts_note: u64,
    pub ts_value: u64,
    pub volume: f64,
    pub is_running: bool,
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
            },
        }
    }

    pub fn start(&mut self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut running = self.settings.is_running.load(Ordering::Relaxed);
        loop {
            if running {
                if self.settings.error.load(Ordering::Relaxed) {
                    break;
                }
                // TODO: Don't load the sample every time, if possible load once and replay.
                // TODO: add functionality for loading different samples, possibly with atomic string crate
                // TODO: Don't tie the refresh rate of this to the metronome clock speed, make it independent if possible
                let file = io::BufReader::new(match File::open("./assets/EmeryBoardClick.wav") {
                    Ok(value) => value,
                    Err(_) => {
                        self.settings.error.swap(true, Ordering::Relaxed);
                        break;
                    }
                });

                let source = Decoder::new(file).unwrap();
                let _ = stream_handle.play_raw(
                    source
                        .amplify((self.settings.volume.load(Ordering::Relaxed) / 100.0) as f32)
                        .convert_samples(),
                );

                // Bar count
                let mut current_beat_count = self.settings.current_beat_count.load(Ordering::Relaxed);
                if current_beat_count + 1 == self.settings.ts_note.load(Ordering::Relaxed) {
                    self.settings.current_beat_count.swap(0, Ordering::Relaxed);
                    let new_bar_count = self.settings.bar_count.load(Ordering::Relaxed) + 1;
                    self.settings.bar_count.swap(new_bar_count, Ordering::Relaxed);
                } else {
                    current_beat_count += 1;
                    self.settings.current_beat_count.swap(current_beat_count, Ordering::Relaxed);
                }

                // Wait
                spin_sleep::sleep(time::Duration::from_millis(
                    self.settings.ms_delay.load(Ordering::Relaxed),
                ));
            }
            if self.settings.error.load(Ordering::Relaxed) {
                break;
            }
            // TODO: Right now the loop just spins while it waits. Waiting for a signal to start loop would be better
            running = self.settings.is_running.load(Ordering::Relaxed);
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

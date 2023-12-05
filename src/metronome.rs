use atomic_float::AtomicF64;
use rodio::source::Source;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time;

pub struct Metronome {
    pub settings: MetronomeSettings,
}

// #[derive(Clone)]
pub struct MetronomeSettings {
    pub bpm: Arc<AtomicU64>,
    pub ms_delay: Arc<AtomicU64>,
    pub volume: Arc<AtomicF64>,
    pub is_running: Arc<AtomicBool>,
}

impl Metronome {
    pub fn new(new_settings: &MetronomeSettings) -> Metronome {
        Metronome {
            settings: MetronomeSettings {
                bpm: Arc::clone(&new_settings.bpm),
                ms_delay: Arc::clone(&new_settings.ms_delay),
                volume: Arc::clone(&new_settings.volume),
                is_running: Arc::clone(&new_settings.is_running),
            },
        }
    }

    pub fn start(&mut self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut running = self.settings.is_running.load(Ordering::Relaxed);
        loop {
            if running {
                // TODO: Don't load the sample every time, if possible load once and replay.
                // Need to add functionality for loading different samples and handling errors.
                let file =
                    io::BufReader::new(File::open("./src/assets/EmeryBoardClick.wav").unwrap());
                let source = Decoder::new(file).unwrap();
                let _ = stream_handle.play_raw(
                    source
                        .amplify((self.settings.volume.load(Ordering::Relaxed) / 100.0) as f32)
                        .convert_samples(),
                );
                spin_sleep::sleep(time::Duration::from_millis(
                    self.settings.ms_delay.load(Ordering::Relaxed),
                ));
            }
            // TODO: Right now the loop just spins while it waits. Waiting for a signal to start loop would be better
            running = self.settings.is_running.load(Ordering::Relaxed);
        }
    }

    pub fn update_settings(&self, bpm: u64, volume: f64, is_running: bool) {
        self.settings.bpm.swap(bpm, Ordering::Relaxed);
        self.settings.volume.swap(volume, Ordering::Relaxed);
        self.settings.is_running.swap(is_running, Ordering::Relaxed);
    }
}

use atomic_float::AtomicF64;
use rodio::source::Source;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::{thread, time};

pub struct Metronome {
    pub settings: MetronomeSettings,
}

// #[derive(Clone)]
pub struct MetronomeSettings {
    pub bpm: Arc<AtomicU64>,
    pub volume: Arc<AtomicF64>,
    pub is_running: Arc<AtomicBool>,
}

impl Metronome {
    pub fn new(new_settings: &MetronomeSettings) -> Metronome {
        Metronome {
            settings: MetronomeSettings {
                bpm: Arc::clone(&new_settings.bpm),
                volume: Arc::clone(&new_settings.volume),
                is_running: Arc::clone(&new_settings.is_running),
            },
        }
    }

    pub fn start(&mut self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut running = true;
        loop {
            if running {
                // TODO: Don't load the sample every time, if possible load once and replay. Convert to Sink
                let file =
                    io::BufReader::new(File::open("./src/assets/EmeryBoardClick.wav").unwrap());
                let source = Decoder::new(file).unwrap();
                let _ = stream_handle.play_raw(source.convert_samples());
                spin_sleep::sleep(time::Duration::from_millis(
                    self.settings.bpm.load(Ordering::Relaxed),
                ));
            }
            running = self.settings.is_running.load(Ordering::Relaxed);
        }
    }

    // pub fn get_settings(&self) -> MetronomeSettings {
    //     self.settings.clone()
    // }

    pub fn update_settings(&self, bpm: u64, volume: f64, is_running: bool) {
        self.settings.bpm.swap(bpm, Ordering::Relaxed);
        self.settings.volume.swap(volume, Ordering::Relaxed);
        self.settings.is_running.swap(is_running, Ordering::Relaxed);
    }
}

// TODO

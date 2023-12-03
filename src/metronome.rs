use std::{thread, time};
use std::io;
use std::fs::File;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;

// let mut program_running = true;
// let default_bpm = get_ms_from_bpm(120);

// // Set up Atomics for metronome thread
// let bpm = Arc::new(AtomicU64::new(default_bpm));
// let bpm_clone = Arc::clone(&bpm);
// let metronome_running = Arc::new(AtomicBool::new(false));
// let metronome_running_clone = Arc::clone(&metronome_running);

// let metronome_thread = thread::spawn(move || {
//     let mut running = false;
//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//     loop {
//         if running {
//             // TODO: Don't load the sample every time, if possible load once and replay. Convert to Sink
//             let file = io::BufReader::new(File::open("./src/assets/EmeryBoardClick.wav").unwrap());
//             let source = Decoder::new(file).unwrap();
//             let _ = stream_handle.play_raw(source.convert_samples());
//             spin_sleep::sleep(time::Duration::from_millis(bpm_clone.load(Ordering::Relaxed)));
//         }
//         running = metronome_running_clone.load(Ordering::Relaxed);
//     }
// });

struct Metronome {
    settings :MetronomeSettings,
}

#[derive(Clone)]
struct MetronomeSettings {
    bpm: Arc<AtomicU64>,
    volume: Arc<AtomicU64>,
    is_running: Arc<AtomicBool>,
}

impl Metronome {
    pub fn new() -> Metronome {
        Metronome {
            settings = MetronomeSettings{
                bpm: Arc::new(AtomicU64::new(get_ms_from_bpm(120))),
                volume: Arc::new(Atomicf64::new(1.0)),
                is_running: Arc::new(AtomicBool::new(false)),
            },
        }
    }

    pub fn get_settings(&self) -> MetronomeSettings {
        self.settings.clone()
    }

    pub fn update_settings(&self, new_settings :MetronomeSettings) {
        self.settings = new_settings;
    }

    // Convert a bpm value to the millisecond delay
    pub fn get_ms_from_bpm(&self, bpm :u64) -> u64 {
        let result :u64 = (60_000.0_f64 / bpm as f64).round() as u64;
        result
    }
}


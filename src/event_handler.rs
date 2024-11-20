// The event handler is taken pretty much wholesale from this example: https://ratatui.rs/tutorials/counter-app/multiple-files/event/
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

/// Terminal events
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,              // Terminal tick.
    Key(KeyEvent),     // Key press.
    Mouse(MouseEvent), // Mouse click/scroll.
    Resize(u16, u16),  // Terminal resize.
    FocusChange(bool), // terminal focus gained / lost
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>, // Event sender channel.
    receiver: mpsc::Receiver<Event>, // Event receiver channel.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>, // Event handler thread.
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(new_tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(new_tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            CrosstermEvent::FocusGained => sender.send(Event::FocusChange(true)),
                            CrosstermEvent::FocusLost => sender.send(Event::FocusChange(false)),
                            CrosstermEvent::Paste(_data) => Ok(()),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }
    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}

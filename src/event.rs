use std::{sync::mpsc, thread, time::Duration};

use crossterm::event;

pub enum TerminalEvent {
    Input(Key),
    Tick,
}

/// Simple event handler wrapping cross-term input and tick events. Each event
/// is handled on its own thread in an attempt to prevent any kind of interface "hangups".
pub struct TerminalEvents {
    rx: mpsc::Receiver<TerminalEvent>,

    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<TerminalEvent>,
}

impl TerminalEvents {
    pub fn listen() -> Self {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        let tick_rate = Duration::from_millis(250);
        thread::spawn(move || {
            loop {
                if event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);
                        event_tx.send(TerminalEvent::Input(key)).unwrap();
                    }
                }

                event_tx.send(TerminalEvent::Tick).unwrap();
            }
        });

        TerminalEvents { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<TerminalEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    Enter,
    Tab,
    Backspace,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl Key {
    pub fn from_function_key(n: u8) -> Key {
        match n {
            0 => Key::F0,
            1 => Key::F1,
            2 => Key::F2,
            3 => Key::F3,
            4 => Key::F4,
            5 => Key::F5,
            6 => Key::F6,
            7 => Key::F7,
            8 => Key::F8,
            9 => Key::F9,
            10 => Key::F10,
            11 => Key::F11,
            12 => Key::F12,
            _ => panic!("unknown function key: F{}", n),
        }
    }
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Escape,

            event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => Key::Backspace,

            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,

            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,

            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,

            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,

            event::KeyEvent {
                code: event::KeyCode::Home,
                ..
            } => Key::Home,

            event::KeyEvent {
                code: event::KeyCode::End,
                ..
            } => Key::End,

            event::KeyEvent {
                code: event::KeyCode::PageUp,
                ..
            } => Key::PageUp,

            event::KeyEvent {
                code: event::KeyCode::PageDown,
                ..
            } => Key::PageDown,

            event::KeyEvent {
                code: event::KeyCode::Delete,
                ..
            } => Key::Delete,

            event::KeyEvent {
                code: event::KeyCode::Insert,
                ..
            } => Key::Insert,

            event::KeyEvent {
                code: event::KeyCode::F(n),
                ..
            } => Key::from_function_key(n),

            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,

            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => Key::Tab,

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::ALT,
                ..
            } => Key::Alt(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),

            _ => Key::Unknown,
        }
    }
}

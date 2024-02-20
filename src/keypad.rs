extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};

use sdl2::EventPump;
use sdl2::Sdl;

use crate::keymaps::Keymap;

pub struct Keypad {
    pump: EventPump,
    keymap: Vec<Keymap>,
}

pub enum State {
    Exit,
    Continue,
    Increase,
    Decrease,
    Debug,
    Reset,
    PauseToggle,
    Pause,
    Unpause,
}

impl Keypad {
    pub fn new(sdl_context: &Sdl, keymap: Vec<Keymap>) -> Self {
        Keypad {
            pump: sdl_context.event_pump().unwrap(),
            keymap,
        }
    }

    pub fn key_press(&mut self, key: &mut [u8; 16]) -> State {
        for event in self.pump.poll_iter() {
            return match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => State::Exit,
                Event::KeyDown {
                    keycode: Some(Keycode::Equals),
                    ..
                } => State::Increase,
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => State::Decrease,
                Event::KeyDown {
                    keycode: Some(Keycode::F12),
                    ..
                } => State::Debug,
                Event::KeyDown {
                    keycode: Some(Keycode::F3),
                    ..
                } => State::Reset,
                Event::KeyDown {
                    keycode: Some(Keycode::F8),
                    ..
                } => State::PauseToggle,
                Event::AppWillEnterBackground { .. } => State::Pause,
                Event::AppWillEnterForeground { .. } => State::Unpause,
                _ => State::Continue,
            };
        }

        let key_state = KeyboardState::new(&mut self.pump);

        for keymap in <Vec<Keymap> as Clone>::clone(&self.keymap).into_iter() {
            key[keymap.key as usize] = key_state.is_scancode_pressed(
                Scancode::from_i32(keymap.scancode)
                    .expect(format!("Invalid scancode: {}", keymap.scancode).as_str()),
            ) as u8;
        }

        State::Continue
    }
}

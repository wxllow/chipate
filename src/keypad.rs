extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};

use sdl2::EventPump;
use sdl2::Sdl;

pub struct Keypad {
    pump: EventPump,
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
    pub fn new(sdl_context: &Sdl) -> Self {
        Keypad {
            pump: sdl_context.event_pump().unwrap(),
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
        key[0x1] = key_state.is_scancode_pressed(Scancode::Num1) as u8;
        key[0x2] = key_state.is_scancode_pressed(Scancode::Num2) as u8;
        key[0x3] = key_state.is_scancode_pressed(Scancode::Num3) as u8;
        key[0xC] = key_state.is_scancode_pressed(Scancode::Num4) as u8;

        key[0x4] = key_state.is_scancode_pressed(Scancode::Q) as u8;
        key[0x5] = key_state.is_scancode_pressed(Scancode::W) as u8;
        key[0x6] = key_state.is_scancode_pressed(Scancode::E) as u8;
        key[0xD] = key_state.is_scancode_pressed(Scancode::R) as u8;

        key[0x7] = key_state.is_scancode_pressed(Scancode::A) as u8;
        key[0x8] = key_state.is_scancode_pressed(Scancode::S) as u8;
        key[0x9] = key_state.is_scancode_pressed(Scancode::D) as u8;
        key[0xE] = key_state.is_scancode_pressed(Scancode::F) as u8;

        key[0xA] = key_state.is_scancode_pressed(Scancode::Z) as u8;
        key[0x0] = key_state.is_scancode_pressed(Scancode::X) as u8;
        key[0xB] = key_state.is_scancode_pressed(Scancode::C) as u8;
        key[0xF] = key_state.is_scancode_pressed(Scancode::V) as u8;

        State::Continue
    }
}

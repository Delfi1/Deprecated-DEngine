use std::{time::Instant, default};

use winit::event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState, };

pub struct Key {
    last_press: Instant,
    culldown: f32,
    virtual_keycode: VirtualKeyCode
}

impl Key {
    pub fn new(culldown: f32, virtual_keycode: VirtualKeyCode) -> &'static mut Self {
        let last_press = Instant::now();
        
        Box::leak(Box::new(Self {last_press, culldown, virtual_keycode}))
    }

    pub fn is_pressed(&mut self, event: &Event<'_, ()>) -> bool {
        if Instant::now().duration_since(self.last_press).as_secs_f32() < self.culldown {
            return false;
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {virtual_keycode, state, ..},
                    ..
                },
                ..
            } => {
                if *virtual_keycode == Some(self.virtual_keycode.clone()) && *state == ElementState::Pressed  {
                    self.last_press = Instant::now();
                    return true;
                }
            },
            _ => ()
        }
        return false;
    }
}

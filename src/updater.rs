use std::{thread, time::Duration};

use winit;

const CD: u64 = 1000; // Millis

pub fn check_updates(window: &winit::window::Window) {
    loop {
        
        
        thread::sleep(Duration::from_millis(CD));
    }
}
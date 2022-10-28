//! # Example: Run Snake on a 8x8 RGB display, like a LED matrix
//!

use embedded_graphics::{
    pixelcolor::{Rgb888},
    prelude::*,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};
use rand::rngs::ThreadRng;

use snake::*;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(8, 8));

    let output_settings = OutputSettingsBuilder::new().scale(30).build();
    let mut window = Window::new("Progress", &output_settings);
    let mut game = SnakeGame::<20, Rgb888, ThreadRng>::new(8, 8, rand::thread_rng(), Rgb888::RED, Rgb888::YELLOW, 10);
    window.update(&display);
    'running: loop {
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => {
                    let direction = match keycode {
                        Keycode::Left => Direction::Left,
                        Keycode::Right => Direction::Right,
                        Keycode::Up => Direction::Up,
                        Keycode::Down => Direction::Down,
                        _ => Direction::None,
                    };
                    game.set_direction(direction);
                }
                _ => {}
            }
        }
        display.clear(Rgb888::BLACK)?;
        game.draw(&mut display);
        window.update(&display);
        thread::sleep(Duration::from_millis(300));
    }
}

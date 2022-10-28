//! # Example: Run Snake on a 8x8 RGB display, like a LED matrix
//!

use embedded_graphics::{
    pixelcolor::{BinaryColor},
    prelude::*,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};
use rand::rngs::ThreadRng;

use snake::*;

fn main() -> Result<(), std::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    let output_settings = OutputSettingsBuilder::new().scale(5).theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue).build();
    let mut window = Window::new("Progress", &output_settings);
    let mut game = SnakeGame::<BinaryColor, 100, ThreadRng>::new(
        128, 64,
        rand::thread_rng(),
        BinaryColor::On,
        BinaryColor::On,
        50);
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
        display.clear(BinaryColor::Off)?;
        game.draw(&mut display);
        window.update(&display);
        thread::sleep(Duration::from_millis(50));
    }
}

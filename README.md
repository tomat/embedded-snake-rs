# embedded-snake-rs
Snake game implementation in Rust with no-std. It uses embedded-graphics as a display target.

It might be a useful demo application for embedded projects with any display (supported by [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/)) and at least four buttons.

See the examples on computer, using the embedded graphics simulator.

Note this is a work-in-progress project.

## Existing features
* Works with arbitrary displays (color scheme and resolution are constructor params)
* Custom colors for the snake and the food
* Custom food timeout
* Custom maximum snake length
* Custom scale (positive integers) for extra-small (or dense) displays

## TODO
* Any sort of game-over condition, such as:
  * Walls (outer or inner)
  * Snake hits itself

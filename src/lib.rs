use embedded_graphics::{
    pixelcolor::*,
    prelude::{DrawTarget, OriginDimensions, Point, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable, Pixel,
};
use serde::{Deserialize, Serialize};

pub struct Snake<T: PixelColor, const MAX_SIZE: usize> {
    pub color: T,
    parts: [Pixel<T>; MAX_SIZE],
    pub len: usize,
    pub direction: Direction,
    size_x: u8,
    size_y: u8,
}

pub struct SnakeIntoIterator<'a, T: PixelColor, const MAX_SIZE: usize> {
    snake: &'a Snake<T, MAX_SIZE>,
    index: usize,
}

impl<'a, T: PixelColor, const MAX_SIZE: usize> IntoIterator for &'a Snake<T, MAX_SIZE> {
    type Item = Pixel<T>;
    type IntoIter = SnakeIntoIterator<'a, T, MAX_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        SnakeIntoIterator {
            snake: self,
            index: 0,
        }
    }
}

impl<'a, T: PixelColor, const MAX_SIZE: usize> Iterator for SnakeIntoIterator<'a, T, MAX_SIZE> {
    type Item = Pixel<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.snake.parts[self.index];
        if self.index < self.snake.len {
            self.index += 1;
            return Some(cur);
        }
        None
    }
}

impl<T: PixelColor, const MAX_SIZE: usize> Snake<T, MAX_SIZE> {
    fn new(color: T, size_x: u8, size_y: u8, start_at: Point, starting_direction: Direction) -> Snake<T, MAX_SIZE> {
        Snake {
            color,
            parts: [Pixel::<T>(start_at, color); MAX_SIZE],
            len: 5,
            direction: starting_direction,
            size_x,
            size_y,
        }
    }
    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    fn contains(&self, this: Point) -> bool {
        for part in self.into_iter() {
            if part.0 == this {
                return true;
            };
        }
        false
    }
    fn grow(&mut self) {
        if self.len < MAX_SIZE - 1 {
            self.len += 1;
        }
    }
    fn make_step(&mut self) {
        let mut i = self.len;
        while i > 0 {
            self.parts[i] = self.parts[i - 1];
            i -= 1;
        }
        match self.direction {
            Direction::Left => {
                if self.parts[0].0.x == 0 {
                    self.parts[0].0.x = (self.size_x - 1) as i32;
                } else {
                    self.parts[0].0.x -= 1;
                }
            }
            Direction::Right => {
                if self.parts[0].0.x == (self.size_x - 1) as i32 {
                    self.parts[0].0.x = 0;
                } else {
                    self.parts[0].0.x += 1;
                }
            }
            Direction::Up => {
                if self.parts[0].0.y == 0 {
                    self.parts[0].0.y = (self.size_y - 1) as i32;
                } else {
                    self.parts[0].0.y -= 1;
                }
            }
            Direction::Down => {
                if self.parts[0].0.y == (self.size_y - 1) as i32 {
                    self.parts[0].0.y = 0;
                } else {
                    self.parts[0].0.y += 1;
                }
            }
            Direction::None => {}
        }
    }
}

struct Food<T: PixelColor, RNG: rand_core::RngCore> {
    size_x: u8,
    size_y: u8,
    place: Pixel<T>,
    rng: RNG,
}

impl<T: PixelColor, RNG: rand_core::RngCore> Food<T, RNG> {
    pub fn new(color: T, rand_source: RNG, size_x: u8, size_y: u8) -> Self {
        Food {
            size_x,
            size_y,
            place: Pixel(Point { x: 0, y: 0 }, color),
            rng: rand_source,
        }
    }
    fn replace<'a, const MAX_SIZE: usize>(&mut self, iter_source: &[Snake<T, MAX_SIZE>; 3]) {
        let mut p: Point;
        'outer: loop {
            let random_number = self.rng.next_u32();

            p = Point {
                x: ((random_number >> 24) as u8 % self.size_x).into(),
                y: ((random_number >> 16) as u8 % self.size_y).into(),
            };
            let snakes = iter_source.into_iter();
            for s in snakes {
                let blocked_positions = s.into_iter();
                for blocked_position in blocked_positions {
                    if p == blocked_position.0 {
                        continue 'outer;
                    }
                }
            }
            break;
        }
        self.place = Pixel::<T> {
            0: p,
            1: self.place.1,
        }
    }
    fn get_pixel(&self) -> Pixel<T> {
        self.place
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameStatus {
    Continue,
    End,
}

pub struct SnakeGame<const MAX_SNAKE_SIZE: usize, T: PixelColor, RNG: rand_core::RngCore> {
    pub snakes: [Snake<T, MAX_SNAKE_SIZE>; 3],
    food: Food<T, RNG>,
    food_age: u8,
    food_lifetime: u8,
    size_x: u8,
    size_y: u8,
    scale_x: u8,
    scale_y: u8,
    pub player_count: usize,
}

impl<const MAX_SIZE: usize, T: PixelColor, RNG: rand_core::RngCore> SnakeGame<MAX_SIZE, T, RNG> {
    pub fn new(
        size_x: u8,
        size_y: u8,
        scale_x: u8,
        scale_y: u8,
        rand_source: RNG,
        snake_colors: [T; 3],
        food_color: T,
        food_lifetime: u8,
        player_count: usize,
    ) -> Self {
        let snakes = [
            Snake::<T, MAX_SIZE>::new(snake_colors[0], size_x / scale_x, size_y / scale_y, Point{ x: 0, y: 0 }, Direction::Right),
            Snake::<T, MAX_SIZE>::new(snake_colors[1], size_x / scale_x, size_y / scale_y, Point{ x: 0, y: 7 }, Direction::Right),
            Snake::<T, MAX_SIZE>::new(snake_colors[2], size_x / scale_x, size_y / scale_y, Point{ x: 31, y: 0 }, Direction::Left),
        ];

        let mut food = Food::<T, RNG>::new(food_color, rand_source, size_x / scale_x, size_y / scale_y);
        food.replace(&snakes);

        SnakeGame {
            snakes,
            food,
            food_age: 0,
            food_lifetime,
            size_x,
            size_y,
            scale_x,
            scale_y,
            player_count,
        }
    }
    pub fn set_direction(&mut self, player: usize, direction: Direction) {
        self.snakes[player].set_direction(direction);
    }
    pub fn draw<D>(&mut self, target: &mut D) -> GameStatus
    where
        D: DrawTarget<Color = T>,
    {
        let mut hit = false;
        for snake in self.snakes.iter_mut().take(self.player_count) {
            snake.make_step();

            let snake_parts = &snake.parts[1..snake.len];
            for (i, s) in snake_parts.iter().enumerate() {
                if s.0 == snake.parts[0].0 {
                    println!("{i}: {:?} == {:?}", s.0, snake.parts[0].0);

                    return GameStatus::End;
                }
            }

            if !hit {
                hit = snake.contains(self.food.get_pixel().0);
                if hit {
                    snake.grow();
                }
            }
        }
        self.food_age += 1;
        if self.food_age >= self.food_lifetime || hit {
            self.food.replace(&self.snakes);
            self.food_age = 0;
        }

        let mut scaled_display = ScaledDisplay::<D> {
            real_display: target,
            size_x: self.size_x / self.scale_x,
            size_y: self.size_y / self.scale_y,
            scale_x: self.scale_x,
            scale_y: self.scale_y,
        };

        for snake in self.snakes.iter().take(self.player_count) {
            for part in snake.into_iter() {
                _ = part.draw(&mut scaled_display);
            }
        }
        _ = self.food.get_pixel().draw(&mut scaled_display);

        GameStatus::Continue
    }
}

/// A dummy DrawTarget implementation that can magnify each pixel so the user code does not need to adapt for scaling things
struct ScaledDisplay<'a, T: DrawTarget> {
    real_display: &'a mut T,
    size_x: u8,
    size_y: u8,
    scale_x: u8,
    scale_y: u8,
}

impl<'a, T: DrawTarget> DrawTarget for ScaledDisplay<'a, T> {
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            let style = PrimitiveStyle::with_fill(pixel.1);
            Rectangle::new(
                Point::new(
                    pixel.0.x * self.scale_x as i32,
                    pixel.0.y * self.scale_y as i32,
                ),
                Size::new(self.scale_x as u32, self.scale_y as u32),
            )
            .into_styled(style)
            .draw(self.real_display)?;
        }
        Ok(())
    }
}

impl<'a, T: DrawTarget> OriginDimensions for ScaledDisplay<'a, T> {
    fn size(&self) -> Size {
        Size::new(self.size_x as u32, self.size_y as u32)
    }
}

#[cfg(test)]
mod tests {

    use crate::Snake;
    use embedded_graphics::pixelcolor::*;
    use embedded_graphics::prelude::*;

    #[test]
    fn snake_basic() {
        let mut snake = Snake::<Rgb888, 20>::new(Rgb888::RED, 8, 8);
        snake.set_direction(crate::Direction::Right);
        assert_eq!(
            Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED),
            snake.into_iter().next().unwrap()
        );
        snake.make_step();
        assert_eq!(
            Pixel::<Rgb888>(Point { x: 1, y: 0 }, Rgb888::RED),
            snake.into_iter().nth(0).unwrap()
        );
        assert_eq!(
            Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED),
            snake.into_iter().nth(1).unwrap()
        );
        snake.set_direction(crate::Direction::Down);
        snake.make_step();
        assert_eq!(
            Pixel::<Rgb888>(Point { x: 1, y: 1 }, Rgb888::RED),
            snake.into_iter().nth(0).unwrap()
        );
        assert_eq!(
            Pixel::<Rgb888>(Point { x: 1, y: 0 }, Rgb888::RED),
            snake.into_iter().nth(1).unwrap()
        );
        assert_eq!(
            Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED),
            snake.into_iter().nth(2).unwrap()
        );
        assert_eq!(true, snake.contains(Point { x: 0, y: 0 }));
        assert_eq!(true, snake.contains(Point { x: 1, y: 0 }));
        assert_eq!(true, snake.contains(Point { x: 1, y: 1 }));
    }
}

#![no_std]

use embedded_graphics::{
    pixelcolor::{Rgb888, self},
    prelude::{DrawTarget, Point, RgbColor},
    Drawable, Pixel,
};

struct Snake<const MAX_SIZE: usize> {
    parts: [Pixel<Rgb888>; MAX_SIZE],
    len: usize,
    direction: Direction,
    size_x: u8,
    size_y: u8
}

struct SnakeIntoIterator<'a, const MAX_SIZE: usize> {
    snake: &'a Snake<MAX_SIZE>,
    index: usize,
}

impl<'a, const MAX_SIZE: usize> IntoIterator for &'a Snake<MAX_SIZE> {
    type Item = Pixel<Rgb888>;
    type IntoIter = SnakeIntoIterator<'a, MAX_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        SnakeIntoIterator {
            snake: self,
            index: 0,
        }
    }
}


impl<'a, const MAX_SIZE: usize> Iterator for SnakeIntoIterator<'a, MAX_SIZE> {
    type Item = Pixel<Rgb888>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.snake.parts[self.index];
        if self.index < self.snake.len {
            self.index += 1;
            return Some(cur);
        }
        None
    }
}

impl<const MAX_SIZE: usize> Snake<MAX_SIZE> {
    fn new() -> Snake<MAX_SIZE> {
        Snake { parts: [Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED); MAX_SIZE], len: 5, direction: Direction::None, size_x: 8, size_y: 8 }
    }
    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    fn contains(&self, this: Point) -> bool {
        for part in self.into_iter() {
            if part.0 == this {
                return true
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
            self.parts[i] = self.parts[i-1];
            i -= 1;
        }
        match self.direction {
            Direction::Left => {
                if self.parts[0].0.x == 0 {
                    self.parts[0].0.x = (self.size_x - 1) as i32;
                } else {
                    self.parts[0].0.x -= 1;
                }
            },
            Direction::Right => {
                if self.parts[0].0.x == (self.size_x - 1) as i32 {
                    self.parts[0].0.x = 0;
                } else {
                    self.parts[0].0.x += 1;
                }
            },
            Direction::Up => {
                if self.parts[0].0.y == 0 {
                    self.parts[0].0.y = (self.size_y - 1) as i32;
                } else {
                    self.parts[0].0.y -= 1;
                }
            },
            Direction::Down => {
                if self.parts[0].0.y == (self.size_y - 1) as i32 {
                    self.parts[0].0.y = 0;
                } else {
                    self.parts[0].0.y += 1;
                }
            },
            Direction::None => {},
        }
    }
}

struct Food<RNG: rand_core::RngCore> {
    size_x: u8,
    size_y: u8,
    place: Pixel<Rgb888>,
    rng: RNG
}

impl<RNG: rand_core::RngCore> Food<RNG> {
    pub fn new(color: Rgb888, rand_source: RNG) -> Self {
        Food{ size_x: 8, size_y: 8, place: Pixel(Point { x: 0, y: 0 }, color), rng: rand_source }
    }
    fn replace<'a, const MAX_SIZE: usize>(&mut self, iter_source: &Snake<MAX_SIZE>) {
        let mut p: Point;
        'outer: loop {
            let random_number = self.rng.next_u32();
            let blocked_positions = iter_source.into_iter();
            p = Point{
                x: ((random_number >> 24) as u8 % self.size_x).into(),
                y: ((random_number >> 16) as u8 % self.size_y).into()
            };
            for blocked_position in blocked_positions {
                if p == blocked_position.0 {
                    continue 'outer;
                }
            }
            break
        }
        self.place = Pixel::<Rgb888>{0: p, 1: self.place.1}
    }
    fn get_pixel(&self) -> Pixel<Rgb888> {
        self.place
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

pub struct SnakeGame<const MAX_SIZE: usize, RNG: rand_core::RngCore> {
    snake: Snake<MAX_SIZE>,
    food: Food<RNG>,
    food_age: u8
}

impl<const MAX_SIZE: usize, RNG: rand_core::RngCore> SnakeGame<MAX_SIZE, RNG> {
    pub fn new(rand_source: RNG) -> Self {
        let snake = Snake::<MAX_SIZE>::new();
        let mut food = Food::<RNG>::new(Rgb888::YELLOW, rand_source);
        food.replace(&snake);
        SnakeGame { snake: snake, food, food_age: 0 }
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.snake.set_direction(direction);
    }
    pub fn draw<D>(&mut self, target: &mut D) -> ()
    where
    D: DrawTarget<Color = pixelcolor::Rgb888>,
    {
        self.snake.make_step();
        let hit = self.snake.contains(self.food.get_pixel().0);
        if hit {
            self.snake.grow();
        }
        self.food_age += 1;
        if self.food_age >= 10 || hit {
            self.food.replace(&self.snake);
            self.food_age = 0;
        }
        for part in self.snake.into_iter() {
            _ = part.draw(target);
        }
        _ = self.food.get_pixel().draw(target);
    }
}

#[cfg(test)]
mod tests {

    use crate::Snake;
    use embedded_graphics::prelude::*;
    use embedded_graphics::pixelcolor::*;

   #[test]
    fn it_works() {
        let mut snake = Snake::<20>::new();
        snake.set_direction(crate::Direction::Right);
        assert_eq!(Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED), snake.into_iter().next().unwrap());
        snake.make_step();
        assert_eq!(Pixel::<Rgb888>(Point { x: 1, y: 0 }, Rgb888::RED), snake.into_iter().nth(0).unwrap());
        assert_eq!(Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED), snake.into_iter().nth(1).unwrap());
        snake.set_direction(crate::Direction::Down);
        snake.make_step();
        assert_eq!(Pixel::<Rgb888>(Point { x: 1, y: 1 }, Rgb888::RED), snake.into_iter().nth(0).unwrap());
        assert_eq!(Pixel::<Rgb888>(Point { x: 1, y: 0 }, Rgb888::RED), snake.into_iter().nth(1).unwrap());
        assert_eq!(Pixel::<Rgb888>(Point { x: 0, y: 0 }, Rgb888::RED), snake.into_iter().nth(2).unwrap());
        //assert_eq!(Pixel::<Rgb888>(Point { x: 0, y: 1 }, Rgb888::RED), parts.next().unwrap());
        //assert_eq!(false, snake.contains(Point{x: 0, y: 0}));
        //assert_eq!(false, snake.contains(Point{x: 0, y: 0}));
        assert_eq!(true, snake.contains(Point{x: 0, y: 0}));
        assert_eq!(true, snake.contains(Point{x: 1, y: 0}));
        assert_eq!(true, snake.contains(Point{x: 1, y: 1}));
    }
}

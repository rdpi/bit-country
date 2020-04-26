use crate::canvas::Canvas;
use crate::direction::Direction;
use crate::sprite::Sprite;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinates(u32, u32);

#[derive(Debug, Clone)]
pub struct Fren {
    position: Coordinates,
    height: u32,
    width: u32,
    speed: u32,
    direction: Option<Direction>,
    sprite: Sprite,
}

impl Fren {
    pub fn new(width: u32, height: u32, sprite: Sprite) -> Fren {
        Fren {
            position: Coordinates(0, 0),
            height,
            width,
            speed: 2,
            direction: None,
            sprite,
        }
    }

    pub fn change_direction(&mut self, direction: Option<Direction>) {
        self.direction = direction;
    }

    pub fn update(&mut self) {
        let direction = self.direction;
        match direction{
            Some(direction) => {
                let new_position = match direction {
                    Direction::Up => Coordinates(
                        self.position.0 % self.width,
                        (self.position.1.checked_sub(1).unwrap_or(self.height - self.speed)-1) % self.height,
                    ),
                    Direction::Down => Coordinates(
                        self.position.0 % self.width,
                        (self.position.1 + self.speed) % self.height,
                    ),
                    Direction::Right => Coordinates(
                        (self.position.0 + self.speed) % self.width,
                        self.position.1 % self.height,
                    ),
                    Direction::Left => Coordinates(
                        (self.position.0.checked_sub(1).unwrap_or(self.width - self.speed)-1) % self.width,
                        (self.position.1) % self.height,
                    ),
                    Direction::IdleUp |
                    Direction::IdleDown |
                    Direction:: IdleLeft |
                    Direction:: IdleRight => {Coordinates(self.position.0, self.position.1)},
                };

                self.position = new_position;
            }
            None => {}
        };
    }

    pub fn draw(&self, canvas: &Canvas, frame: u8) {
        canvas.draw_sprite(self.position.0, self.position.1, &self.sprite, &self.direction, frame);
    }
}
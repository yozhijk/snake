use crate::core::{GameMap, Tile};
use bracket_terminal::prelude::*;
use std::collections::VecDeque;

pub enum Outcome {
    MenuRequest,
    PlayerDead,
}

struct Snake {
    vel: (i32, i32),
    body: VecDeque<(usize, usize)>,
}

struct Rabbit {
    pos: (usize, usize),
}

pub struct Gameplay {
    map: GameMap,
    snake: Snake,
    rabbit: Rabbit,
}

impl Gameplay {
    fn random_rabbit(map_size: (usize, usize)) -> Rabbit {
        let x = rand::random::<usize>() % (map_size.0 - 2) + 1;
        let y = rand::random::<usize>() % (map_size.1 - 2) + 1;

        Rabbit { pos: (x, y) }
    }

    pub fn new(map_size: (usize, usize)) -> Self {
        Gameplay {
            map: GameMap::new(map_size),
            snake: Snake::new(&(map_size.0 / 2, map_size.1 / 2)),
            rabbit: Gameplay::random_rabbit(map_size),
        }
    }

    fn handle_input(&mut self, ctx: &mut BTerm) -> Option<Outcome> {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => {
                    self.snake.vel = (0, -1);
                    None
                }
                VirtualKeyCode::Down => {
                    self.snake.vel = (0, 1);
                    None
                }
                VirtualKeyCode::Left => {
                    self.snake.vel = (-1, 0);
                    None
                }
                VirtualKeyCode::Right => {
                    self.snake.vel = (1, 0);
                    None
                }
                VirtualKeyCode::Escape => Some(Outcome::MenuRequest),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> Option<Outcome> {
        let sel = self.handle_input(ctx);
        if sel.is_some() {
            sel
        } else if self.hit_walls() {
            Some(Outcome::PlayerDead)
        } else {
            let yummy = self.yummy();
            if yummy {
                self.rabbit = Gameplay::random_rabbit(self.map.size);
            }
            self.snake.advect(yummy);
            self.map.draw(ctx);
            self.draw_rabbit(ctx);
            self.snake.draw(ctx);
            None
        }
    }

    fn hit_walls(&mut self) -> bool {
        let ahead = self.snake.ahead();
        match self.map.tile_at(&ahead) {
            Tile::Wall => true,
            _ => false,
        }
    }

    fn yummy(&mut self) -> bool {
        let ahead = self.snake.ahead();
        ahead == self.rabbit.pos
    }

    fn draw_rabbit(&self, ctx: &mut BTerm) {
        ctx.print_color(
            self.rabbit.pos.0,
            self.rabbit.pos.1,
            RGBA::named(RED),
            RGBA::named(BLACK),
            "\u{263a}",
        )
    }
}

impl Snake {
    fn new(start: &(usize, usize)) -> Self {
        let mut body = VecDeque::from([*start]);
        for i in 0..2 {
            body.push_back((start.0 + i + 1, start.1));
        }

        Snake { vel: (0, 1), body }
    }

    fn draw(&self, ctx: &mut BTerm) {
        for i in self.body.iter() {
            ctx.print_color(i.0, i.1, RGBA::named(GREEN), RGBA::named(BLACK), "\u{2588}");
        }
    }

    fn advect(&mut self, rabbit: bool) {
        let head = self.head();
        let next = (head.0 as i32 + self.vel.0, head.1 as i32 + self.vel.1);
        self.body.push_front((next.0 as usize, next.1 as usize));
        if !rabbit {
            self.body.pop_back();
        }
    }

    fn head(&self) -> (usize, usize) {
        self.body[0]
    }

    fn ahead(&self) -> (usize, usize) {
        (
            (self.body[0].0 as i32 + self.vel.0) as usize,
            (self.body[0].1 as i32 + self.vel.1) as usize,
        )
    }
}

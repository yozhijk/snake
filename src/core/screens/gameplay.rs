use crate::core::{GameMap, Tile};
use bracket_terminal::prelude::*;
use std::collections::VecDeque;

pub enum ExitReason {
    EscapePressed,
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
    exit_reason: Option<ExitReason>,
}

impl Gameplay {
    fn random_rabbit(map_size: &(usize, usize)) -> Rabbit {
        let x = rand::random::<usize>() % (map_size.0 - 2) + 1;
        let y = rand::random::<usize>() % (map_size.1 - 2) + 1;

        Rabbit { pos: (x, y) }
    }

    pub fn new(map_size: &(usize, usize)) -> Self {
        Gameplay {
            map: GameMap::new(map_size),
            snake: Snake::new(&(map_size.0 / 2, map_size.1 / 2)),
            rabbit: Gameplay::random_rabbit(map_size),
            exit_reason: None,
        }
    }

    pub fn reset(&mut self) {
        self.snake = Snake::new(&(self.map.size.0 / 2, self.map.size.1 / 2));
        self.rabbit = Self::random_rabbit(&self.map.size);
    }

    fn handle_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.snake.vel = (0, -1),
                VirtualKeyCode::Down => self.snake.vel = (0, 1),
                VirtualKeyCode::Left => self.snake.vel = (-1, 0),
                VirtualKeyCode::Right => self.snake.vel = (1, 0),
                VirtualKeyCode::Escape => self.exit_reason = Some(ExitReason::EscapePressed),
                _ => {}
            }
        }
    }

    pub fn exit(&mut self) -> Option<ExitReason> {
        self.exit_reason.take()
    }

    pub fn tick(&mut self, ctx: &mut BTerm) {
        self.handle_input(ctx);
        if self.hit_walls() {
            self.exit_reason = Some(ExitReason::PlayerDead);
        } else {
            let yummy = self.yummy();
            if yummy {
                self.rabbit = Gameplay::random_rabbit(&self.map.size);
            }
            self.snake.advect(yummy);
            self.map.draw(ctx);
            self.draw_rabbit(ctx);
            self.snake.draw(ctx);
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
        if ahead == self.rabbit.pos {
            true
        } else {
            false
        }
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

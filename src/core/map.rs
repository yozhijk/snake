use bracket_terminal::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Wall,
}

pub struct GameMap {
    // map consist of tiles
    pub tiles: Vec<Tile>,
    pub size: (usize, usize),
}

impl GameMap {
    pub fn new(size: (usize, usize)) -> Self {
        let mut tiles = vec![Tile::Empty; size.0 * size.1];
        // create horizontal walls
        for i in 0..size.0 {
            tiles[i] = Tile::Wall;
            tiles[size.1 * size.0 - i - 1] = Tile::Wall;
        }
        for i in 0..size.1 {
            tiles[size.0 * i] = Tile::Wall;
            tiles[size.0 * i + size.0 - 1] = Tile::Wall;
        }
        GameMap { tiles, size: size }
    }

    pub fn draw(&mut self, ctx: &mut BTerm) {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                match self.tile_at(&(i, j)) {
                    Tile::Wall => ctx.print(i, j, "\u{2588}"),
                    _ => {}
                };
            }
        }
    }

    pub fn tile_at(&self, xy: &(usize, usize)) -> Tile {
        self.tiles[xy.1 * self.size.0 + xy.0]
    }
}

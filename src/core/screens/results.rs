use bracket_terminal::prelude::*;
pub struct Results {}

impl Results {
    pub fn new() -> Self {
        Results {}
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> Option<()> {
        let x = 5;
        let y = 8;
        ctx.print_color(x, y, RGBA::named(WHITE), RGBA::named(BLACK), "Game Over");
        ctx.draw_hollow_box(1, 1, 20, 14, RGBA::named(WHITE), RGBA::named(BLACK));

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => Some(()),
                _ => None,
            }
        } else {
            None
        }
    }
}

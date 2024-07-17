use bracket_terminal::prelude::*;
pub struct Results {
    done: Option<bool>,
}

impl Results {
    pub fn new() -> Self {
        Results { done: None }
    }

    pub fn done(&mut self) -> Option<bool> {
        self.done.take()
    }

    pub fn tick(&mut self, ctx: &mut BTerm) {
        let x = 5;
        let y = 8;
        ctx.print_color(x, y, RGBA::named(WHITE), RGBA::named(BLACK), "Game Over");
        ctx.draw_hollow_box(1, 1, 20, 14, RGBA::named(WHITE), RGBA::named(BLACK));

        if let Some(key) = ctx.key {
            if key == VirtualKeyCode::Escape {
                self.done = Some(true);
            }
        }
    }
}

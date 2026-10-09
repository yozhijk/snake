use bracket_terminal::prelude::*;
pub struct Menu {
    // current active menu item
    active_idx: usize,
    //
    choice: Option<usize>,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            active_idx: 0,
            choice: None,
        }
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> Option<usize> {
        self.handle_input(ctx);

        let x = 5;
        let mut y = 5;
        let items = vec!["1. New Game", "2. Options", "3. Exit"];

        for i in 0..items.len() {
            let col = if i == self.active_idx {
                RGBA::named(RED)
            } else {
                RGBA::named(GREEN)
            };

            ctx.print_color(x, y, col, RGBA::named(BLACK), items[i]);
            y += 3
        }

        ctx.draw_hollow_box(1, 1, 20, 14, RGBA::named(WHITE), RGBA::named(BLACK));

        self.choice.take()
    }

    fn handle_input(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            None => {}
            Some(key) => match key {
                // handle menu down
                VirtualKeyCode::Down => {
                    self.active_idx = if self.active_idx < 2 {
                        self.active_idx + 1
                    } else {
                        self.active_idx
                    }
                }
                // handle menu up
                VirtualKeyCode::Up => {
                    self.active_idx = if self.active_idx > 0 {
                        self.active_idx - 1
                    } else {
                        self.active_idx
                    }
                }
                // save choice
                VirtualKeyCode::Space => self.choice = Some(self.active_idx),
                _ => {}
            },
        }
    }
}

use bracket_terminal::prelude::*;

#[derive(Copy, Clone)]
pub enum Selection {
    NewGame,
    Options,
    Exit,
}

pub struct Menu {
    // current active menu item
    active_idx: usize,
}

static ITEMS: [(&str, Selection); 3] = [
    ("1. New Game", Selection::NewGame),
    ("2. Options", Selection::Options),
    ("3. Exit", Selection::Exit),
];

impl Menu {
    pub fn new() -> Self {
        Menu { active_idx: 0 }
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> Option<Selection> {
        let selection = self.handle_input(ctx);

        let x = 5;
        let mut y = 5;

        for i in 0..ITEMS.len() {
            let col = if i == self.active_idx {
                RGBA::named(RED)
            } else {
                RGBA::named(GREEN)
            };

            ctx.print_color(x, y, col, RGBA::named(BLACK), ITEMS[i].0);
            y += 3
        }

        ctx.draw_hollow_box(1, 1, 20, 14, RGBA::named(WHITE), RGBA::named(BLACK));
        selection
    }

    fn handle_input(&mut self, ctx: &mut BTerm) -> Option<Selection> {
        match ctx.key {
            Some(key) => match key {
                // handle menu down
                VirtualKeyCode::Down => {
                    self.active_idx = if self.active_idx < 2 {
                        self.active_idx + 1
                    } else {
                        self.active_idx
                    };
                    None
                }
                // handle menu up
                VirtualKeyCode::Up => {
                    self.active_idx = if self.active_idx > 0 {
                        self.active_idx - 1
                    } else {
                        self.active_idx
                    };
                    None
                }
                // save choice
                VirtualKeyCode::Space => Some(ITEMS[self.active_idx].1),
                _ => None,
            },
            _ => None,
        }
    }
}

use super::screens::gameplay::Outcome;
use super::screens::menu::Selection;
use super::screens::{Gameplay, Menu, Results};

use bracket_lib::terminal::GameState;
use bracket_terminal::prelude::*;

pub enum Screen {
    Menu(Menu),
    Gameplay(Gameplay),
    Results(Results),
}

pub struct State {
    screen: Screen,
    map_size: (usize, usize),
}

impl State {
    pub fn new(map_size: (usize, usize)) -> Self {
        State {
            screen: Screen::Menu(Menu::new()),
            map_size,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear the screen
        ctx.cls();

        match self.screen {
            // show game menu screen in Game::Menu mode
            Screen::Menu(ref mut menu) => {
                // update menu
                if let Some(sel) = menu.tick(ctx) {
                    match sel {
                        Selection::NewGame => {
                            // new game selected
                            self.screen = Screen::Gameplay(Gameplay::new(self.map_size))
                        }
                        // exit selected
                        Selection::Exit => ctx.quit(),
                        // Options
                        _ => (),
                    }
                }
            }
            // update gameplay
            Screen::Gameplay(ref mut gameplay) => {
                if let Some(outcome) = gameplay.tick(ctx) {
                    match outcome {
                        Outcome::MenuRequest => self.screen = Screen::Menu(Menu::new()),
                        Outcome::PlayerDead => self.screen = Screen::Results(Results::new()),
                    }
                }
            }
            Screen::Results(ref mut results) => {
                if results.tick(ctx).is_some() {
                    self.screen = Screen::Menu(Menu::new());
                }
            }
        }
    }
}

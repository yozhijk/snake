use super::screens::gameplay::Outcome;
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
                if let Some(choice) = menu.tick(ctx) {
                    match choice {
                        0 => {
                            // new game selected
                            self.screen = Screen::Gameplay(Gameplay::new(self.map_size))
                        }
                        // exit selected
                        2 => ctx.quit(),
                        // nothing selected
                        _ => (),
                    }
                }
            }
            // update gameplay
            Screen::Gameplay(ref mut gameplay) => {
                if let Some(outcome) = gameplay.tick(ctx) {
                    match outcome {
                        Outcome::EscapePressed => self.screen = Screen::Menu(Menu::new()),
                        Outcome::PlayerDead => self.screen = Screen::Results(Results::new()),
                    }
                }
            }
            Screen::Results(ref mut results) => {
                if let Some(done) = results.tick(ctx) {
                    if done {
                        self.screen = Screen::Menu(Menu::new());
                    }
                }
            }
        }
    }
}

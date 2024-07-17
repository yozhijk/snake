use super::screens::ExitReason;
use super::screens::{Gameplay, Menu, Results};

use bracket_lib::terminal::GameState;
use bracket_terminal::prelude::*;

pub enum GameMode {
    Menu,
    Gameplay,
    Results,
}

pub struct State {
    // current screen
    mode: GameMode,
    // menu screen
    menu: Menu,
    // gameplay screen
    gameplay: Gameplay,
    // result screen
    results: Results,
}

impl State {
    pub fn new(map_size: &(usize, usize)) -> Self {
        State {
            mode: GameMode::Menu,
            menu: Menu::new(),
            gameplay: Gameplay::new(map_size),
            results: Results::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear the screen
        ctx.cls();

        match self.mode {
            // show game menu screen in Game::Menu mode
            GameMode::Menu => {
                // update menu
                self.menu.tick(ctx);
                // check if anything has been selected
                if let Some(choice) = self.menu.choice() {
                    match choice {
                        0 => {
                            // new game selected
                            self.mode = GameMode::Gameplay;
                            self.gameplay.reset()
                        }
                        // exit selected
                        2 => ctx.quit(),
                        // nothing selected
                        _ => (),
                    }
                }
            }
            // update gameplay
            GameMode::Gameplay => {
                self.gameplay.tick(ctx);
                // check if gameplay has ended
                if let Some(reason) = self.gameplay.exit() {
                    match reason {
                        ExitReason::EscapePressed => self.mode = GameMode::Menu,
                        ExitReason::PlayerDead => self.mode = GameMode::Results,
                    }
                }
            }
            GameMode::Results => {
                self.results.tick(ctx);
                if let Some(done) = self.results.done() {
                    if done {
                        self.mode = GameMode::Menu;
                    }
                }
            }
        }
    }
}

use bracket_terminal::prelude::*;
mod core;

use crate::core::State;

fn main() -> BResult<()> {
    // add error handling later
    // create terminal
    let context = BTermBuilder::simple80x50().with_title("test").build()?;
    // create game state
    let game_state = State::new(&(80, 50));
    // run game loop
    main_loop(context, game_state)?;
    Ok(())
}

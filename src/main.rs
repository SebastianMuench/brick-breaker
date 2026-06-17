use macroquad::prelude::*;

use crate::game::Game;

mod entities;
mod game;
mod state;
mod system;

pub const BLOCKS_W: usize = 10;
pub const BLOCKS_H: usize = 10;
const SPEED_MULTIPLYER: f32 = 1.1;

#[macroquad::main("Seppong")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        game.update();
        game.draw();

        next_frame().await
    }
}

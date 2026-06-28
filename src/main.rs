use macroquad::prelude::*;

use crate::game::Game;

mod draw;
mod entities;
mod game;
mod state;
mod system;

pub const BLOCKS_W: usize = 10;
pub const BLOCKS_H: usize = 10;

// we'll use this to scale the entire game
pub const WORLD_W: f32 = 500.0;
pub const WORLD_H: f32 = 500.0;
pub const PADDLE_MOVE_DELTA: f32 = WORLD_W * 0.01;

const SPEED_MULTIPLYER: f32 = 1.1;

#[macroquad::main("Seppong")]
async fn main() {
    let paddle_texture = load_texture("assets/yachter.png").await.unwrap();
    let block_texture = load_texture("assets/block.png").await.unwrap();
    let ball_texture = load_texture("assets/bottle-cap.png").await.unwrap();
    let holy_texture = load_texture("assets/holy-energy.png").await.unwrap();
    let monster_texture = load_texture("assets/monster.png").await.unwrap();

    let mut game = Game::new(
        paddle_texture,
        block_texture,
        ball_texture,
        holy_texture,
        monster_texture,
    );

    loop {
        clear_background(BLACK);

        game.update();
        game.draw();

        next_frame().await
    }
}

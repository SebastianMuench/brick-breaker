use macroquad::prelude::*;

use crate::game::{Game, GameTextures};

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
    let extra_ball_texture = load_texture("assets/holy-energy.png").await.unwrap();
    let paddle_expand_texture = load_texture("assets/monster.png").await.unwrap();
    let rainbow_mode_texture = load_texture("assets/magic-mushroom-power-up.png")
        .await
        .unwrap();
    let fire_ball_texture = load_texture("assets/fireball-power-up.png").await.unwrap();
    let fire_ball_effects = vec![
        load_texture("assets/fireball-beercap-0.png").await.unwrap(),
        load_texture("assets/fireball-beercap-1.png").await.unwrap(),
        load_texture("assets/fireball-beercap-2.png").await.unwrap(),
        load_texture("assets/fireball-beercap-3.png").await.unwrap(),
        load_texture("assets/fireball-beercap-4.png").await.unwrap(),
    ];
    let beer_fountain_sheet_texture = load_texture("assets/beer-fountain-sheet.png")
        .await
        .unwrap();
    let beer_fountain_splash_texture = load_texture("assets/beer-fountain-splash.png")
        .await
        .unwrap();
    let upright_beer_bottle_texture = load_texture("assets/yachter-upright.png").await.unwrap();

    let textures = GameTextures {
        paddle: paddle_texture,
        block: block_texture,
        ball: ball_texture,
        extra_ball_power_up: extra_ball_texture,
        paddle_expand_power_up: paddle_expand_texture,
        rainbow_mode_power_up: rainbow_mode_texture,
        fire_ball_power_up: fire_ball_texture,
        fire_ball_effects,
        beer_fountain_sheet: beer_fountain_sheet_texture,
        beer_fountain_splash: beer_fountain_splash_texture,
        upright_beer_bottle: upright_beer_bottle_texture,
    };
    let mut game = Game::new(textures);

    loop {
        clear_background(BLACK);

        game.update();
        game.draw();

        next_frame().await
    }
}

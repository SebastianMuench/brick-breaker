use macroquad::{
    color::WHITE,
    shapes::{draw_circle, draw_line, draw_rectangle},
    text::draw_text,
    window::{screen_height, screen_width},
};

use crate::{BLOCKS_H, BLOCKS_W, game::Game};

pub fn draw_game_over() {
    draw_text(
        "Game Over! Press Space to Restart",
        screen_width() / 2.0 - 150.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );
}

pub fn draw_game_won() {
    draw_text(
        "You Won! Press Space to Restart",
        screen_width() / 2.0 - 150.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );
}

pub fn draw_pause() {
    draw_text(
        "Game Paused! Press P to Resume",
        screen_width() / 2.0 - 150.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );
}

pub fn draw_start() {
    draw_text(
        "Press Space to Start",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );
}

pub fn draw_game(game: &Game) {
    draw_line(
        game.player.x,
        game.player.y,
        game.player.x + game.player.width,
        game.player.y,
        5.0,
        WHITE,
    );

    draw_circle(game.ball.x, game.ball.y, game.ball.radius, WHITE);

    for (j, row) in game.blocks.iter().enumerate().take(BLOCKS_H) {
        for (i, block) in row.iter().enumerate().take(BLOCKS_W) {
            if *block {
                let block_width = screen_width() / BLOCKS_W as f32;
                let block_height = screen_height() / (2.0 * BLOCKS_H as f32);
                draw_rectangle(
                    i as f32 * block_width,
                    j as f32 * block_height,
                    block_width - 2.0,
                    block_height - 2.0,
                    WHITE,
                );
            }
        }
    }
}

use macroquad::{
    camera::{Camera2D, set_camera},
    color::{Color, WHITE},
    math::{Vec2, vec2},
    shapes::{draw_circle, draw_rectangle},
    text::{draw_text, measure_text},
    texture::{DrawTextureParams, draw_texture_ex},
    window::{screen_height, screen_width},
};

use crate::{BLOCKS_H, BLOCKS_W, WORLD_H, WORLD_W, game::Game};

fn draw_centered_text(text: &str, font_size: u16, color: Color) {
    // Optional: draw a semi-transparent background overlay to make the text pop
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.5),
    );

    let dimensions = measure_text(text, None, font_size, 1.0);
    let x = (screen_width() - dimensions.width) / 2.0;
    let y = (screen_height() - dimensions.height) / 2.0 + dimensions.offset_y;
    draw_text(text, x, y, font_size as f32, color);
}

pub fn draw_game_over() {
    draw_centered_text("Game Over! Press Space to Restart", 32, WHITE);
}

pub fn draw_game_won() {
    draw_centered_text("You Won! Press Space to Restart", 32, WHITE);
}

pub fn draw_pause() {
    draw_centered_text("Game Paused! Press P to Resume", 32, WHITE);
}

pub fn draw_start() {
    draw_centered_text("Press Space to Start", 32, WHITE);
}

pub fn draw_game(game: &Game) {
    draw_texture_ex(
        &game.paddle_texture,
        game.player.x,
        game.player.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(game.player.width, game.player.height)),
            ..Default::default()
        },
    );

    for ball in game.balls.iter() {
        draw_circle(ball.x, ball.y, ball.radius, WHITE);
    }

    for (j, row) in game.blocks.iter().enumerate().take(BLOCKS_H) {
        for (i, block) in row.iter().enumerate().take(BLOCKS_W) {
            if block.active {
                let block_width = WORLD_W / BLOCKS_W as f32;
                let block_height = WORLD_H / (2.0 * BLOCKS_H as f32);
                draw_rectangle(
                    i as f32 * block_width + 0.25,
                    j as f32 * block_height + 0.25,
                    block_width - 0.5,
                    block_height - 0.5,
                    WHITE,
                );
            }
        }
    }
}

pub fn set_game_camera() {
    set_camera(&Camera2D {
        zoom: vec2(2.0 / WORLD_W, 2.0 / WORLD_H),
        target: vec2(WORLD_W / 2.0, WORLD_H / 2.0),
        ..Default::default()
    });
}

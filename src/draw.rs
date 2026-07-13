use macroquad::{
    camera::{Camera2D, set_camera},
    color::{Color, WHITE},
    math::{Rect as TextureRect, Vec2, vec2},
    shapes::draw_rectangle,
    text::{draw_text, measure_text},
    texture::{DrawTextureParams, draw_texture_ex},
    time::get_time,
    window::{screen_height, screen_width},
};

use crate::{
    BLOCKS_H, BLOCKS_W, WORLD_H, WORLD_W,
    entities::{
        BEER_FOUNTAIN_FRAME_COUNT, BEER_FOUNTAIN_FRAME_TIME, BallEffect,
        CHARCOAL_BLOCK_FRAME_COUNT, CHARCOAL_BLOCK_FRAME_TIME, FallingPowerUp, PowerUp,
    },
    game::Game,
};

const FIRE_BALL_EFFECT_SCALE: f32 = 1.4;

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
    let color = get_color(game);

    draw_player(game, color);

    for ball in game.entities.balls.iter() {
        match ball.ball_effect {
            BallEffect::Normal => {
                draw_texture_ex(
                    &game.textures.ball,
                    ball.x - ball.radius,
                    ball.y - ball.radius,
                    color,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(ball.radius * 2.0, ball.radius * 2.0)),
                        ..Default::default()
                    },
                );
            }
            BallEffect::Fire { animation, .. } => {
                let fire_ball_effect_texture = &game.textures.fire_ball_effects[animation.frame];
                let effect_size = ball.radius * 2.0 * FIRE_BALL_EFFECT_SCALE;
                let effect_offset = effect_size / 2.0;

                draw_texture_ex(
                    fire_ball_effect_texture,
                    ball.x - effect_offset,
                    ball.y - effect_offset,
                    color,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(effect_size, effect_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    for (j, row) in game.entities.blocks.iter().enumerate().take(BLOCKS_H) {
        for (i, block) in row.iter().enumerate().take(BLOCKS_W) {
            if block.active {
                let block_width = WORLD_W / BLOCKS_W as f32;
                let block_height = WORLD_H / (2.0 * BLOCKS_H as f32);
                let grill_active = game.timers.grill_end_time > 0.0;
                let texture = if grill_active {
                    &game.textures.charcoal_block_sheet
                } else {
                    &game.textures.block
                };
                let source = if grill_active {
                    let frame_width = game.textures.charcoal_block_sheet.width()
                        / CHARCOAL_BLOCK_FRAME_COUNT as f32;
                    let frame_height = game.textures.charcoal_block_sheet.height();
                    let frame = ((get_time() / CHARCOAL_BLOCK_FRAME_TIME) as usize + i + j)
                        % CHARCOAL_BLOCK_FRAME_COUNT;

                    Some(TextureRect::new(
                        frame as f32 * frame_width,
                        0.0,
                        frame_width,
                        frame_height,
                    ))
                } else {
                    None
                };

                draw_texture_ex(
                    texture,
                    i as f32 * block_width,
                    j as f32 * block_height,
                    color,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(block_width, block_height)),
                        source,
                        ..Default::default()
                    },
                );
            }
        }
    }

    // falling power ups
    for power_up in game.entities.falling_power_ups.iter() {
        draw_power_up(game, power_up, color);
    }
}

fn draw_player(game: &Game, color: Color) {
    if game.timers.beer_fountain_end_time > 0.0 {
        draw_beer_fountain(game, color);
    } else {
        draw_texture_ex(
            &game.textures.paddle,
            game.entities.player.x,
            game.entities.player.y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    game.entities.player.width,
                    game.entities.player.height,
                )),
                ..Default::default()
            },
        );
    }
}

fn draw_power_up(game: &Game, power_up: &FallingPowerUp, color: Color) {
    let texture = match power_up.power_up {
        PowerUp::ExtraBall => &game.textures.extra_ball_power_up,
        PowerUp::PaddleExpand => &game.textures.paddle_expand_power_up,
        PowerUp::RainbowMode => &game.textures.rainbow_mode_power_up,
        PowerUp::FireBall => &game.textures.fire_ball_power_up,
        PowerUp::BeerFountain => &game.textures.beer_fountain_splash,
        PowerUp::StickyPaddle => &game.textures.sticky_paddle_power_up,
        PowerUp::Grill => &game.textures.grill_power_up,
    };

    draw_texture_ex(
        texture,
        power_up.x,
        power_up.y,
        color,
        DrawTextureParams {
            dest_size: Some(Vec2::new(power_up.width, power_up.height)),
            ..Default::default()
        },
    );
}

fn draw_beer_fountain(game: &Game, color: Color) {
    let fountain = game.entities.player.beer_fountain_rect();
    let bottle = game.entities.player.upright_beer_bottle_rect();
    let splash = game.entities.player.beer_fountain_splash_rect();
    let frame_width = game.textures.beer_fountain_sheet.width() / BEER_FOUNTAIN_FRAME_COUNT as f32;
    let frame_height = game.textures.beer_fountain_sheet.height();
    let frame = ((get_time() / BEER_FOUNTAIN_FRAME_TIME) as usize) % BEER_FOUNTAIN_FRAME_COUNT;

    draw_texture_ex(
        &game.textures.beer_fountain_sheet,
        fountain.x,
        fountain.y,
        color,
        DrawTextureParams {
            dest_size: Some(Vec2::new(fountain.width, fountain.height)),
            source: Some(TextureRect::new(
                frame as f32 * frame_width,
                0.0,
                frame_width,
                frame_height,
            )),
            ..Default::default()
        },
    );

    draw_texture_ex(
        &game.textures.upright_beer_bottle,
        bottle.x,
        bottle.y,
        color,
        DrawTextureParams {
            dest_size: Some(Vec2::new(bottle.width, bottle.height)),
            ..Default::default()
        },
    );

    if game.timers.beer_fountain_splash_end_time > 0.0 {
        draw_texture_ex(
            &game.textures.beer_fountain_splash,
            splash.x,
            splash.y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(splash.width, splash.height)),
                ..Default::default()
            },
        );
    }
}

pub fn set_game_camera() {
    set_camera(&Camera2D {
        zoom: vec2(2.0 / WORLD_W, 2.0 / WORLD_H),
        target: vec2(WORLD_W / 2.0, WORLD_H / 2.0),
        ..Default::default()
    });
}

fn get_color(game: &Game) -> Color {
    if game.timers.rainbow_mode_end_time > 0.0 {
        let t = get_time() as f32;
        Color::new(
            (t * 2.0).sin() * 0.5 + 0.5,
            (t * 2.0 + 2.0).sin() * 0.5 + 0.5,
            (t * 2.0 + 4.0).sin() * 0.5 + 0.5,
            1.0,
        )
    } else {
        WHITE
    }
}

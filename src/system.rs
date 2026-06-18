use crate::{
    BLOCKS_H, BLOCKS_W,
    entities::{Ball, Block},
    game::Game,
    state::GameState,
};
use macroquad::prelude::*;

pub fn toggle_game(game: &mut Game) {
    if is_key_pressed(KeyCode::P) {
        game.state = match game.state {
            GameState::Playing => GameState::Pause,
            GameState::Pause => GameState::Playing,
            s => s,
        };
    }
}

pub fn increase_speed(game: &mut Game, speed_multiplier: f32) {
    // every 10 seconds, increase the speed of the ball by 10% // this currently isn't working but i'm not sure why
    if get_time() > game.next_speed_increase_time {
        game.next_speed_increase_time += 10.0;
        for ball in game.balls.iter_mut() {
            ball.velocity_x *= speed_multiplier;
            ball.velocity_y *= speed_multiplier;
        }
    }
}

pub fn game_won(game: &Game) -> bool {
    // if all blocks are false, we won
    for row in game.blocks.iter().take(BLOCKS_H) {
        for block in row.iter().take(BLOCKS_W) {
            if *block {
                return false;
            }
        }
    }

    true
}

pub fn game_lost(balls: &Vec<Ball>) -> bool {
    // if there's only one ball and it's below the screen, we lost
    if balls.is_empty() {
        return true;
    }

    false
}

pub fn handle_block_collision(game: &mut Game) {
    let mut collision_occurred = false;

    for ball in game.balls.iter_mut() {
        for (j, row) in game.blocks.iter_mut().enumerate() {
            for (i, block) in row.iter_mut().enumerate() {
                if *block && !collision_occurred {
                    let block_rect: Block = calculate_block_rect(i, j);

                    if block_collisides(&ball, &block_rect) {
                        let directions = detect_direction(&ball, &block_rect);
                        *block = false;
                        if directions.0 || directions.1 {
                            ball.velocity_x = -ball.velocity_x;
                        } else {
                            ball.velocity_y = -ball.velocity_y;
                        }
                        collision_occurred = true;
                        break;
                    }
                }
            }

            if collision_occurred {
                break;
            }
        }
    }
}

pub fn detect_direction(ball: &Ball, block: &Block) -> (bool, bool, bool, bool) {
    (
        ball.prev_x + ball.radius <= block.x,
        ball.prev_x - ball.radius >= block.x + block.width,
        ball.prev_y + ball.radius <= block.y,
        ball.prev_y - ball.radius >= block.y + block.height,
    )
}

pub fn calculate_block_rect(i: usize, j: usize) -> Block {
    let block_width = screen_width() / BLOCKS_W as f32;
    let block_height = screen_height() / (2.0 * BLOCKS_H as f32);
    Block {
        x: i as f32 * block_width,
        y: j as f32 * block_height,
        width: block_width,
        height: block_height,
    }
}

pub fn block_collisides(ball: &Ball, block: &Block) -> bool {
    ball.x + ball.radius >= block.x
        && ball.x - ball.radius <= block.x + block.width
        && ball.y + ball.radius >= block.y
        && ball.y - ball.radius <= block.y + block.height
}

pub fn handle_site_collision(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        if ball.x - ball.radius <= 0.0 || ball.x + ball.radius >= screen_width() {
            ball.velocity_x = -ball.velocity_x;
        }
    }
}

pub fn handle_top_collision(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        if ball.y - ball.radius <= 0.0 {
            ball.velocity_y = -ball.velocity_y;
        }
    }
}

pub fn handle_paddle_collision(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        if ball.y + ball.radius >= game.player.y
            && ball.x >= game.player.x
            && ball.x <= game.player.x + game.player.width
        {
            ball.velocity_y = -ball.velocity_y;
        }
    }
}

pub fn check_ball_out_of_bounds(game: &mut Game) {
    game.balls
        .retain(|ball| ball.y - ball.radius <= screen_height());
}

pub fn update_ball_position(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        ball.x += ball.velocity_x;
        ball.y += ball.velocity_y;
    }
}

pub fn update_ball_previous_position(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        ball.prev_x = ball.x;
        ball.prev_y = ball.y;
    }
}

use crate::{
    BLOCKS_H, BLOCKS_W, WORLD_H, WORLD_W,
    entities::{Ball, BlockCoordinates},
    game::Game,
    state::GameState,
};
use macroquad::prelude::*;

pub fn toggle_game(game: &mut Game) {
    if is_key_pressed(KeyCode::P) {
        game.state = match game.state {
            GameState::Playing => GameState::Pause,
            GameState::Pause => {
                game.next_speed_increase_time = get_time() + 10.0;
                GameState::Playing
            }
            s => s,
        };
    }
}

pub fn increase_speed(game: &mut Game, speed_multiplier: f32) {
    // every 10 seconds, increase the speed of the ball by 10%
    if get_time() > game.next_speed_increase_time {
        game.next_speed_increase_time = get_time() + 10.0;
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
            if block.active {
                return false;
            }
        }
    }

    true
}

pub fn game_lost(balls: &[Ball]) -> bool {
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
                if block.active && !collision_occurred {
                    let block_rect: BlockCoordinates = calculate_block_rect(i, j);

                    if block_collides(ball, &block_rect) {
                        block.active = false;

                        // find the point on the block that is closest to the ball center
                        let closest_x = ball.x.clamp(block_rect.x, block_rect.x + block_rect.width);
                        let closest_y =
                            ball.y.clamp(block_rect.y, block_rect.y + block_rect.height);

                        // figure out how far the ball has pushed inside the block on both axes
                        let overlap_x = ball.radius - (ball.x - closest_x).abs();
                        let overlap_y = ball.radius - (ball.y - closest_y).abs();

                        // we bounce on the axis with the smaller overlap because that's where the hit happened
                        if overlap_x < overlap_y {
                            ball.velocity_x = -ball.velocity_x;
                            // push the ball back outside the block on the x axis so it doesn't get stuck
                            if ball.x < closest_x {
                                ball.x = block_rect.x - ball.radius;
                            } else {
                                ball.x = block_rect.x + block_rect.width + ball.radius;
                            }
                        } else {
                            ball.velocity_y = -ball.velocity_y;
                            // push the ball back outside the block on the y axis so it doesn't get stuck
                            if ball.y < closest_y {
                                ball.y = block_rect.y - ball.radius;
                            } else {
                                ball.y = block_rect.y + block_rect.height + ball.radius;
                            }
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

pub fn calculate_block_rect(i: usize, j: usize) -> BlockCoordinates {
    let block_width = WORLD_W / BLOCKS_W as f32;
    let block_height = WORLD_H / (2.0 * BLOCKS_H as f32);
    BlockCoordinates {
        x: i as f32 * block_width,
        y: j as f32 * block_height,
        width: block_width,
        height: block_height,
    }
}

pub fn block_collides(ball: &Ball, block: &BlockCoordinates) -> bool {
    ball.x + ball.radius >= block.x
        && ball.x - ball.radius <= block.x + block.width
        && ball.y + ball.radius >= block.y
        && ball.y - ball.radius <= block.y + block.height
}

pub fn handle_site_collision(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        if ball.x - ball.radius <= 0.0 {
            ball.velocity_x = ball.velocity_x.abs();
            ball.x = ball.radius;
        } else if ball.x + ball.radius >= WORLD_W {
            ball.velocity_x = -ball.velocity_x.abs();
            ball.x = WORLD_W - ball.radius;
        }
    }
}

pub fn handle_top_collision(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        if ball.y - ball.radius <= 0.0 {
            ball.velocity_y = ball.velocity_y.abs();
            ball.y = ball.radius;
        }
    }
}

pub fn handle_paddle_collision(game: &mut Game) {
    for ball in game.balls.iter_mut() {
        // check if the ball overlaps with the paddle area
        if ball.y + ball.radius >= game.player.y
            && ball.y - ball.radius <= game.player.y + game.player.height
            && ball.x >= game.player.x
            && ball.x <= game.player.x + game.player.width
        {
            // only bounce if the ball is moving down to prevent it from getting stuck inside the paddle
            if ball.velocity_y > 0.0 {
                ball.velocity_y = -ball.velocity_y;
                // push the ball back up to the top surface of the paddle
                ball.y = game.player.y - ball.radius;
            }
        }
    }
}

pub fn check_ball_out_of_bounds(game: &mut Game) {
    game.balls.retain(|ball| ball.y - ball.radius <= WORLD_H);
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

// we need to do this in order to make sure the player is displayed correctly even after a resize
pub fn update_player_position(game: &mut Game) {
    game.player.y = WORLD_H * 0.9;
}

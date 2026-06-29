use crate::{
    BLOCKS_H, BLOCKS_W, PADDLE_MOVE_DELTA, WORLD_H, WORLD_W,
    entities::{self, Ball, BallEffect, BlockCoordinates, FallingPowerUp, PowerUp},
    game::Game,
    state::GameState,
};
use macroquad::prelude::*;

const PADDLE_ENGLISH_STRENGTH: f32 = 0.45;
const MAX_HORIZONTAL_BOUNCE_RATIO: f32 = 0.85;
const PADDLE_STILL_EPSILON: f32 = 0.01;

pub fn toggle_game(game: &mut Game) {
    if is_key_pressed(KeyCode::P) {
        game.state = match game.state {
            GameState::Playing => GameState::Pause,
            GameState::Pause => {
                game.timers.next_speed_increase_time = get_time() + 10.0;
                GameState::Playing
            }
            s => s,
        };
    }
}

pub fn increase_speed(game: &mut Game, speed_multiplier: f32) {
    // every 10 seconds, increase the speed of the ball by 10%
    if get_time() > game.timers.next_speed_increase_time {
        game.timers.next_speed_increase_time = get_time() + 10.0;
        for ball in game.entities.balls.iter_mut() {
            ball.velocity_x *= speed_multiplier;
            ball.velocity_y *= speed_multiplier;
        }
    }
}

pub fn game_won(game: &Game) -> bool {
    // if all blocks are false, we won
    for row in game.entities.blocks.iter().take(BLOCKS_H) {
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
    let balls = &mut game.entities.balls;
    let blocks = &mut game.entities.blocks;
    let falling_power_ups = &mut game.entities.falling_power_ups;

    for ball in balls.iter_mut() {
        for (j, row) in blocks.iter_mut().enumerate() {
            for (i, block) in row.iter_mut().enumerate() {
                if block.active && !collision_occurred {
                    let block_rect: BlockCoordinates = calculate_block_rect(i, j);

                    if block_collides(ball, &block_rect) {
                        match ball.ball_effect {
                            BallEffect::Normal => {
                                handle_normal_collision(falling_power_ups, ball, block, block_rect);
                                collision_occurred = true;
                                break;
                            }
                            BallEffect::Fire { .. } => {
                                block.active = false;
                                let power_up = block.power_up;
                                handle_power_up(falling_power_ups, block_rect, power_up);
                            }
                        }
                    }
                }
            }

            if collision_occurred {
                break;
            }
        }
    }
}

fn handle_normal_collision(
    falling_power_ups: &mut Vec<FallingPowerUp>,
    ball: &mut Ball,
    block: &mut entities::Block,
    block_rect: BlockCoordinates,
) {
    block.active = false;
    let power_up = block.power_up;
    handle_power_up(falling_power_ups, block_rect, power_up);

    // find the point on the block that is closest to the ball center
    let closest_x = ball.x.clamp(block_rect.x, block_rect.x + block_rect.width);
    let closest_y = ball.y.clamp(block_rect.y, block_rect.y + block_rect.height);

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
}

fn handle_power_up(
    falling_power_ups: &mut Vec<FallingPowerUp>,
    block_rect: BlockCoordinates,
    power_up: Option<PowerUp>,
) {
    if let Some(power_up) = power_up {
        match power_up {
            PowerUp::ExtraBall => {
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0,
                    y: block_rect.y + block_rect.height / 2.0,
                    width: block_rect.width * 0.8,
                    height: block_rect.height * 1.4,
                    power_up,
                    velocity_y: WORLD_H * 0.002,
                });
            }
            PowerUp::PaddleExpand => {
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0,
                    y: block_rect.y + block_rect.height / 2.0,
                    width: block_rect.width * 1.0,
                    height: block_rect.height * 2.4,
                    power_up,
                    velocity_y: WORLD_H * 0.002,
                });
            }
            PowerUp::RainbowMode => {
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0,
                    y: block_rect.y + block_rect.height / 2.0,
                    width: block_rect.width * 1.0,
                    height: block_rect.height * 2.4,
                    power_up,
                    velocity_y: WORLD_H * 0.002,
                });
            }
            PowerUp::FireBall => {
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0,
                    y: block_rect.y + block_rect.height / 2.0,
                    width: block_rect.width * 1.0,
                    height: block_rect.height * 2.4,
                    power_up,
                    velocity_y: WORLD_H * 0.002,
                });
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
    for ball in game.entities.balls.iter_mut() {
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
    for ball in game.entities.balls.iter_mut() {
        if ball.y - ball.radius <= 0.0 {
            ball.velocity_y = ball.velocity_y.abs();
            ball.y = ball.radius;
        }
    }
}

fn calculate_paddle_bounce_velocity(
    ball_velocity_x: f32,
    ball_velocity_y: f32,
    paddle_velocity_x: f32,
) -> (f32, f32) {
    if paddle_velocity_x.abs() < PADDLE_STILL_EPSILON {
        return (ball_velocity_x, -ball_velocity_y);
    }

    let speed = ball_velocity_x.hypot(ball_velocity_y);
    if speed == 0.0 {
        return (0.0, 0.0);
    }

    let paddle_motion = (paddle_velocity_x / PADDLE_MOVE_DELTA).clamp(-1.0, 1.0);
    let max_horizontal_velocity = speed * MAX_HORIZONTAL_BOUNCE_RATIO;
    let new_velocity_x = (ball_velocity_x + paddle_motion * speed * PADDLE_ENGLISH_STRENGTH)
        .clamp(-max_horizontal_velocity, max_horizontal_velocity);
    let new_velocity_y = -((speed * speed - new_velocity_x * new_velocity_x)
        .max(0.0)
        .sqrt());

    (new_velocity_x, new_velocity_y)
}

pub fn handle_paddle_collision(game: &mut Game) {
    let paddle_hitbox = game.entities.player.hitbox();
    let paddle_velocity_x = game.entities.player.velocity_x;

    for ball in game.entities.balls.iter_mut() {
        if ball.y + ball.radius >= paddle_hitbox.y
            && ball.y - ball.radius <= paddle_hitbox.y + paddle_hitbox.height
            && ball.x >= paddle_hitbox.x
            && ball.x <= paddle_hitbox.x + paddle_hitbox.width
        {
            // only bounce if the ball is moving down to prevent it from getting stuck inside the paddle
            if ball.velocity_y > 0.0 {
                (ball.velocity_x, ball.velocity_y) = calculate_paddle_bounce_velocity(
                    ball.velocity_x,
                    ball.velocity_y,
                    paddle_velocity_x,
                );
                // push the ball back up to the visible top surface of the paddle
                ball.y = paddle_hitbox.y - ball.radius;
            }
        }
    }
}

pub fn handle_power_up_collision(game: &mut Game) {
    let fire_ball_effect_frame_count = game.textures.fire_ball_effects.len();
    let player = &mut game.entities.player;
    let balls = &mut game.entities.balls;
    let falling_power_ups = &mut game.entities.falling_power_ups;
    let rainbow_mode_end_time = &mut game.timers.rainbow_mode_end_time;
    let paddle_hitbox = player.hitbox();

    falling_power_ups.retain(|power_up| {
        //player collision
        let collides = power_up.y + power_up.height >= paddle_hitbox.y
            && power_up.y <= paddle_hitbox.y + paddle_hitbox.height
            && power_up.x + power_up.width >= paddle_hitbox.x
            && power_up.x <= paddle_hitbox.x + paddle_hitbox.width;

        if collides {
            match power_up.power_up {
                PowerUp::ExtraBall => {
                    balls.push(Ball::new());
                }
                PowerUp::PaddleExpand => {
                    // Keep the expanded paddle inside the world.  If it is against the
                    // right edge, simply increasing its width would grow entirely
                    // off-screen and make the pickup appear to have no effect.
                    // let center_x = player.x + player.width / 2.0;
                    // player.width = (player.width * 1.5).min(WORLD_W);
                    // player.x = (center_x - player.width / 2.0)
                    //     .clamp(0.0, WORLD_W - player.width);
                    player.expand_power_up_end_times.push(get_time() + 30.0);
                }
                PowerUp::RainbowMode => {
                    *rainbow_mode_end_time = get_time() + 30.0;
                }
                PowerUp::FireBall => {
                    for ball in balls.iter_mut() {
                        ball.ball_effect = BallEffect::Fire {
                            expires_at: get_time() + 30.0,
                            animation: entities::Animation::new(fire_ball_effect_frame_count, 0.1),
                        };
                    }
                }
            }
        }

        // check if the power up is out of bounds
        let out_of_bounds = power_up.y > WORLD_H;

        !collides && !out_of_bounds
    });
}

pub fn check_ball_out_of_bounds(game: &mut Game) {
    game.entities
        .balls
        .retain(|ball| ball.y - ball.radius <= WORLD_H);
}

pub fn update_ball_position(game: &mut Game) {
    for ball in game.entities.balls.iter_mut() {
        ball.x += ball.velocity_x;
        ball.y += ball.velocity_y;
    }
}

pub fn update_falling_power_ups_position(game: &mut Game) {
    for power_up in game.entities.falling_power_ups.iter_mut() {
        power_up.y += power_up.velocity_y;
    }
}

pub fn update_ball_previous_position(game: &mut Game) {
    for ball in game.entities.balls.iter_mut() {
        ball.prev_x = ball.x;
        ball.prev_y = ball.y;
    }
}

pub fn update_rainbow_mode(game: &mut Game) {
    if get_time() > game.timers.rainbow_mode_end_time {
        game.timers.rainbow_mode_end_time = 0.0;
    }
}

pub fn update_ball_effects(game: &mut Game) {
    for ball in game.entities.balls.iter_mut() {
        match &mut ball.ball_effect {
            BallEffect::Normal => {}
            BallEffect::Fire {
                expires_at,
                animation,
            } => {
                if get_time() > *expires_at {
                    ball.ball_effect = BallEffect::Normal;
                } else {
                    animation.update(get_frame_time());
                }
            }
        }
    }
}

// we need to do this in order to make sure the player is displayed correctly even after a resize
pub fn update_player_position(game: &mut Game) {
    game.entities.player.y = WORLD_H * 0.95;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn assert_approx_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {actual} to be close to {expected}",
        );
    }

    fn speed(velocity_x: f32, velocity_y: f32) -> f32 {
        velocity_x.hypot(velocity_y)
    }

    #[test]
    fn still_paddle_keeps_existing_bounce_behavior() {
        let (velocity_x, velocity_y) = calculate_paddle_bounce_velocity(0.2, 0.6, 0.0);

        assert_approx_eq(velocity_x, 0.2);
        assert_approx_eq(velocity_y, -0.6);
    }

    #[test]
    fn moving_right_adds_rightward_ball_velocity() {
        let (velocity_x, velocity_y) =
            calculate_paddle_bounce_velocity(0.0, 1.0, PADDLE_MOVE_DELTA);

        assert!(velocity_x > 0.0);
        assert!(velocity_y < 0.0);
    }

    #[test]
    fn moving_left_adds_leftward_ball_velocity() {
        let (velocity_x, velocity_y) =
            calculate_paddle_bounce_velocity(0.2, 1.0, -PADDLE_MOVE_DELTA);

        assert!(velocity_x < 0.2);
        assert!(velocity_y < 0.0);
    }

    #[test]
    fn moving_paddle_preserves_total_ball_speed() {
        let before = speed(0.3, 0.8);
        let (velocity_x, velocity_y) =
            calculate_paddle_bounce_velocity(0.3, 0.8, PADDLE_MOVE_DELTA);
        let after = speed(velocity_x, velocity_y);

        assert_approx_eq(after, before);
    }

    #[test]
    fn horizontal_velocity_is_clamped() {
        let speed = speed(0.8, 0.6);
        let (velocity_x, velocity_y) =
            calculate_paddle_bounce_velocity(0.8, 0.6, PADDLE_MOVE_DELTA);

        assert_approx_eq(velocity_x, speed * MAX_HORIZONTAL_BOUNCE_RATIO);
        assert!(velocity_y < 0.0);
    }
}

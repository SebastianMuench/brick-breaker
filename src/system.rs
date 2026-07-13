use crate::{
    BLOCKS_H, BLOCKS_W, PADDLE_MOVE_DELTA, WORLD_H, WORLD_W,
    entities::{
        self, BEER_FOUNTAIN_DURATION_SECONDS, BEER_FOUNTAIN_SPLASH_DURATION_SECONDS, Ball,
        BallEffect, BlockCoordinates, FallingPowerUp, GRILL_DURATION_SECONDS, PaddleEffect,
        PowerUp, Rect as EntityRect,
    },
    game::Game,
    state::GameState,
};
use macroquad::prelude::*;

const PADDLE_ENGLISH_STRENGTH: f32 = 0.45;
const MAX_HORIZONTAL_BOUNCE_RATIO: f32 = 0.85;
const PADDLE_STILL_EPSILON: f32 = 0.01;
const BEER_FOUNTAIN_PUSH_SPEED: f32 = WORLD_H * 0.003;

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
            scale_ball_speed(ball, speed_multiplier);
        }
    }
}

fn scale_ball_speed(ball: &mut Ball, speed_multiplier: f32) {
    if let Some(held) = ball.held_by_paddle.as_mut() {
        held.release_velocity_x *= speed_multiplier;
        held.release_velocity_y *= speed_multiplier;
    } else {
        ball.velocity_x *= speed_multiplier;
        ball.velocity_y *= speed_multiplier;
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
        if ball.is_held() {
            continue;
        }

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
            PowerUp::BeerFountain => {
                let power_up_size = block_rect.width * 0.8;
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0 - power_up_size / 2.0,
                    y: block_rect.y + block_rect.height / 2.0 - power_up_size / 2.0,
                    width: power_up_size,
                    height: power_up_size,
                    power_up,
                    velocity_y: WORLD_H * 0.002,
                });
            }
            PowerUp::StickyPaddle => {
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0,
                    y: block_rect.y + block_rect.height / 2.0,
                    width: block_rect.width * 1.0,
                    height: block_rect.height * 2.4,
                    power_up,
                    velocity_y: WORLD_H * 0.002,
                });
            }
            PowerUp::Grill => {
                let power_up_size = block_rect.width * 0.8;
                falling_power_ups.push(FallingPowerUp {
                    x: block_rect.x + block_rect.width / 2.0 - power_up_size / 2.0,
                    y: block_rect.y + block_rect.height / 2.0 - power_up_size / 2.0,
                    width: power_up_size,
                    height: power_up_size,
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
        if ball.is_held() {
            continue;
        }

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
        if ball.is_held() {
            continue;
        }

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

fn handle_ball_paddle_collision(
    ball: &mut Ball,
    paddle_hitbox: EntityRect,
    paddle_velocity_x: f32,
    sticky_active: bool,
) -> bool {
    if ball.is_held() {
        return false;
    }

    if ball.y + ball.radius >= paddle_hitbox.y
        && ball.y - ball.radius <= paddle_hitbox.y + paddle_hitbox.height
        && ball.x >= paddle_hitbox.x
        && ball.x <= paddle_hitbox.x + paddle_hitbox.width
        && ball.velocity_y > 0.0
    {
        if sticky_active {
            ball.hold_on_paddle(paddle_hitbox);
        } else {
            (ball.velocity_x, ball.velocity_y) = calculate_paddle_bounce_velocity(
                ball.velocity_x,
                ball.velocity_y,
                paddle_velocity_x,
            );
            // push the ball back up to the visible top surface of the paddle
            ball.y = paddle_hitbox.y - ball.radius;
        }

        return true;
    }

    false
}

pub fn handle_paddle_collision(game: &mut Game) {
    let paddle_hitbox = game
        .entities
        .player
        .active_hitbox(game.timers.beer_fountain_end_time > 0.0);
    let paddle_velocity_x = game.entities.player.velocity_x;
    let sticky_active = game.entities.player.has_sticky_effect();

    for ball in game.entities.balls.iter_mut() {
        handle_ball_paddle_collision(ball, paddle_hitbox, paddle_velocity_x, sticky_active);
    }
}

pub fn handle_beer_fountain_collision(game: &mut Game) {
    if game.timers.beer_fountain_end_time <= 0.0 {
        return;
    }

    let fountain_hitbox = game.entities.player.beer_fountain_hitbox();

    for ball in game.entities.balls.iter_mut() {
        if ball.is_held() {
            continue;
        }

        push_ball_from_beer_fountain(ball, &fountain_hitbox);
    }
}

fn push_ball_from_beer_fountain(ball: &mut Ball, fountain_hitbox: &EntityRect) -> bool {
    if ball.is_held() || !ball_overlaps_rect(ball, fountain_hitbox) {
        return false;
    }

    ball.velocity_y = ball.velocity_y.min(-BEER_FOUNTAIN_PUSH_SPEED);
    true
}

fn ball_overlaps_rect(ball: &Ball, rect: &EntityRect) -> bool {
    let closest_x = ball.x.clamp(rect.x, rect.x + rect.width);
    let closest_y = ball.y.clamp(rect.y, rect.y + rect.height);
    let distance_x = ball.x - closest_x;
    let distance_y = ball.y - closest_y;

    distance_x * distance_x + distance_y * distance_y <= ball.radius * ball.radius
}

pub fn handle_power_up_collision(game: &mut Game) {
    let fire_ball_effect_frame_count = game.textures.fire_ball_effects.len();
    let paddle_hitbox = game
        .entities
        .player
        .active_hitbox(game.timers.beer_fountain_end_time > 0.0);
    let falling_power_ups = std::mem::take(&mut game.entities.falling_power_ups);
    let mut remaining_power_ups = Vec::with_capacity(falling_power_ups.len());
    let mut collected_power_ups = Vec::new();

    for power_up in falling_power_ups {
        if power_up_collides_with_paddle(&power_up, paddle_hitbox) {
            collected_power_ups.push(power_up.power_up);
        } else if power_up.y <= WORLD_H {
            remaining_power_ups.push(power_up);
        }
    }

    game.entities.falling_power_ups = remaining_power_ups;

    for power_up in collected_power_ups {
        apply_collected_power_up(game, power_up, fire_ball_effect_frame_count);
    }
}

fn power_up_collides_with_paddle(power_up: &FallingPowerUp, paddle_hitbox: EntityRect) -> bool {
    power_up.y + power_up.height >= paddle_hitbox.y
        && power_up.y <= paddle_hitbox.y + paddle_hitbox.height
        && power_up.x + power_up.width >= paddle_hitbox.x
        && power_up.x <= paddle_hitbox.x + paddle_hitbox.width
}

fn apply_collected_power_up(
    game: &mut Game,
    power_up: PowerUp,
    fire_ball_effect_frame_count: usize,
) {
    match power_up {
        PowerUp::ExtraBall => {
            game.entities.balls.push(Ball::new());
        }
        PowerUp::PaddleExpand => {
            game.entities
                .player
                .paddle_effects
                .push(PaddleEffect::Expanded {
                    expires_at: get_time() + 30.0,
                });
        }
        PowerUp::RainbowMode => {
            game.timers.rainbow_mode_end_time = get_time() + 30.0;
        }
        PowerUp::FireBall => {
            let now = get_time();

            for ball in game.entities.balls.iter_mut() {
                ball.ball_effect = BallEffect::Fire {
                    expires_at: now + 5.0,
                    animation: entities::Animation::new(fire_ball_effect_frame_count, 0.1),
                };
            }
        }
        PowerUp::BeerFountain => {
            let now = get_time();

            game.timers.beer_fountain_end_time = now + BEER_FOUNTAIN_DURATION_SECONDS;
            game.timers.beer_fountain_splash_end_time = now + BEER_FOUNTAIN_SPLASH_DURATION_SECONDS;
        }
        PowerUp::StickyPaddle => {
            game.entities
                .player
                .paddle_effects
                .push(PaddleEffect::Sticky {
                    expires_at: get_time() + 30.0,
                });
        }
        PowerUp::Grill => {
            game.timers.grill_end_time = get_time() + GRILL_DURATION_SECONDS;
            game.entities.recalculate_power_ups(10);
        }
    }
}

pub fn check_ball_out_of_bounds(game: &mut Game) {
    game.entities
        .balls
        .retain(|ball| ball.is_held() || ball.y - ball.radius <= WORLD_H);
}

pub fn update_ball_position(game: &mut Game) {
    for ball in game.entities.balls.iter_mut() {
        if ball.is_held() {
            continue;
        }

        ball.x += ball.velocity_x;
        ball.y += ball.velocity_y;
    }
}

pub fn update_held_ball_positions(game: &mut Game) {
    let paddle_hitbox = game
        .entities
        .player
        .active_hitbox(game.timers.beer_fountain_end_time > 0.0);

    for ball in game.entities.balls.iter_mut() {
        ball.update_held_position(paddle_hitbox);
    }
}

pub fn release_sticky_balls(game: &mut Game) {
    let paddle_velocity_x = game.entities.player.velocity_x;

    for ball in game.entities.balls.iter_mut() {
        release_sticky_ball(ball, paddle_velocity_x);
    }
}

fn release_sticky_ball(ball: &mut Ball, paddle_velocity_x: f32) -> bool {
    if let Some((release_velocity_x, release_velocity_y)) = ball.release_from_paddle() {
        (ball.velocity_x, ball.velocity_y) = calculate_paddle_bounce_velocity(
            release_velocity_x,
            release_velocity_y,
            paddle_velocity_x,
        );

        return true;
    }

    false
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

pub fn update_beer_fountain(game: &mut Game) {
    let now = get_time();

    if now > game.timers.beer_fountain_end_time {
        game.timers.beer_fountain_end_time = 0.0;
    }

    if now > game.timers.beer_fountain_splash_end_time {
        game.timers.beer_fountain_splash_end_time = 0.0;
    }
}

pub fn update_grill(game: &mut Game) {
    if get_time() > game.timers.grill_end_time {
        game.timers.grill_end_time = 0.0;
        game.entities.recalculate_power_ups(5);
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

    fn ball_at(x: f32, y: f32, velocity_x: f32, velocity_y: f32) -> Ball {
        Ball {
            x,
            y,
            prev_x: x,
            prev_y: y,
            radius: 5.0,
            velocity_x,
            velocity_y,
            ball_effect: BallEffect::Normal,
            held_by_paddle: None,
        }
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

    #[test]
    fn normal_paddle_collision_keeps_existing_bounce_behavior() {
        let paddle_hitbox = EntityRect {
            x: 100.0,
            y: 200.0,
            width: 50.0,
            height: 10.0,
        };
        let mut ball = ball_at(120.0, 196.0, 0.2, 0.6);

        assert!(handle_ball_paddle_collision(
            &mut ball,
            paddle_hitbox,
            0.0,
            false
        ));

        assert!(!ball.is_held());
        assert_approx_eq(ball.velocity_x, 0.2);
        assert_approx_eq(ball.velocity_y, -0.6);
        assert_approx_eq(ball.y, paddle_hitbox.y - ball.radius);
    }

    #[test]
    fn sticky_paddle_collision_holds_downward_ball() {
        let paddle_hitbox = EntityRect {
            x: 100.0,
            y: 200.0,
            width: 50.0,
            height: 10.0,
        };
        let mut ball = ball_at(120.0, 196.0, 0.2, 0.6);

        assert!(handle_ball_paddle_collision(
            &mut ball,
            paddle_hitbox,
            0.0,
            true
        ));

        let held = ball.held_by_paddle.unwrap();
        assert_approx_eq(held.paddle_offset_x, -5.0);
        assert_approx_eq(held.release_velocity_x, 0.2);
        assert_approx_eq(held.release_velocity_y, 0.6);
        assert_approx_eq(ball.velocity_x, 0.0);
        assert_approx_eq(ball.velocity_y, 0.0);
        assert_approx_eq(ball.y, paddle_hitbox.y - ball.radius);
    }

    #[test]
    fn sticky_release_launches_held_ball_upward() {
        let paddle_hitbox = EntityRect {
            x: 100.0,
            y: 200.0,
            width: 50.0,
            height: 10.0,
        };
        let mut ball = ball_at(120.0, 196.0, 0.2, 0.6);
        ball.hold_on_paddle(paddle_hitbox);

        assert!(release_sticky_ball(&mut ball, 0.0));

        assert!(!ball.is_held());
        assert_approx_eq(ball.velocity_x, 0.2);
        assert_approx_eq(ball.velocity_y, -0.6);
    }

    #[test]
    fn held_release_velocity_scales_when_speed_increases() {
        let paddle_hitbox = EntityRect {
            x: 100.0,
            y: 200.0,
            width: 50.0,
            height: 10.0,
        };
        let mut ball = ball_at(120.0, 196.0, 0.2, 0.6);
        ball.hold_on_paddle(paddle_hitbox);

        scale_ball_speed(&mut ball, 1.5);

        let held = ball.held_by_paddle.unwrap();
        assert_approx_eq(ball.velocity_x, 0.0);
        assert_approx_eq(ball.velocity_y, 0.0);
        assert_approx_eq(held.release_velocity_x, 0.3);
        assert_approx_eq(held.release_velocity_y, 0.9);
    }

    #[test]
    fn beer_fountain_pushes_overlapping_ball_upward() {
        let hitbox = EntityRect {
            x: 100.0,
            y: 100.0,
            width: 30.0,
            height: 80.0,
        };
        let mut ball = ball_at(115.0, 170.0, 0.4, 0.8);

        assert!(push_ball_from_beer_fountain(&mut ball, &hitbox));
        assert_approx_eq(ball.velocity_x, 0.4);
        assert_approx_eq(ball.velocity_y, -BEER_FOUNTAIN_PUSH_SPEED);
    }

    #[test]
    fn beer_fountain_ignores_non_overlapping_ball() {
        let hitbox = EntityRect {
            x: 100.0,
            y: 100.0,
            width: 30.0,
            height: 80.0,
        };
        let mut ball = ball_at(180.0, 170.0, 0.4, 0.8);

        assert!(!push_ball_from_beer_fountain(&mut ball, &hitbox));
        assert_approx_eq(ball.velocity_x, 0.4);
        assert_approx_eq(ball.velocity_y, 0.8);
    }

    #[test]
    fn beer_fountain_does_not_slow_fast_upward_ball() {
        let hitbox = EntityRect {
            x: 100.0,
            y: 100.0,
            width: 30.0,
            height: 80.0,
        };
        let mut ball = ball_at(115.0, 170.0, 0.4, -BEER_FOUNTAIN_PUSH_SPEED * 2.0);

        assert!(push_ball_from_beer_fountain(&mut ball, &hitbox));
        assert_approx_eq(ball.velocity_y, -BEER_FOUNTAIN_PUSH_SPEED * 2.0);
    }

    #[test]
    fn beer_fountain_hitbox_is_centered_on_paddle() {
        let paddle = entities::Paddle {
            x: 80.0,
            y: 450.0,
            width: 100.0,
            height: 25.0,
            base_width: 100.0,
            velocity_x: 0.0,
            paddle_effects: vec![],
        };
        let hitbox = paddle.beer_fountain_hitbox();
        let paddle_center_x = paddle.x + paddle.width / 2.0;
        let hitbox_center_x = hitbox.x + hitbox.width / 2.0;

        assert_approx_eq(hitbox_center_x, paddle_center_x);
    }
}

use macroquad::rand;

use crate::{WORLD_H, WORLD_W};

const PADDLE_TEXTURE_W: f32 = 2106.0;
const PADDLE_TEXTURE_H: f32 = 747.0;
const PADDLE_VISIBLE_X: f32 = 84.0;
const PADDLE_VISIBLE_Y: f32 = 112.0;
const PADDLE_VISIBLE_W: f32 = 1961.0;
const PADDLE_VISIBLE_H: f32 = 514.0;
// assets/yachter-upright.png is assets/yachter.png rotated by -90 degrees.
const UPRIGHT_PADDLE_VISIBLE_X: f32 = PADDLE_VISIBLE_Y;
const UPRIGHT_PADDLE_VISIBLE_Y: f32 = PADDLE_TEXTURE_W - PADDLE_VISIBLE_X - PADDLE_VISIBLE_W;
const UPRIGHT_PADDLE_VISIBLE_W: f32 = PADDLE_VISIBLE_H;
const UPRIGHT_PADDLE_VISIBLE_H: f32 = PADDLE_VISIBLE_W;

pub const BEER_FOUNTAIN_DURATION_SECONDS: f64 = 20.0;
pub const BEER_FOUNTAIN_SPLASH_DURATION_SECONDS: f64 = 0.25;
pub const BEER_FOUNTAIN_FRAME_COUNT: usize = 8;
pub const BEER_FOUNTAIN_FRAME_TIME: f64 = 0.1;
pub const GRILL_DURATION_SECONDS: f64 = 20.0;
pub const CHARCOAL_BLOCK_FRAME_COUNT: usize = 8;
pub const CHARCOAL_BLOCK_FRAME_TIME: f64 = 0.12;
pub const UPRIGHT_BEER_BOTTLE_HEIGHT: f32 = WORLD_H * 0.24;
pub const UPRIGHT_BEER_BOTTLE_WIDTH: f32 = UPRIGHT_BEER_BOTTLE_HEIGHT / 3.0;
pub const BEER_FOUNTAIN_WIDTH: f32 = WORLD_W * 0.12;
pub const BEER_FOUNTAIN_HEIGHT: f32 = WORLD_H * 0.36;
pub const BEER_FOUNTAIN_HITBOX_WIDTH: f32 = WORLD_W * 0.07;
pub const BEER_FOUNTAIN_HITBOX_HEIGHT: f32 = WORLD_H * 0.34;
pub const BEER_FOUNTAIN_NECK_OVERLAP: f32 = WORLD_H * 0.012;
pub const BEER_FOUNTAIN_SPLASH_SIZE: f32 = WORLD_W * 0.08;

#[derive(Clone)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub base_width: f32,
    pub base_height: f32,
    pub velocity_x: f32,
    pub paddle_effects: Vec<PaddleEffect>,
}

#[derive(Clone)]
pub enum PaddleEffect {
    Expanded { expires_at: f64 },
    Sticky { expires_at: f64 },
}

impl PaddleEffect {
    pub fn is_sticky(&self) -> bool {
        matches!(self, PaddleEffect::Sticky { .. })
    }

    pub fn is_expired(&self, now: f64) -> bool {
        match self {
            PaddleEffect::Expanded { expires_at } => *expires_at <= now,
            PaddleEffect::Sticky { expires_at } => *expires_at <= now,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            x: WORLD_W / 2.0,
            y: WORLD_H * 0.95,
            width: WORLD_W * 0.15,
            height: WORLD_H * 0.05,
            base_width: WORLD_W * 0.15,
            base_height: WORLD_H * 0.05,
            velocity_x: 0.0,
            paddle_effects: vec![],
        }
    }

    pub fn update_paddle_effects(&mut self, now: f64) {
        self.paddle_effects.retain(|effect| !effect.is_expired(now));
        self.update_paddle_size();
    }

    pub fn has_sticky_effect(&self) -> bool {
        self.paddle_effects.iter().any(PaddleEffect::is_sticky)
    }

    fn update_paddle_size(&mut self) {
        let center_x = self.center_x();

        match self
            .paddle_effects
            .iter()
            .find(|effect| matches!(effect, PaddleEffect::Expanded { .. }))
        {
            Some(_) => {
                self.width = self.base_width * 2.0;
                self.height = self.base_height * 1.5;
            }
            None => {
                self.width = self.base_width;
                self.height = self.base_height;
            }
        }

        // self.width = self.base_width * (1.0 + 0.5 * self.expand_power_up_end_times.len() as f32);
        self.x = center_x - self.width / 2.0;
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn hitbox(&self) -> Rect {
        Rect {
            x: self.x + self.width * (PADDLE_VISIBLE_X / PADDLE_TEXTURE_W),
            y: self.y + self.height * (PADDLE_VISIBLE_Y / PADDLE_TEXTURE_H),
            width: self.width * (PADDLE_VISIBLE_W / PADDLE_TEXTURE_W),
            height: self.height * (PADDLE_VISIBLE_H / PADDLE_TEXTURE_H),
        }
    }

    pub fn side_boundary_rect(&self, beer_fountain_active: bool) -> Rect {
        if beer_fountain_active {
            self.upright_beer_bottle_hitbox()
        } else {
            self.rect()
        }
    }

    pub fn clamp_side_boundary_inside_world(&mut self, beer_fountain_active: bool) {
        let bounds = self.side_boundary_rect(beer_fountain_active);

        if bounds.x < 0.0 {
            self.x -= bounds.x;
        }

        let bounds = self.side_boundary_rect(beer_fountain_active);
        let overflow = bounds.x + bounds.width - WORLD_W;

        if overflow > 0.0 {
            self.x -= overflow;
        }
    }

    pub fn active_hitbox(&self, beer_fountain_active: bool) -> Rect {
        if beer_fountain_active {
            self.upright_beer_bottle_hitbox()
        } else {
            self.hitbox()
        }
    }

    pub fn upright_beer_bottle_rect(&self) -> Rect {
        Rect {
            x: self.center_x() - UPRIGHT_BEER_BOTTLE_WIDTH / 2.0,
            y: self.bottom() - UPRIGHT_BEER_BOTTLE_HEIGHT,
            width: UPRIGHT_BEER_BOTTLE_WIDTH,
            height: UPRIGHT_BEER_BOTTLE_HEIGHT,
        }
    }

    pub fn upright_beer_bottle_hitbox(&self) -> Rect {
        let bottle = self.upright_beer_bottle_rect();

        Rect {
            x: bottle.x + bottle.width * (UPRIGHT_PADDLE_VISIBLE_X / PADDLE_TEXTURE_H),
            y: bottle.y + bottle.height * (UPRIGHT_PADDLE_VISIBLE_Y / PADDLE_TEXTURE_W),
            width: bottle.width * (UPRIGHT_PADDLE_VISIBLE_W / PADDLE_TEXTURE_H),
            height: bottle.height * (UPRIGHT_PADDLE_VISIBLE_H / PADDLE_TEXTURE_W),
        }
    }

    pub fn beer_fountain_rect(&self) -> Rect {
        let bottle = self.upright_beer_bottle_rect();

        Rect {
            x: self.center_x() - BEER_FOUNTAIN_WIDTH / 2.0,
            y: bottle.y - BEER_FOUNTAIN_HEIGHT + BEER_FOUNTAIN_NECK_OVERLAP,
            width: BEER_FOUNTAIN_WIDTH,
            height: BEER_FOUNTAIN_HEIGHT,
        }
    }

    pub fn beer_fountain_hitbox(&self) -> Rect {
        let fountain = self.beer_fountain_rect();

        Rect {
            x: self.center_x() - BEER_FOUNTAIN_HITBOX_WIDTH / 2.0,
            y: fountain.y + fountain.height - BEER_FOUNTAIN_HITBOX_HEIGHT,
            width: BEER_FOUNTAIN_HITBOX_WIDTH,
            height: BEER_FOUNTAIN_HITBOX_HEIGHT,
        }
    }

    pub fn beer_fountain_splash_rect(&self) -> Rect {
        let bottle = self.upright_beer_bottle_rect();

        Rect {
            x: self.center_x() - BEER_FOUNTAIN_SPLASH_SIZE / 2.0,
            y: bottle.y - BEER_FOUNTAIN_SPLASH_SIZE / 2.0,
            width: BEER_FOUNTAIN_SPLASH_SIZE,
            height: BEER_FOUNTAIN_SPLASH_SIZE,
        }
    }

    fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }

    fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

#[derive(Copy, Clone)]
pub struct HeldBall {
    pub paddle_offset_x: f32,
    pub release_velocity_x: f32,
    pub release_velocity_y: f32,
}

#[derive(Copy, Clone)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub prev_x: f32,
    pub prev_y: f32,
    pub radius: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub ball_effect: BallEffect,
    pub held_by_paddle: Option<HeldBall>,
}

impl Ball {
    pub fn new() -> Self {
        // we're randomizing the velocity so balls do not move in the exact same
        // direction every time, which is kinda lame
        let speed = ((WORLD_W * 0.0015).powi(2) + (WORLD_H * 0.0015).powi(2)).sqrt();
        let angle = rand::gen_range(0.35, std::f32::consts::PI - 0.35);

        Ball {
            x: WORLD_W / 2.0,
            y: WORLD_H * 0.6,
            prev_x: WORLD_W / 2.0,
            prev_y: WORLD_H * 0.6,
            radius: WORLD_W * 0.01,
            velocity_x: speed * angle.cos(),
            velocity_y: speed * angle.sin(),
            ball_effect: BallEffect::Normal,
            held_by_paddle: None,
        }
    }

    pub fn is_held(&self) -> bool {
        self.held_by_paddle.is_some()
    }

    pub fn hold_on_paddle(&mut self, paddle_hitbox: Rect) {
        let paddle_center_x = paddle_hitbox.x + paddle_hitbox.width / 2.0;

        self.held_by_paddle = Some(HeldBall {
            paddle_offset_x: self.x - paddle_center_x,
            release_velocity_x: self.velocity_x,
            release_velocity_y: self.velocity_y,
        });
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
        self.update_held_position(paddle_hitbox);
        self.prev_x = self.x;
        self.prev_y = self.y;
    }

    pub fn update_held_position(&mut self, paddle_hitbox: Rect) {
        if let Some(held) = self.held_by_paddle {
            let paddle_center_x = paddle_hitbox.x + paddle_hitbox.width / 2.0;

            self.x = paddle_center_x + held.paddle_offset_x;
            self.y = paddle_hitbox.y - self.radius;
        }
    }

    pub fn release_from_paddle(&mut self) -> Option<(f32, f32)> {
        self.held_by_paddle
            .take()
            .map(|held| (held.release_velocity_x, held.release_velocity_y))
    }
}

#[derive(Clone, Copy)]
pub struct Block {
    pub active: bool,
    pub power_up: Option<PowerUp>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            active: true,
            power_up: Block::get_random_power_up(3),
        }
    }

    pub fn get_random_power_up(drop_chance: u32) -> Option<PowerUp> {
        let random_value: u32 = rand::gen_range(1, 100);
        if random_value < drop_chance {
            Some(PowerUp::ExtraBall)
        } else if random_value < drop_chance * 2 {
            Some(PowerUp::PaddleExpand)
        } else if random_value < drop_chance * 3 {
            Some(PowerUp::RainbowMode)
        } else if random_value < drop_chance * 4 {
            Some(PowerUp::FireBall)
        } else if random_value < drop_chance * 5 {
            Some(PowerUp::BeerFountain)
        } else if random_value < drop_chance * 6 {
            Some(PowerUp::StickyPaddle)
        } else if random_value < drop_chance * 7 {
            Some(PowerUp::Grill)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub enum PowerUp {
    ExtraBall,
    PaddleExpand,
    RainbowMode,
    FireBall,
    BeerFountain,
    StickyPaddle,
    Grill,
}

#[derive(Clone, Copy)]
pub struct FallingPowerUp {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub power_up: PowerUp,
    pub velocity_y: f32,
}

#[derive(Copy, Clone)]
pub struct BlockCoordinates {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Copy, Clone)]
pub struct Animation {
    pub frame: usize,
    pub frame_count: usize,
    pub frame_time: f64,
    pub timer: f32,
}

impl Animation {
    pub fn new(frame_count: usize, frame_time: f64) -> Self {
        Animation {
            frame: 0,
            frame_count,
            frame_time,
            timer: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.timer += delta_time;
        if self.timer >= self.frame_time as f32 {
            self.timer = 0.0;
            self.frame = (self.frame + 1) % self.frame_count;
        }
    }
}

#[derive(Copy, Clone)]
pub enum BallEffect {
    Normal,
    Fire {
        expires_at: f64,
        animation: Animation,
    },
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

    fn assert_rect_approx_eq(actual: Rect, expected: Rect) {
        assert_approx_eq(actual.x, expected.x);
        assert_approx_eq(actual.y, expected.y);
        assert_approx_eq(actual.width, expected.width);
        assert_approx_eq(actual.height, expected.height);
    }

    fn new() -> Paddle {
        Paddle {
            x: 80.0,
            y: 450.0,
            width: 100.0,
            height: 25.0,
            base_width: 100.0,
            base_height: 25.0,
            velocity_x: 0.0,
            paddle_effects: vec![],
        }
    }

    #[test]
    fn sticky_effect_presence_uses_unexpired_paddle_effects() {
        let mut paddle = new();

        assert!(!paddle.has_sticky_effect());

        paddle
            .paddle_effects
            .push(PaddleEffect::Sticky { expires_at: 5.0 });
        assert!(paddle.has_sticky_effect());

        paddle.update_paddle_effects(6.0);
        assert!(!paddle.has_sticky_effect());
    }

    #[test]
    fn held_ball_follows_paddle_hitbox() {
        let mut ball = Ball {
            x: 115.0,
            y: 120.0,
            prev_x: 115.0,
            prev_y: 120.0,
            radius: 5.0,
            velocity_x: 0.4,
            velocity_y: 0.8,
            ball_effect: BallEffect::Normal,
            held_by_paddle: None,
        };
        let first_hitbox = Rect {
            x: 100.0,
            y: 200.0,
            width: 50.0,
            height: 10.0,
        };
        let next_hitbox = Rect {
            x: 130.0,
            y: 210.0,
            width: 50.0,
            height: 10.0,
        };

        ball.hold_on_paddle(first_hitbox);
        assert!(ball.is_held());
        assert_approx_eq(ball.velocity_x, 0.0);
        assert_approx_eq(ball.velocity_y, 0.0);
        assert_approx_eq(ball.x, 115.0);
        assert_approx_eq(ball.y, first_hitbox.y - ball.radius);

        ball.update_held_position(next_hitbox);
        assert_approx_eq(ball.x, 145.0);
        assert_approx_eq(ball.y, next_hitbox.y - ball.radius);

        let (release_velocity_x, release_velocity_y) = ball.release_from_paddle().unwrap();
        assert_approx_eq(release_velocity_x, 0.4);
        assert_approx_eq(release_velocity_y, 0.8);
        assert!(!ball.is_held());
    }

    #[test]
    fn upright_beer_bottle_hitbox_rotates_regular_visible_bounds() {
        let paddle = new();
        let bottle = paddle.upright_beer_bottle_rect();
        let hitbox = paddle.upright_beer_bottle_hitbox();

        assert_approx_eq(
            hitbox.x,
            bottle.x + bottle.width * (PADDLE_VISIBLE_Y / PADDLE_TEXTURE_H),
        );
        assert_approx_eq(
            hitbox.y,
            bottle.y
                + bottle.height
                    * ((PADDLE_TEXTURE_W - PADDLE_VISIBLE_X - PADDLE_VISIBLE_W) / PADDLE_TEXTURE_W),
        );
        assert_approx_eq(
            hitbox.width,
            bottle.width * (PADDLE_VISIBLE_H / PADDLE_TEXTURE_H),
        );
        assert_approx_eq(
            hitbox.height,
            bottle.height * (PADDLE_VISIBLE_W / PADDLE_TEXTURE_W),
        );
    }

    #[test]
    fn active_hitbox_uses_upright_bottle_while_beer_fountain_is_active() {
        let paddle = new();

        assert_rect_approx_eq(paddle.active_hitbox(false), paddle.hitbox());
        assert_rect_approx_eq(
            paddle.active_hitbox(true),
            paddle.upright_beer_bottle_hitbox(),
        );
    }

    #[test]
    fn side_boundary_clamp_keeps_regular_paddle_bounds_when_inactive() {
        let mut paddle = new();

        paddle.x = -10.0;
        paddle.clamp_side_boundary_inside_world(false);
        assert_approx_eq(paddle.rect().x, 0.0);

        paddle.x = WORLD_W - paddle.width + 10.0;
        paddle.clamp_side_boundary_inside_world(false);
        assert_approx_eq(paddle.rect().x + paddle.rect().width, WORLD_W);
    }

    #[test]
    fn side_boundary_clamp_uses_upright_bottle_body_during_beer_fountain() {
        let mut paddle = new();

        paddle.x = -100.0;
        paddle.clamp_side_boundary_inside_world(true);
        assert_approx_eq(paddle.upright_beer_bottle_hitbox().x, 0.0);
        assert!(paddle.rect().x < 0.0);

        paddle.x = WORLD_W;
        paddle.clamp_side_boundary_inside_world(true);
        let hitbox = paddle.upright_beer_bottle_hitbox();
        assert_approx_eq(hitbox.x + hitbox.width, WORLD_W);
        assert!(paddle.rect().x > WORLD_W - paddle.rect().width);
    }
}

use macroquad::{rand, time::get_time};

use crate::{WORLD_H, WORLD_W};

const PADDLE_TEXTURE_W: f32 = 2106.0;
const PADDLE_TEXTURE_H: f32 = 747.0;
const PADDLE_VISIBLE_X: f32 = 84.0;
const PADDLE_VISIBLE_Y: f32 = 112.0;
const PADDLE_VISIBLE_W: f32 = 1961.0;
const PADDLE_VISIBLE_H: f32 = 514.0;

#[derive(Clone)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub base_width: f32,
    pub expanded_until: Vec<f64>,
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
            y: WORLD_H * 0.9,
            width: WORLD_W * 0.15,
            height: WORLD_H * 0.05,
            base_width: WORLD_W * 0.15,
            expanded_until: Vec::new(),
        }
    }

    pub fn apply_size_increases(&mut self) {
        self.expanded_until.retain(|&time| time > get_time());
        self.width = self.base_width * (1.0 + 0.5 * self.expanded_until.len() as f32);
    }

    pub fn hitbox(&self) -> Rect {
        Rect {
            x: self.x + self.width * (PADDLE_VISIBLE_X / PADDLE_TEXTURE_W),
            y: self.y + self.height * (PADDLE_VISIBLE_Y / PADDLE_TEXTURE_H),
            width: self.width * (PADDLE_VISIBLE_W / PADDLE_TEXTURE_W),
            height: self.height * (PADDLE_VISIBLE_H / PADDLE_TEXTURE_H),
        }
    }
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
}

impl Ball {
    pub fn new() -> Self {
        // we're randomizing the velocity so balls do not move in the exact same
        // direction every time, which is kinda lame
        let speed = ((WORLD_W * 0.001).powi(2) + (WORLD_H * 0.001).powi(2)).sqrt();
        let angle = rand::gen_range(0.35, std::f32::consts::PI - 0.35);

        Ball {
            x: WORLD_W / 2.0,
            y: WORLD_H * 0.6,
            prev_x: WORLD_W / 2.0,
            prev_y: WORLD_H * 0.6,
            radius: WORLD_W * 0.01,
            velocity_x: speed * angle.cos(),
            velocity_y: speed * angle.sin(),
        }
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
            power_up: {
                let random_value: u32 = rand::gen_range(1, 100);
                if random_value < 10 {
                    Some(PowerUp::ExtraBall)
                } else if random_value < 20 {
                    Some(PowerUp::PaddleExpand)
                } else {
                    None
                }
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum PowerUp {
    ExtraBall,
    PaddleExpand,
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

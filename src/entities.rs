use macroquad::rand;

use crate::{WORLD_H, WORLD_W};

#[derive(Copy, Clone)]
pub struct Paddle {
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
            width: WORLD_W * 0.1,
            height: WORLD_H * 0.03,
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
        Ball {
            x: WORLD_W / 2.0,
            y: WORLD_H * 0.6,
            prev_x: WORLD_W / 2.0,
            prev_y: WORLD_H * 0.6,
            radius: WORLD_W * 0.01,
            velocity_x: WORLD_W * 0.001,
            velocity_y: WORLD_H * 0.001,
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
enum PowerUp {
    ExtraBall,
    PaddleExpand,
}

#[derive(Copy, Clone)]
pub struct BlockCoordinates {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

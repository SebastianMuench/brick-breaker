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

#[derive(Copy, Clone)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

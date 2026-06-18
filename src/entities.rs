use macroquad::window::{screen_height, screen_width};

#[derive(Copy, Clone)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
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
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            prev_x: screen_width() / 2.0,
            prev_y: screen_height() / 2.0,
            radius: 10.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
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

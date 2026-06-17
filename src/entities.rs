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

#[derive(Copy, Clone)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

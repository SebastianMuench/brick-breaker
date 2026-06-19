use crate::{
    BLOCKS_H, BLOCKS_W, SPEED_MULTIPLYER, WORLD_W,
    draw::{draw_game, draw_game_over, draw_game_won, draw_pause, draw_start, set_game_camera},
    entities::{Ball, Block, Paddle},
    state::GameState::{self, StartScreen},
    system::{
        check_ball_out_of_bounds, game_lost, game_won, handle_block_collision,
        handle_paddle_collision, handle_site_collision, handle_top_collision, increase_speed,
        toggle_game, update_ball_position, update_ball_previous_position, update_player_position,
    },
};

use macroquad::{
    camera::set_default_camera,
    input::{KeyCode, is_key_down, is_key_pressed},
    texture::Texture2D,
    time::get_time,
};

#[derive(Clone)]
pub struct Game {
    pub player: Paddle,
    pub paddle_texture: Texture2D,
    pub balls: Vec<Ball>,
    pub blocks: [[Block; BLOCKS_W]; BLOCKS_H],
    pub state: GameState,
    pub next_speed_increase_time: f64,
}

impl Game {
    pub fn new(paddle_texture: Texture2D) -> Self {
        Game {
            player: Paddle::new(),
            paddle_texture,
            balls: vec![Ball::new()],
            // creating a 2d array of block
            // the inner array creates BLOCKS_W times a boolean true and the outer array then creates
            // BLOCKS_H * that 10 boolean array
            blocks: [[Block::new(); BLOCKS_W]; BLOCKS_H],
            state: StartScreen,
            next_speed_increase_time: get_time() + 10.0,
        }
    }

    fn reset(&mut self) {
        // we need to reset all fields except the textures to default in order to avoid reloading the
        // textures everytime we want to restart the game
        self.player = Paddle::new();
        self.balls = vec![Ball::new()];
        self.blocks = [[Block::new(); BLOCKS_W]; BLOCKS_H];
        self.state = GameState::Playing;
        self.next_speed_increase_time = get_time() + 10.0;
    }

    pub fn update(&mut self) {
        toggle_game(self);

        match self.state {
            GameState::StartScreen => {
                if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => self.update_playing(),
            GameState::Pause => {
                if is_key_pressed(KeyCode::P) {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    Game::reset(self);
                }
            }
            GameState::GameWon => {
                if is_key_pressed(KeyCode::Space) {
                    Game::reset(self);
                }
            }
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::StartScreen => {
                self.draw_playing();
                set_default_camera();
                draw_start();
            }
            GameState::Playing => self.draw_playing(),
            GameState::Pause => {
                self.draw_playing();
                set_default_camera();
                draw_pause();
            }
            GameState::GameOver => {
                self.draw_playing();
                set_default_camera();
                draw_game_over();
            }
            GameState::GameWon => {
                self.draw_playing();
                set_default_camera();
                draw_game_won();
            }
        }
    }

    pub fn update_playing(&mut self) {
        self.handle_input();
        self.update_entities();
        self.handle_collision();
        self.update_game_state();
    }

    pub fn update_game_state(&mut self) {
        if game_lost(&self.balls) {
            self.state = GameState::GameOver;
        } else if game_won(self) {
            self.state = GameState::GameWon;
        }
    }

    pub fn handle_collision(&mut self) {
        handle_site_collision(self);
        handle_top_collision(self);
        handle_paddle_collision(self);
        handle_block_collision(self);
    }

    fn update_entities(&mut self) {
        update_player_position(self);
        update_ball_previous_position(self);
        update_ball_position(self);
        check_ball_out_of_bounds(self);
        increase_speed(self, SPEED_MULTIPLYER);
    }

    fn handle_input(&mut self) {
        const DELTA: f32 = WORLD_W * 0.01;

        if is_key_down(KeyCode::Left) {
            if self.player.x - DELTA < 0.0 {
                self.player.x = 0.0;
            } else {
                self.player.x -= DELTA;
            }
        }

        if is_key_down(KeyCode::Right) {
            if self.player.x + self.player.width + DELTA > WORLD_W {
                self.player.x = WORLD_W - self.player.width;
            } else {
                self.player.x += DELTA;
            }
        }
    }

    pub fn draw_playing(&self) {
        set_game_camera();
        draw_game(self);
    }
}

use crate::{
    BLOCKS_H, BLOCKS_W, SPEED_MULTIPLYER,
    draw::{draw_game, draw_game_over, draw_game_won, draw_pause, draw_start},
    entities::{Ball, Paddle},
    state::GameState::{self, StartScreen},
    system::{
        check_ball_out_of_bounds, game_lost, game_won, handle_block_collision,
        handle_paddle_collision, handle_site_collision, handle_top_collision, increase_speed,
        toggle_game, update_ball_position, update_ball_previous_position,
    },
};
use macroquad::{
    input::{KeyCode, is_key_down, is_key_pressed},
    time::get_time,
    window::{screen_height, screen_width},
};

#[derive(Clone)]
pub struct Game {
    pub player: Paddle,
    pub balls: Vec<Ball>,
    pub blocks: [[bool; BLOCKS_W]; BLOCKS_H],
    pub state: GameState,
    pub next_speed_increase_time: f64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: Paddle {
                x: screen_width() / 2.0,
                y: screen_height() - 20.0,
                width: 100.0,
            },
            balls: vec![
                Ball::new(),
                Ball {
                    x: screen_width() / 1.75,
                    y: screen_height() / 1.75,
                    prev_x: screen_width() / 1.75,
                    prev_y: screen_height() / 1.75,
                    radius: 10.0,
                    velocity_x: 1.0,
                    velocity_y: 1.0,
                },
                Ball {
                    x: screen_width() / 1.65,
                    y: screen_height() / 1.65,
                    prev_x: screen_width() / 1.65,
                    prev_y: screen_height() / 1.65,
                    radius: 10.0,
                    velocity_x: 1.0,
                    velocity_y: 1.0,
                },
            ],
            // creating a 2d array of block
            // the inner array creates BLOCKS_W times a boolean true and the outer array then creates
            // BLOCKS_H * that 10 boolean array
            blocks: [[true; BLOCKS_W]; BLOCKS_H],
            state: StartScreen,
            next_speed_increase_time: get_time() + 10.0,
        }
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
                    *self = Game::new();
                }
            }
            GameState::GameWon => {
                if is_key_pressed(KeyCode::Space) {
                    *self = Game::new();
                    self.state = GameState::Playing;
                }
            }
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::StartScreen => {
                draw_start();
            }
            GameState::Playing => self.draw_playing(),
            GameState::Pause => {
                draw_pause();
            }
            GameState::GameOver => {
                draw_game_over();
            }
            GameState::GameWon => {
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
        update_ball_previous_position(self);
        update_ball_position(self);
        check_ball_out_of_bounds(self);
        increase_speed(self, SPEED_MULTIPLYER);
    }

    fn handle_input(&mut self) {
        if is_key_down(KeyCode::Left) {
            if self.player.x - 5.0 < 0.0 {
                self.player.x = 0.0;
            } else {
                self.player.x -= 5.0;
            }
        }

        if is_key_down(KeyCode::Right) {
            if self.player.x + self.player.width + 5.0 > screen_width() {
                self.player.x = screen_width() - self.player.width;
            } else {
                self.player.x += 5.0;
            }
        }

        if is_key_down(KeyCode::C) {
            for row in self.blocks.iter_mut() {
                for block in row.iter_mut() {
                    *block = false;
                }
            }
        }
    }

    pub fn draw_playing(&self) {
        draw_game(self);
    }
}

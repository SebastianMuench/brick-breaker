use crate::{
    BLOCKS_H, BLOCKS_W, PADDLE_MOVE_DELTA, SPEED_MULTIPLYER, WORLD_W,
    draw::{draw_game, draw_game_over, draw_game_won, draw_pause, draw_start, set_game_camera},
    entities::{Ball, Block, FallingPowerUp, Paddle},
    state::GameState::{self, StartScreen},
    system::{
        check_ball_out_of_bounds, game_lost, game_won, handle_block_collision,
        handle_paddle_collision, handle_power_up_collision, handle_site_collision,
        handle_top_collision, increase_speed, toggle_game, update_ball_position,
        update_ball_previous_position, update_falling_power_ups_position, update_player_position,
        update_rainbow_mode,
    },
};

use macroquad::{
    camera::set_default_camera,
    input::{KeyCode, is_key_down, is_key_pressed},
    texture::Texture2D,
    time::get_time,
};

#[derive(Clone)]
pub struct GameEntities {
    pub player: Paddle,
    pub balls: Vec<Ball>,
    pub blocks: [[Block; BLOCKS_W]; BLOCKS_H],
    pub falling_power_ups: Vec<FallingPowerUp>,
}

fn new_blocks() -> [[Block; BLOCKS_W]; BLOCKS_H] {
    std::array::from_fn(|_| std::array::from_fn(|_| Block::new()))
}

impl GameEntities {
    fn new() -> Self {
        GameEntities {
            player: Paddle::new(),
            balls: vec![Ball::new()],
            blocks: new_blocks(),
            falling_power_ups: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct GameTextures {
    pub paddle: Texture2D,
    pub block: Texture2D,
    pub ball: Texture2D,
    pub extra_ball_power_up: Texture2D,
    pub paddle_expand_power_up: Texture2D,
    pub rainbow_mode_power_up: Texture2D,
}

impl GameTextures {
    pub fn new(
        paddle: Texture2D,
        block: Texture2D,
        ball: Texture2D,
        extra_ball_power_up: Texture2D,
        paddle_expand_power_up: Texture2D,
        rainbow_mode_power_up: Texture2D,
    ) -> Self {
        GameTextures {
            paddle,
            block,
            ball,
            extra_ball_power_up,
            paddle_expand_power_up,
            rainbow_mode_power_up,
        }
    }
}

#[derive(Clone)]
pub struct GameTimers {
    pub next_speed_increase_time: f64,
    pub rainbow_mode_end_time: f64,
}

impl GameTimers {
    fn new() -> Self {
        GameTimers {
            next_speed_increase_time: get_time() + 10.0,
            rainbow_mode_end_time: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct Game {
    pub entities: GameEntities,
    pub textures: GameTextures,
    pub state: GameState,
    pub timers: GameTimers,
}

impl Game {
    pub fn new(textures: GameTextures) -> Self {
        Game {
            entities: GameEntities::new(),
            textures,
            state: StartScreen,
            timers: GameTimers::new(),
        }
    }

    fn reset(&mut self) {
        // we need to reset all fields except the textures to default in order to avoid reloading the
        // textures everytime we want to restart the game
        self.entities = GameEntities::new();
        self.state = GameState::Playing;
        self.timers = GameTimers::new();
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
        if game_lost(&self.entities.balls) {
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
        handle_power_up_collision(self);
    }

    fn update_entities(&mut self) {
        update_rainbow_mode(self);
        update_player_position(self);
        update_falling_power_ups_position(self);
        self.entities.player.apply_size_increases();
        update_ball_previous_position(self);
        update_ball_position(self);
        check_ball_out_of_bounds(self);
        increase_speed(self, SPEED_MULTIPLYER);
    }

    fn handle_input(&mut self) {
        let player = &mut self.entities.player;
        let previous_x = player.x;

        if is_key_down(KeyCode::Left) {
            if player.x - PADDLE_MOVE_DELTA < 0.0 {
                player.x = 0.0;
            } else {
                player.x -= PADDLE_MOVE_DELTA;
            }
        }

        if is_key_down(KeyCode::Right) {
            if player.x + player.width + PADDLE_MOVE_DELTA > WORLD_W {
                player.x = WORLD_W - player.width;
            } else {
                player.x += PADDLE_MOVE_DELTA;
            }
        }

        player.velocity_x = player.x - previous_x;
    }

    pub fn draw_playing(&self) {
        set_game_camera();
        draw_game(self);
    }
}

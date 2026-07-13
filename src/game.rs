use crate::{
    BLOCKS_H, BLOCKS_W, PADDLE_MOVE_DELTA, SPEED_MULTIPLYER,
    draw::{draw_game, draw_game_over, draw_game_won, draw_pause, draw_start, set_game_camera},
    entities::{Ball, Block, FallingPowerUp, Paddle},
    state::GameState::{self, StartScreen},
    system::*,
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

    pub fn recalculate_power_ups(&mut self, drop_chance: u32) {
        for block_row in self.blocks.iter_mut() {
            for block in block_row.iter_mut() {
                block.power_up = Block::get_random_power_up(drop_chance);
            }
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
    pub fire_ball_power_up: Texture2D,
    pub fire_ball_effects: Vec<Texture2D>,
    pub beer_fountain_sheet: Texture2D,
    pub beer_fountain_splash: Texture2D,
    pub upright_beer_bottle: Texture2D,
    pub sticky_paddle_power_up: Texture2D,
    pub grill_power_up: Texture2D,
    pub charcoal_block_sheet: Texture2D,
}

#[derive(Clone)]
pub struct GameTimers {
    pub next_speed_increase_time: f64,
    pub rainbow_mode_end_time: f64,
    pub beer_fountain_end_time: f64,
    pub beer_fountain_splash_end_time: f64,
    pub grill_end_time: f64,
}

impl GameTimers {
    fn new() -> Self {
        GameTimers {
            next_speed_increase_time: get_time() + 10.0,
            rainbow_mode_end_time: 0.0,
            beer_fountain_end_time: 0.0,
            beer_fountain_splash_end_time: 0.0,
            grill_end_time: 0.0,
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
        handle_beer_fountain_collision(self);
        handle_block_collision(self);
        handle_power_up_collision(self);
    }

    fn update_entities(&mut self) {
        update_rainbow_mode(self);
        update_beer_fountain(self);
        update_grill(self);
        update_ball_effects(self);
        update_player_position(self);
        self.entities.player.update_paddle_effects(get_time());
        update_falling_power_ups_position(self);
        self.entities
            .player
            .clamp_side_boundary_inside_world(self.timers.beer_fountain_end_time > 0.0);
        update_held_ball_positions(self);
        update_ball_previous_position(self);
        update_ball_position(self);
        check_ball_out_of_bounds(self);
        increase_speed(self, SPEED_MULTIPLYER);
    }

    fn handle_input(&mut self) {
        let beer_fountain_active = self.timers.beer_fountain_end_time > get_time();
        let previous_x = self.entities.player.x;

        {
            let player = &mut self.entities.player;

            if is_key_down(KeyCode::Left) {
                player.x -= PADDLE_MOVE_DELTA;
                player.clamp_side_boundary_inside_world(beer_fountain_active);
            }

            if is_key_down(KeyCode::Right) {
                player.x += PADDLE_MOVE_DELTA;
                player.clamp_side_boundary_inside_world(beer_fountain_active);
            }

            player.clamp_side_boundary_inside_world(beer_fountain_active);
            player.velocity_x = player.x - previous_x;
        }

        if is_key_pressed(KeyCode::Space) {
            release_sticky_balls(self);
        }
    }

    pub fn draw_playing(&self) {
        set_game_camera();
        draw_game(self);
    }
}

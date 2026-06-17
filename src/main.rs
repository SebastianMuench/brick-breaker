use macroquad::prelude::*;

const BLOCKS_W: usize = 10;
const BLOCKS_H: usize = 10;
const SPEED_MULTIPLYER: f32 = 1.1;

#[macroquad::main("Paddle Game")]
async fn main() {
    let mut game = Game {
        player: Paddle {
            x: screen_width() / 2.0,
            y: screen_height() - 20.0,
            width: 100.0,
        },
        ball: Ball {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            radius: 10.0,
            velocity_x: 1.0,
            velocity_y: 1.0,
        },
        // creating a 2d array of block
        // the inner array creates BLOCKS_W times a boolean true and the outer array then creates
        // BLOCKS_H * that 10 boolean array
        blocks: [[true; BLOCKS_W]; BLOCKS_H],
        state: GameState::StartScreen,
        next_speed_increase_time: get_time() + 10.0,
    };

    loop {
        clear_background(BLACK);

        // p to pause and unpause the game
        toggle_game(&mut game);

        match game.state {
            GameState::StartScreen => {
                draw_start_screen();

                if is_key_pressed(KeyCode::Space) {
                    game.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                game.update();
                game.draw();

                if game_lost(&game.ball) {
                    game.state = GameState::GameOver;
                }

                if game_won(&game) {
                    game.state = GameState::GameWon;
                }
            }
            GameState::Pause => {
                draw_pause_screen();
            }
            GameState::GameOver => {
                draw_text(
                    "Game Over! Press Space to Restart",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    30.0,
                    WHITE,
                );

                if is_key_pressed(KeyCode::Space) {
                    // reset the game state
                    reset_game(&mut game);
                }
            }
            GameState::GameWon => {
                draw_text(
                    "You Won! Press Space to Restart",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    30.0,
                    WHITE,
                );

                if is_key_down(KeyCode::Space) {
                    reset_game(&mut game);
                }
            }
        }

        next_frame().await
    }
}

fn reset_game(game: &mut Game) {
    game.blocks = [[true; BLOCKS_W]; BLOCKS_H];
    game.player.x = screen_width() / 2.0;
    game.ball.x = screen_width() / 2.0;
    game.ball.y = screen_height() / 2.0;
    game.ball.velocity_x = 1.0;
    game.ball.velocity_y = 1.0;
    game.next_speed_increase_time = get_time() + 10.0;

    game.state = GameState::Playing;
}

fn toggle_game(game: &mut Game) {
    if is_key_pressed(KeyCode::P) {
        game.state = match game.state {
            GameState::Playing => GameState::Pause,
            GameState::Pause => GameState::Playing,
            s => s,
        };
    }
}

fn draw_pause_screen() {
    draw_text(
        "Game Paused! Press P to Resume",
        screen_width() / 2.0 - 150.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );
}

fn cheat(game: &mut Game) {
    if is_key_down(KeyCode::C) {
        for row in game.blocks.iter_mut() {
            for block in row.iter_mut() {
                *block = false;
            }
        }
    }
}

fn increase_speed(game: &mut Game, speed_multiplier: f32) {
    // every 10 seconds, increase the speed of the ball by 10% // this currently isn't working but i'm not sure why
    if get_time() > game.next_speed_increase_time {
        game.next_speed_increase_time += 10.0;
        game.ball.velocity_x *= speed_multiplier;
        game.ball.velocity_y *= speed_multiplier;
    }
}

fn show_velocity(ball: &Ball) {
    draw_text(
        &format!("Velocity: ({:.2}, {:.2})", ball.velocity_x, ball.velocity_y),
        10.0,
        20.0,
        20.0,
        WHITE,
    );
}

fn apply_movement(paddle: &mut Paddle) {
    if is_key_down(KeyCode::Left) {
        if paddle.x - 5.0 < 0.0 {
            paddle.x = 0.0;
        } else {
            paddle.x -= 5.0;
        }
    }

    if is_key_down(KeyCode::Right) {
        if paddle.x + paddle.width + 5.0 > screen_width() {
            paddle.x = screen_width() - paddle.width;
        } else {
            paddle.x += 5.0;
        }
    }
}

fn draw_start_screen() {
    draw_text(
        "Press Space to Start",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        30.0,
        WHITE,
    );
}

fn game_won(game: &Game) -> bool {
    // if all blocks are false, we won
    for row in game.blocks.iter().take(BLOCKS_H) {
        for block in row.iter().take(BLOCKS_W) {
            if *block {
                return false;
            }
        }
    }

    true
}

fn game_lost(ball: &Ball) -> bool {
    // if the ball goes below the paddle, we lose
    ball.y - ball.radius > screen_height()
}

fn apply_block_collision(game: &mut Game) {
    for (j, row) in game.blocks.iter_mut().enumerate().take(BLOCKS_H) {
        for (i, block) in row.iter_mut().enumerate().take(BLOCKS_W) {
            if *block {
                let block_width = screen_width() / BLOCKS_W as f32;
                let block_height = screen_height() / (2.0 * BLOCKS_H as f32);
                // check for collision with the ball
                if game.ball.x + game.ball.radius >= i as f32 * block_width
                    && game.ball.x - game.ball.radius <= (i + 1) as f32 * block_width
                    && game.ball.y + game.ball.radius >= j as f32 * block_height
                    && game.ball.y - game.ball.radius <= (j + 1) as f32 * block_height
                {
                    *block = false;
                    game.ball.velocity_y = -game.ball.velocity_y;
                }
            }
        }
    }
}

fn draw_game(game: &Game) {
    draw_line(
        game.player.x,
        game.player.y,
        game.player.x + game.player.width,
        game.player.y,
        5.0,
        WHITE,
    );

    draw_circle(game.ball.x, game.ball.y, game.ball.radius, WHITE);

    for (j, row) in game.blocks.iter().enumerate().take(BLOCKS_H) {
        for (i, block) in row.iter().enumerate().take(BLOCKS_W) {
            if *block {
                let block_width = screen_width() / BLOCKS_W as f32;
                let block_height = screen_height() / (2.0 * BLOCKS_H as f32);
                draw_rectangle(
                    i as f32 * block_width,
                    j as f32 * block_height,
                    block_width - 2.0,
                    block_height - 2.0,
                    WHITE,
                );
            }
        }
    }
}

fn apply_ball_movement(ball: &mut Ball, player: &Paddle) {
    // we need to check if we hit the sites and then invert the velocity
    if ball.x - ball.radius <= 0.0 || ball.x + ball.radius >= screen_width() {
        ball.velocity_x = -ball.velocity_x;
    }

    // we also need to invert the y movement if we hit the top of the screen
    if ball.y - ball.radius <= 0.0 {
        ball.velocity_y = -ball.velocity_y;
    }

    // on collision with the paddle, we also need to invert the y movement
    if ball.y + ball.radius >= player.y && ball.x >= player.x && ball.x <= player.x + player.width {
        ball.velocity_y = -ball.velocity_y;
    }

    ball.x += ball.velocity_x;
    ball.y += ball.velocity_y;
}

#[derive(Copy, Clone)]
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
}

#[derive(Copy, Clone)]
struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    velocity_x: f32,
    velocity_y: f32,
}

#[derive(Copy, Clone)]
struct Game {
    player: Paddle,
    ball: Ball,
    blocks: [[bool; BLOCKS_W]; BLOCKS_H],
    state: GameState,
    next_speed_increase_time: f64,
}

impl Game {
    fn update(&mut self) {
        show_velocity(&self.ball);
        apply_movement(&mut self.player);
        apply_ball_movement(&mut self.ball, &self.player);
        apply_block_collision(self);
        increase_speed(self, SPEED_MULTIPLYER);
        cheat(self);
    }

    fn draw(&self) {
        draw_game(self);
    }
}

#[derive(Copy, Clone)]
enum GameState {
    StartScreen,
    Playing,
    Pause,
    GameOver,
    GameWon,
}

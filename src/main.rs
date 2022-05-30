mod snake;
mod food;
mod prelude {
    pub const TILE_WIDTH: i8 = 40;
    pub const BOARD_SQUARES_WIDTH: u8 = 20;
    pub const BOARD_SQUARES_HEIGHT: u8 = 15;
    pub use ggez::{Context, ContextBuilder, GameResult};
    pub use ggez::graphics::{self, Color};
    pub use ggez::event::{self, EventHandler, KeyCode, KeyMods};
    pub use ggez::timer;
    pub use crate::snake::*;
    pub use crate::food::*;
}

use prelude::*;

struct GameState {
    score: u32,
    snake: Snake,
    direction: SnakeDirection,
    food: Food
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {
            score: 0,
            snake: Snake::new(),
            direction: SnakeDirection::Down,
            food: Food::new()
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while !timer::check_update_time(ctx, 16) {
            return Ok(());
        }

        let movement_result = self.snake.move_to_direction(self.direction);
        match movement_result {
            Ok(position) => {
                if self.food.compare_position(position) {
                    self.score += 1;
                    self.food.update_position(&self.snake);
                } else {
                    self.snake.remove_tail();
                }
            },
            Err(_) => {
                reset_game(self);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        self.snake.render(ctx);
        self.food.render(ctx);
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => self.direction = SnakeDirection::Up,
            KeyCode::Down => self.direction = SnakeDirection::Down,
            KeyCode::Left => self.direction = SnakeDirection::Left,
            KeyCode::Right => self.direction = SnakeDirection::Right,
            _ => {}
        }
    }

}

fn reset_game(state: &mut GameState) {
    state.score = 0;
    state.snake = Snake::new();
    state.direction = SnakeDirection::Down;
    state.food = Food::new();
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Simple Rust Snake", "Sergio Ornaque")
        .build()
        .expect("Error creating ggez context");
    let my_game = GameState::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}

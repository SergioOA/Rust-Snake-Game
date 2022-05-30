use crate::prelude::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct Food {
    position: Vec<i8>,
}

impl Food {
    pub fn new() -> Self {
        Self {
            position: random_position(),
        }
    }

    pub fn update_position(&mut self, snake: &Snake) {
        let mut random_square: Vec<i8> = random_position();
        while snake.contains_square(random_square[0] as u8, random_square[1] as u8) {
            random_square = random_position();
        }
        self.position[0] = random_square[0] as i8;
        self.position[1] = random_square[1] as i8;
    }

    pub fn compare_position(&self, other: Vec<i8>) -> bool {
        return self.position[0] == other[0] && self.position[1] == other[1];
    }

    pub fn render(&self, ctx: &mut Context) {
        let color: Color = Color::new(1.0, 0.0, 0.0, 1.0);
        let bounds = graphics::Rect::new_i32(
            self.position[0] as i32 * TILE_WIDTH as i32,
            self.position[1] as i32 * TILE_WIDTH as i32,
            TILE_WIDTH as i32,
            TILE_WIDTH as i32,
        );
        let drawing =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds, color).unwrap();
        let _ = graphics::draw(ctx, &drawing, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
    }
}

pub fn random_position() -> Vec<i8> {
    let mut rng = thread_rng();
    let x: i8 = rng.gen_range(0..(BOARD_SQUARES_WIDTH - 1)) as i8;
    let y: i8 = rng.gen_range(0..(BOARD_SQUARES_HEIGHT - 1)) as i8;
    vec![x, y]
}

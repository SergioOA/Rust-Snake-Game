use crate::prelude::*;
use std::collections::LinkedList;

#[derive(Clone, Copy)]
pub enum SnakeDirection {
    Up,
    Left,
    Down,
    Right,
}

impl SnakeDirection {
    fn to_vector(&self) -> Vec<i8> {
        match &self {
            SnakeDirection::Up => vec![0, -1],
            SnakeDirection::Down => vec![0, 1],
            SnakeDirection::Left => vec![-1, 0],
            SnakeDirection::Right => vec![1, 0],
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    body_list: LinkedList<Vec<i8>>,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body_list: LinkedList::from([vec![1, 1]]),
        }
    }

    pub fn contains_square(&self, x: u8, y: u8) -> bool {
        for element in self.body_list.iter() {
            if element[0] == x as i8 && element[1] == y as i8 {
                return true;
            }
        }
        return false;
    }

    fn move_to(&mut self, x: i8, y: i8) {
        self.body_list.push_front(vec![x, y]);
    }

    pub fn move_to_direction(&mut self, direction: SnakeDirection) -> Result<Vec<i8>, String> {
        let head_node = self.body_list.front().unwrap();
        let offset = direction.to_vector();

        let new_x: i8 = head_node[0] + offset[0];
        let new_y: i8 = head_node[1] + offset[1];

        if new_x < 0
            || new_y < 0
            || new_x >= BOARD_SQUARES_WIDTH as i8
            || new_y >= BOARD_SQUARES_HEIGHT as i8
        {
            return Err("You left the board!".to_string());
        }

        if self.contains_square(new_x as u8, new_y as u8) {
            return Err("You ate yourself!".to_string());
        }

        self.move_to(new_x, new_y);
        Ok(vec![new_x, new_y])
    }

    pub fn render(&self, ctx: &mut Context) {
        let color: Color = Color::new(0.0, 0.0, 1.0, 1.0);
        for body_piece in self.body_list.iter() {
            let bounds = graphics::Rect::new_i32(
                body_piece[0] as i32 * TILE_WIDTH as i32,
                body_piece[1] as i32 * TILE_WIDTH as i32,
                TILE_WIDTH as i32,
                TILE_WIDTH as i32,
            );
            let drawing =
                graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), bounds, color)
                    .unwrap();
            let _ = graphics::draw(ctx, &drawing, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
        }
    }

    pub fn remove_tail(&mut self) {
        self.body_list.pop_back();
    }
}

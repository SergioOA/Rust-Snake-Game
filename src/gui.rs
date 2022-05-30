use crate::prelude::*;
use ggez::graphics::{self, Text, TextFragment};

pub fn render_gui(ctx: &mut Context, score: u32) {
    let text = Text::new(TextFragment {
        text: format!("Score: {}", score),
        color: Some(Color::new(1.0, 0.0, 0.0, 1.0)),
        ..Default::default()
    });
    let _ = graphics::draw(ctx, &text, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));
}

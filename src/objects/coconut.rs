// Dependencies
use ggez::graphics;
use ggez::Context;

// Constants
const COCONUT_SPEED: f32 = 300.0;

pub struct Coconut {
    coconut_image: graphics::Image,
    coconut_rect: graphics::Rect,
    coconut_pos: glam::Vec2,
}

pub fn new_coconut(ctx: &mut Context) -> Coconut {
    let mut coconut_image = graphics::Image::new(ctx, "/coconut.png").unwrap();
    let coconut_rect = coconut_image.dimensions();
    coconut_image.set_filter(graphics::FilterMode::Nearest);

    Coconut {
        coconut_image: coconut_image,
        coconut_rect: coconut_rect,
        coconut_pos: glam::Vec2::new(0.0, 0.0),
    }
}

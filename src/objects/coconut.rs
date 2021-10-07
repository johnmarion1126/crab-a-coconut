// Dependencies
use ggez::graphics;
use ggez::Context;

// Constants
const COCONUT_SPEED: f32 = 250.0;

pub struct Coconut {
    pub coconut_image: graphics::Image,
    pub coconut_rect: graphics::Rect,
    pub coconut_pos: glam::Vec2,
}

impl Coconut {
    pub fn move_coconut(&mut self, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.coconut_pos.y += 1.0 * COCONUT_SPEED * dt;
    }
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

// Dependencies
use ggez::graphics;
use ggez::Context;
use rand::prelude::*;
use rand::thread_rng;

// Constants
const COCONUT_SPEED: f32 = 250.0;

pub struct Coconut {
    pub coconut_image: graphics::Image,
    pub coconut_rect: graphics::Rect,
    pub coconut_pos: glam::Vec2,
    pub is_destroyed: bool,
}

impl Coconut {
    pub fn move_coconut(&mut self, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.coconut_pos.y += COCONUT_SPEED * dt;
    }
}

pub fn new_coconut(ctx: &mut Context, SCREEN_WIDTH: f32, game_scale: f32) -> Coconut {
    let mut coconut_image = graphics::Image::new(ctx, "/coconut.png").unwrap();
    let coconut_rect = coconut_image.dimensions();
    coconut_image.set_filter(graphics::FilterMode::Nearest);

    let RIGHT_LIMIT = SCREEN_WIDTH - (coconut_rect.w * game_scale);

    Coconut {
        coconut_image,
        coconut_rect,
        coconut_pos: glam::Vec2::new(
            thread_rng().gen_range(0..RIGHT_LIMIT as i32) as f32,
            0.0 - coconut_rect.y,
        ),
        is_destroyed: false,
    }
}

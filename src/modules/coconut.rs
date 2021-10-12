// Dependencies
use crate::object::Object;
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
    pub coconut_speed: f32,
    pub is_destroyed: bool,
}

impl Object for Coconut {
    fn new(ctx: &mut Context, SCREEN_WIDTH: f32, SCALE: f32) -> Coconut {
        let mut coconut_image = graphics::Image::new(ctx, "/coconut.png").unwrap();
        let coconut_rect = coconut_image.dimensions();
        coconut_image.set_filter(graphics::FilterMode::Nearest);

        let RIGHT_LIMIT = SCREEN_WIDTH - (coconut_rect.w * SCALE);

        Coconut {
            coconut_image,
            coconut_rect,
            coconut_pos: glam::Vec2::new(
                thread_rng().gen_range(0..RIGHT_LIMIT as i32) as f32,
                0.0 - coconut_rect.y,
            ),
            coconut_speed: COCONUT_SPEED,
            is_destroyed: false,
        }
    }

    fn get_position_y(&self) -> f32 {
        self.coconut_pos.y
    }

    fn set_position_y(&mut self, pos_y: f32) {
        self.coconut_pos.y = pos_y;
    }

    fn get_speed(&self) -> f32 {
        self.coconut_speed
    }
}

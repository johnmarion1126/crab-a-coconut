// Dependencies
use super::object::Object;
use ggez::graphics;
use ggez::Context;
use rand::prelude::*;
use rand::thread_rng;

// Constants
const ROCK_SPEED: f32 = 350.0;

pub struct Rock {
    pub rock_image: graphics::Image,
    pub rock_rect: graphics::Rect,
    pub rock_pos: glam::Vec2,
    pub rock_speed: f32,
    pub is_destroyed: bool,
}

impl Object for Rock {
    fn new(ctx: &mut Context, SCREEN_WIDTH: f32, SCALE: f32) -> Rock {
        let mut rock_image = graphics::Image::new(ctx, "/rock.png").unwrap();
        let rock_rect = rock_image.dimensions();
        rock_image.set_filter(graphics::FilterMode::Nearest);

        let RIGHT_LIMIT = SCREEN_WIDTH - (rock_rect.w * SCALE);

        Rock {
            rock_image,
            rock_rect,
            rock_pos: glam::Vec2::new(
                thread_rng().gen_range(0..RIGHT_LIMIT as i32) as f32,
                0.0 - rock_rect.y,
            ),
            rock_speed: ROCK_SPEED,
            is_destroyed: false,
        }
    }

    fn get_position_y(&self) -> f32 {
        self.rock_pos.y
    }

    fn set_position_y(&mut self, pos_y: f32) {
        self.rock_pos.y = pos_y;
    }

    fn get_speed(&self) -> f32 {
        self.rock_speed
    }
}
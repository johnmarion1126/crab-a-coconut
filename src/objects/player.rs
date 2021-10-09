// Dependencies
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;

// Constants
const PLAYER_SPEED: f32 = 400.0;

pub struct Player {
    pub player_image: graphics::Image,
    pub player_rect: graphics::Rect,
    pub player_pos: glam::Vec2,
    pub player_score: i32,
}

impl Player {
    pub fn move_player(
        &mut self,
        keycode: KeyCode,
        x_dir: f32,
        ctx: &mut Context,
        PLAYER_WIDTH: f32,
        SCALE: f32,
    ) {
        let MAX_WIDTH = graphics::drawable_size(ctx).0;
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        if keyboard::is_key_pressed(ctx, keycode) {
            self.player_pos.x += x_dir * PLAYER_SPEED * dt;
        }

        limit_boundaries(
            &mut self.player_pos.x,
            0.0,
            MAX_WIDTH - PLAYER_WIDTH * SCALE,
        );
    }
}

fn limit_boundaries(pos: &mut f32, left_boundary: f32, right_boundary: f32) {
    if *pos <= left_boundary {
        *pos = left_boundary;
    } else if *pos >= right_boundary {
        *pos = right_boundary;
    }
}

pub fn new_player(
    ctx: &mut Context,
    SCREEN_HEIGHT: f32,
    SCREEN_WIDTH_HALF: f32,
    BOTTOM_PADDING: f32,
    SCALE: f32,
) -> Player {
    let mut player_image = graphics::Image::new(ctx, "/crab.png").unwrap();
    let player_rect = player_image.dimensions();
    player_image.set_filter(graphics::FilterMode::Nearest);
    let player_pos = glam::Vec2::new(
        SCREEN_WIDTH_HALF - (player_rect.w * SCALE) / 2.0,
        SCREEN_HEIGHT - BOTTOM_PADDING,
    );

    Player {
        player_image: player_image,
        player_rect: player_rect,
        player_pos: player_pos,
        player_score: 0,
    }
}

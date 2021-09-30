// Dependencies
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;

// Constants
const PLAYER_SPEED: f32 = 300.0;

pub fn move_player(pos: &mut glam::Vec2, keycode: KeyCode, x_dir: f32, ctx: &mut Context) {
    let MAX_WIDTH = graphics::drawable_size(ctx).0;
    let dt = ggez::timer::delta(ctx).as_secs_f32();

    if keyboard::is_key_pressed(ctx, keycode) {
        pos.x += x_dir * PLAYER_SPEED * dt;
    }

}
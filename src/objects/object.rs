// Dependencies
use ggez::graphics;
use ggez::Context;

pub trait Object {
    fn get_image(&self) -> graphics::Image;

    fn get_rect(&self) -> graphics::Rect;

    fn get_speed(&self) -> f32;

    fn get_status(&self) -> bool;

    fn destroy_object(&mut self);

    fn get_position(&self) -> glam::Vec2;

    fn set_position_y(&mut self, pos_y: f32);

    fn get_damage(&self) -> i32;

    fn get_points(&self) -> i32;

    fn move_object(&mut self, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let mut pos_y = self.get_position().y;
        pos_y += self.get_speed() * dt;
        self.set_position_y(pos_y);
    }
}

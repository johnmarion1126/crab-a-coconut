// Dependencies
use ggez::Context;

pub trait Object {
    fn new(ctx: &mut Context, SCREEN_WIDTH: f32, SCALE: f32) -> Self;

    fn get_position_y(&self) -> f32;

    fn set_position_y(&mut self, pos_y: f32);

    fn get_speed(&self) -> f32;

    fn move_object(&mut self, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let mut pos_y = self.get_position_y();
        pos_y += self.get_speed() * dt;
        self.set_position_y(pos_y);
    }
}

// Dependencies
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

struct MainState {}

impl MainState {
    pub fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let player_rect = graphics::Rect::new(0.0, 0.0, 100.0, 100.0);
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            graphics::Color::WHITE,
        )?;
        let draw_param = graphics::DrawParam::new();
        let player_dist = glam::Vec2::new(0.0, 0.0);
        graphics::draw(ctx, &player_mesh, draw_param.dest(player_dist))?;
        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Crab-A-Coconut", "John");
    let (ctx, event_loop) = context_builder.build()?;
    graphics::set_window_title(&ctx, "Crab-A-Coconut");

    let state = MainState::new();
    event::run(ctx, event_loop, state);
    Ok(())
}

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

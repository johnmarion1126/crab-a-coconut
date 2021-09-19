// Dependencies
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Crab-A-Coconut", "John");
    let (ctx, event_loop) = context_builder.build()?;
    graphics::set_window_title(&ctx, "Crab-A-Coconut");

    Ok(())
}

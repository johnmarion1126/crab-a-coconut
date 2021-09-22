// Dependencies
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

struct MainState {
    SCREEN_HEIGHT: f32,
    SCREEN_WIDTH: f32,
    SCREEN_HEIGHT_HALF: f32,
    SCREEN_WIDTH_HALF: f32,
}

impl MainState {
    pub fn new(ctx: &Context) -> Self {
        let (SCREEN_WIDTH, SCREEN_HEIGHT) = graphics::drawable_size(ctx);
        let (SCREEN_WIDTH_HALF, SCREEN_HEIGHT_HALF) = (SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5);

        MainState {
            SCREEN_HEIGHT: SCREEN_HEIGHT,
            SCREEN_WIDTH: SCREEN_WIDTH,
            SCREEN_HEIGHT_HALF: SCREEN_HEIGHT_HALF,
            SCREEN_WIDTH_HALF: SCREEN_WIDTH_HALF,
        }
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
        let player_dist =
            glam::Vec2::new(self.SCREEN_WIDTH_HALF - 50.0, self.SCREEN_HEIGHT - 100.0);
        graphics::draw(ctx, &player_mesh, draw_param.dest(player_dist))?;
        graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("Crab-A-Coconut", "John");
    let (ctx, event_loop) = context_builder.build()?;
    graphics::set_window_title(&ctx, "Crab-A-Coconut");

    let state = MainState::new(&ctx);
    event::run(ctx, event_loop, state);
    Ok(())
}

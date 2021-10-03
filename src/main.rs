// Dependencies
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::env;
use std::path;

// Objects
#[path = "./objects/player.rs"]
mod player;

// Constants
const BOTTOM_PADDING: f32 = 100.0;

struct MainState {
    player_image: graphics::Image,
    player_rect: graphics::Rect,
    coconut_image: graphics::Image,
    coconut_rect: graphics::Rect,
    player_pos: glam::Vec2,
    SCREEN_HEIGHT: f32,
    SCREEN_WIDTH: f32,
    SCREEN_HEIGHT_HALF: f32,
    SCREEN_WIDTH_HALF: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let mut player_image = graphics::Image::new(ctx, "/crab.png").unwrap();
        let player_rect = player_image.dimensions();
        player_image.set_filter(graphics::FilterMode::Nearest);

        let mut coconut_image = graphics::Image::new(ctx, "/coconut.png").unwrap();
        let coconut_rect = coconut_image.dimensions();
        coconut_image.set_filter(graphics::FilterMode::Nearest);

        let (SCREEN_WIDTH, SCREEN_HEIGHT) = graphics::drawable_size(ctx);
        let (SCREEN_WIDTH_HALF, SCREEN_HEIGHT_HALF) = (SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5);

        let player_pos = glam::Vec2::new(SCREEN_WIDTH_HALF - player_rect.w*2.0, SCREEN_HEIGHT - BOTTOM_PADDING);

        MainState {
            player_image: player_image,
            player_rect: player_rect,
            coconut_image: coconut_image,
            coconut_rect: coconut_rect,
            player_pos: player_pos,
            SCREEN_HEIGHT: SCREEN_HEIGHT,
            SCREEN_WIDTH: SCREEN_WIDTH,
            SCREEN_HEIGHT_HALF: SCREEN_HEIGHT_HALF,
            SCREEN_WIDTH_HALF: SCREEN_WIDTH_HALF,
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        player::move_player(&mut self.player_pos, KeyCode::A, -1.0, ctx, self.player_rect.w);
        player::move_player(&mut self.player_pos, KeyCode::D, 1.0, ctx, self.player_rect.w);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        let draw_param = graphics::DrawParam::new();
        let game_scale = glam::Vec2::new(5.0, 5.0);

        graphics::draw(
            ctx,
            &self.player_image,
            draw_param.dest(self.player_pos).scale(game_scale),
        )?;

        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(
                self.player_rect.x,
                self.player_rect.y + 4.0,
                self.player_rect.w,
                self.player_rect.h - 5.0,
            ),
            graphics::Color::WHITE,
        )?;

        graphics::draw(
            ctx,
            &player_mesh,
            draw_param.dest(self.player_pos).scale(game_scale),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    let context_builder =
        ggez::ContextBuilder::new("Crab-A-Coconut", "John").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = context_builder.build()?;
    graphics::set_window_title(&ctx, "Crab-A-Coconut");
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);
    Ok(())
}

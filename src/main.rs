// Dependencies
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::env;
use std::path;

// Objects
#[path = "./objects/coconut.rs"]
mod coconut;
#[path = "./objects/player.rs"]
mod player;

// Constants
const BOTTOM_PADDING: f32 = 100.0;
const SCALE: f32 = 5.0;

struct MainState {
    spawn_time: f32,
    coconuts: Vec<coconut::Coconut>,
    player: player::Player,
    SCREEN_HEIGHT: f32,
    SCREEN_WIDTH: f32,
    SCREEN_HEIGHT_HALF: f32,
    SCREEN_WIDTH_HALF: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (SCREEN_WIDTH, SCREEN_HEIGHT) = graphics::drawable_size(ctx);
        let (SCREEN_WIDTH_HALF, SCREEN_HEIGHT_HALF) = (SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5);

        let player =
            player::new_player(ctx, SCREEN_HEIGHT, SCREEN_WIDTH_HALF, BOTTOM_PADDING, SCALE);

        MainState {
            spawn_time: 0.0,
            coconuts: vec![],
            player: player,
            SCREEN_HEIGHT: SCREEN_HEIGHT,
            SCREEN_WIDTH: SCREEN_WIDTH,
            SCREEN_HEIGHT_HALF: SCREEN_HEIGHT_HALF,
            SCREEN_WIDTH_HALF: SCREEN_WIDTH_HALF,
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.spawn_time += 1.0;
        if self.spawn_time == 200.0 {
            self.spawn_time = 0.0;
            self.coconuts
                .push(coconut::new_coconut(ctx, self.SCREEN_WIDTH));
        }
        &self
            .player
            .move_player(KeyCode::A, -1.0, ctx, self.player.player_rect.w, SCALE);
        &self
            .player
            .move_player(KeyCode::D, 1.0, ctx, self.player.player_rect.w, SCALE);

        for coconut in &mut self.coconuts {
            coconut.move_coconut(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        let draw_param = graphics::DrawParam::new();
        let game_scale = glam::Vec2::new(SCALE, SCALE);

        graphics::draw(
            ctx,
            &self.player.player_image,
            draw_param.dest(self.player.player_pos).scale(game_scale),
        )?;

        for coconut in &self.coconuts {
            graphics::draw(
                ctx,
                &coconut.coconut_image,
                draw_param.dest(coconut.coconut_pos).scale(game_scale),
            )?;
        }
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(
                self.player.player_rect.x,
                self.player.player_rect.y + 4.0,
                self.player.player_rect.w,
                self.player.player_rect.h - 5.0,
            ),
            graphics::Color::WHITE,
        )?;

        graphics::draw(
            ctx,
            &player_mesh,
            draw_param.dest(self.player.player_pos).scale(game_scale),
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

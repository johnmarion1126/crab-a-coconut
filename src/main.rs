// Dependencies
use ggez::event;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context, GameResult};
use std::env;
use std::path;

// Modules
mod objects;
use objects::coconut;
use objects::object::Object;
use objects::player;
use objects::rock;

// Types
type ObjectT = Box<dyn Object>;

// Constants
const BOTTOM_PADDING: f32 = 100.0;
const SCALE: f32 = 5.0;

enum GameState {
    InGame,
    GameOver,
    StartScreen,
}

struct MainState {
    spawn_time: f32,
    objects: Vec<ObjectT>,
    player: player::Player,
    SCREEN_HEIGHT: f32,
    SCREEN_WIDTH: f32,
    SCREEN_HEIGHT_HALF: f32,
    SCREEN_WIDTH_HALF: f32,
    game_state: GameState,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (SCREEN_WIDTH, SCREEN_HEIGHT) = graphics::drawable_size(ctx);
        let (SCREEN_WIDTH_HALF, SCREEN_HEIGHT_HALF) = (SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5);

        let player =
            player::new_player(ctx, SCREEN_HEIGHT, SCREEN_WIDTH_HALF, BOTTOM_PADDING, SCALE);

        MainState {
            spawn_time: 0.0,
            objects: vec![],
            player: player,
            SCREEN_HEIGHT: SCREEN_HEIGHT,
            SCREEN_WIDTH: SCREEN_WIDTH,
            SCREEN_HEIGHT_HALF: SCREEN_HEIGHT_HALF,
            SCREEN_WIDTH_HALF: SCREEN_WIDTH_HALF,
            game_state: GameState::StartScreen,
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.game_state {
            GameState::GameOver => {
                if keyboard::is_key_pressed(ctx, KeyCode::R) {
                    self.game_state = GameState::InGame;
                    self.player = player::new_player(
                        ctx,
                        self.SCREEN_HEIGHT,
                        self.SCREEN_WIDTH_HALF,
                        BOTTOM_PADDING,
                        SCALE,
                    );
                    self.objects = vec![];
                }
            }
            GameState::InGame => {
                &self
                    .player
                    .move_player(KeyCode::A, -1.0, ctx, self.player.player_rect.w, SCALE);
                &self
                    .player
                    .move_player(KeyCode::D, 1.0, ctx, self.player.player_rect.w, SCALE);

                self.spawn_time += 1.0;
                if self.spawn_time % 100.0 == 0.0 {
                    self.objects
                        .push(Box::new(rock::new(ctx, self.SCREEN_WIDTH, SCALE)));
                }
                if self.spawn_time == 200.0 {
                    self.spawn_time = 0.0;
                    self.objects
                        .push(Box::new(coconut::new(ctx, self.SCREEN_WIDTH, SCALE)));
                }

                remove_objects(&mut self.objects);

                for object in &mut self.objects {
                    object.move_object(ctx);
                }
            }
            GameState::StartScreen => {
                if keyboard::is_key_pressed(ctx, KeyCode::Z) {
                    self.game_state = GameState::InGame;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        let draw_param = graphics::DrawParam::new();
        let game_scale = glam::Vec2::new(SCALE, SCALE);

        let mut background_image = graphics::Image::new(ctx, "/beach.png").unwrap();
        let background_rect = background_image.dimensions();
        background_image.set_filter(graphics::FilterMode::Nearest);
        graphics::draw(
            ctx,
            &background_image,
            draw_param
                .dest(glam::Vec2::new(
                    0.0, // self.SCREEN_WIDTH_HALF - background_rect.w,
                    0.0, //self.SCREEN_HEIGHT_HALF - background_rect.h,
                ))
                .scale(game_scale),
        )?;

        match self.game_state {
            GameState::GameOver => {
                let game_over_text = graphics::Text::new("     Game over\nPress r to try again");
                let game_over_rect = game_over_text.dimensions(ctx);
                graphics::draw(
                    ctx,
                    &game_over_text,
                    draw_param.dest(glam::Vec2::new(
                        self.SCREEN_WIDTH_HALF - game_over_rect.w / 2.0,
                        self.SCREEN_HEIGHT_HALF,
                    )),
                )?;
            }
            GameState::InGame => {
                graphics::draw(
                    ctx,
                    &self.player.player_image,
                    draw_param.dest(self.player.player_pos).scale(game_scale),
                )?;

                for object in &mut self.objects {
                    if object.get_position().y >= self.player.player_pos.y
                        && object.get_position().y
                            < self.player.player_pos.y + (self.player.player_rect.h * SCALE)
                        && object.get_position().x
                            < self.player.player_pos.x + (self.player.player_rect.w * SCALE)
                        && object.get_position().x + (object.get_rect().w * SCALE)
                            > self.player.player_pos.x
                    {
                        object.destroy_object();
                        self.player.player_score += object.get_points();
                        if self.player.player_hp != 0 {
                            self.player.player_hp -= object.get_damage();
                        }
                        if self.player.player_hp == 0 {
                            self.game_state = GameState::GameOver;
                        }
                    }

                    if object.get_position().y >= self.SCREEN_HEIGHT {
                        object.destroy_object()
                    }
                    graphics::draw(
                        ctx,
                        &object.get_image(),
                        draw_param.dest(object.get_position()).scale(game_scale),
                    )?;
                }

                let score_text =
                    graphics::Text::new(format!("Score: {}", self.player.player_score));
                graphics::draw(ctx, &score_text, draw_param.dest(glam::Vec2::new(0.0, 0.0)))?;
                let hp_text = graphics::Text::new(format!("HP: {}", self.player.player_hp));
                let hp_rect = hp_text.dimensions(ctx);
                graphics::draw(
                    ctx,
                    &hp_text,
                    draw_param.dest(glam::Vec2::new(self.SCREEN_WIDTH - hp_rect.w, 0.0)),
                )?;
            }
            GameState::StartScreen => {
                let start_text = graphics::Text::new(" Crab-A-Coconut\nPress z to start");
                let start_rect = start_text.dimensions(ctx);
                graphics::draw(
                    ctx,
                    &start_text,
                    draw_param.dest(glam::Vec2::new(
                        self.SCREEN_WIDTH_HALF - start_rect.w / 2.0,
                        self.SCREEN_HEIGHT_HALF,
                    )),
                )?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn remove_objects(objects: &mut Vec<ObjectT>) {
    let mut index = 0;
    while index < objects.len() {
        if objects[index].get_status() {
            objects.remove(index);
        }
        index += 1;
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

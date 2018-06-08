use std::time::{Instant, Duration};
use ggez::*;
use ggez::event::{Keycode, Mod};
use nalgebra::{Point2};
use super::entity_manager::*;
use super::asset_manager::*;
use super::camera::*;

const MAX_UPDATES_PER_SECOND: u32 = 60;
const MS_PER_UPDATE: u64 = ((1.0/MAX_UPDATES_PER_SECOND as f64)*1000.0) as u64;
const MAX_FRAMES_PER_SECOND: u32 = 144;
const MS_PER_FRAME: u64 = ((1.0/MAX_FRAMES_PER_SECOND as f64)*1000.0) as u64;

pub struct GameState {
    window_w: u32,
    window_h: u32,
    last_update: Instant,
    last_draw: Instant,
    camera: Camera,
    entity_manager: EntityManager,
    asset_manager: AssetManager
}

impl GameState {
    pub fn new(ctx: &mut Context, window_w: u32, window_h: u32) -> GameResult<GameState> {
        if let Ok(asset_manager) = AssetManager::new(ctx) {
            return Ok(GameState {
                window_w,
                window_h,
                last_update: Instant::now(),
                last_draw: Instant::now(),
                camera: Camera::new(window_w, window_h),
                entity_manager: EntityManager::new(),
                asset_manager
            });
        }
        Err(GameError::UnknownError("Failed to inialize game state! Game exiting..".to_string()))
    }

    fn draw_game_over_text(&mut self, ctx: &mut Context) {
        let game_over_text = graphics::Text::new(
            ctx, "Game Over", &self.asset_manager.game_over_font).unwrap();
        graphics::draw(
            ctx,
            &game_over_text,
            Point2::new(
                (self.window_w / 2) as f32 - (game_over_text.width() / 2) as f32,
                (self.window_h / 2) as f32 - (game_over_text.height() / 2) as f32
            ),
            0.0
        ).unwrap();
    }
}

fn get_interpolation_value(game_state: &GameState) -> f32 {
    (game_state.last_update.elapsed().subsec_nanos() as f32 / 1_000_000.0 ) / (MS_PER_UPDATE as f32)
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.last_update.elapsed() > Duration::from_millis(MS_PER_UPDATE) {
            self.entity_manager.update();
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let last_draw_elapsed = self.last_draw.elapsed();
        if last_draw_elapsed > Duration::from_millis(MS_PER_FRAME) {
            graphics::clear(ctx);
            graphics::set_background_color(ctx, graphics::BLACK);

            let interpolation_value = get_interpolation_value(self);
            self.entity_manager.draw(&self.asset_manager, ctx, interpolation_value, &self.camera);

            if !self.entity_manager.is_player_alive() {
                self.draw_game_over_text(ctx);
            }
           
            graphics::present(ctx);
            self.last_draw = Instant::now();
        } else {
            timer::sleep((Duration::from_millis(MS_PER_FRAME) - last_draw_elapsed) / 3);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool
    ) {
        match keycode {
            Keycode::W => self.entity_manager.player_move(0),
            Keycode::S => self.entity_manager.player_move(1),
            Keycode::D => self.entity_manager.player_move(2),
            Keycode::A => self.entity_manager.player_move(3),
            Keycode::Space => if !repeat {
                self.entity_manager.player_fire()
            },
            _ => {}
        }
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: Keycode,
        keymod: Mod,
        repeat: bool
    ) {
        match keycode {
            Keycode::W => self.entity_manager.player_move_cancel(0),
            Keycode::S => self.entity_manager.player_move_cancel(1),
            Keycode::D => self.entity_manager.player_move_cancel(2),
            Keycode::A => self.entity_manager.player_move_cancel(3),
            _ => {}
        }
    }
}
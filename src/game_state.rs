use std::time::{Instant, Duration};
use ggez::*;
use ggez::event::{Keycode, Mod};
use super::entity_manager::*;
use super::asset_manager::*;
use super::camera::*;
use super::wave_manager::*;

const MAX_UPDATES_PER_SECOND: u32 = 60;
const MS_PER_UPDATE: u64 = ((1.0/MAX_UPDATES_PER_SECOND as f64)*1000.0) as u64;
const MAX_FRAMES_PER_SECOND: u32 = 144;
const MS_PER_FRAME: u64 = ((1.0/MAX_FRAMES_PER_SECOND as f64)*1000.0) as u64;

pub struct GameState {
    last_update: Instant,
    last_draw: Instant,
    camera: Camera,
    player_paused: bool,
    game_started: bool,
    entity_manager: EntityManager,
    asset_manager: AssetManager,
    wave_manager: WaveManager
}

impl GameState {
    pub fn new(ctx: &mut Context, window_w: u32, window_h: u32) -> GameResult<GameState> {
        let play_area = graphics::Rect::new(0.0, window_h as f32, window_w as f32, window_h as f32);
        if let Ok(asset_manager) = AssetManager::new(ctx, window_w, window_h) {
            return Ok(GameState {
                last_update: Instant::now(),
                last_draw: Instant::now(),
                camera: Camera::new(window_w, window_h),
                player_paused: false,
                game_started: false,
                entity_manager: EntityManager::new(play_area.clone()),
                asset_manager,
                wave_manager: WaveManager::new(play_area.clone())
            });
        }
        Err(GameError::UnknownError("Failed to inialize game state! Game exiting..".to_string()))
    }

    fn update_game(&mut self) {
        if !self.get_game_paused() {
            self.entity_manager.update();
            self.wave_manager.update(&mut self.entity_manager);
        }
    }

    fn draw_game(&self, ctx: &mut Context) {
        self.entity_manager.draw(&self.asset_manager, ctx, self.get_interpolation_value(), &self.camera);
        self.draw_overlay(ctx);
    }

    fn draw_overlay(&self, ctx: &mut Context) {
        if !self.game_started {
            self.draw_game_start_text(ctx);
        } else {
            if !self.entity_manager.is_player_alive() {
                self.draw_game_over_text(ctx);
            }
        }
    }

    fn draw_game_start_text(&self, ctx: &mut Context) {
        let start_text = graphics::Text::new(ctx, "Press a key to Start!", &self.asset_manager.large_splash_font).unwrap();
        self.asset_manager.draw_centered_text(
            ctx, start_text
        );
    }

    fn draw_game_over_text(&self, ctx: &mut Context) {
        let game_over_text = graphics::Text::new(ctx, "Game Over", &self.asset_manager.large_splash_font).unwrap();
        self.asset_manager.draw_centered_text(
            ctx, game_over_text
        );
    }

    fn get_game_paused(&self) -> bool { !self.game_started || self.player_paused }

    fn get_interpolation_value(&self) -> f32 {
        if self.get_game_paused() {
            0.0
        } else {
            (self.last_update.elapsed().subsec_nanos() as f32 / 1_000_000.0 ) / (MS_PER_UPDATE as f32)
        }
    }

    fn is_wave_complete(&self) -> bool {
        self.wave_manager.wave_spawn_complete() && self.entity_manager.get_enemy_count() == 0
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.last_update.elapsed() > Duration::from_millis(MS_PER_UPDATE) {
            self.update_game();
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let last_draw_elapsed = self.last_draw.elapsed();
        if last_draw_elapsed > Duration::from_millis(MS_PER_FRAME) {
            graphics::clear(ctx);
            graphics::set_background_color(ctx, graphics::BLACK);

            self.draw_game(ctx);
           
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
        self.game_started = true;
        match keycode {
            Keycode::W => self.entity_manager.player_move_cancel(0),
            Keycode::S => self.entity_manager.player_move_cancel(1),
            Keycode::D => self.entity_manager.player_move_cancel(2),
            Keycode::A => self.entity_manager.player_move_cancel(3),
            Keycode::Escape => self.player_paused = !self.player_paused,
            _ => {}
        }
    }
}
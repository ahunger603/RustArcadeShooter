use std::time::{Instant, Duration};
use ggez::*;
use ggez::event::{Keycode, Mod};
use super::entity_manager::*;
use super::asset_manager::*;
use super::camera::*;
use super::wave_manager::*;
use super::play_space::*;

const MAX_UPDATES_PER_SECOND: u32 = 60;
const MS_PER_UPDATE: u64 = ((1.0/MAX_UPDATES_PER_SECOND as f64)*1000.0) as u64;
const MAX_FRAMES_PER_SECOND: u32 = 144;
const MS_PER_FRAME: u64 = ((1.0/MAX_FRAMES_PER_SECOND as f64)*1000.0) as u64;

const STARTING_LIVES: i32 = 10;

pub struct GameState {
    last_update: Instant,
    last_draw: Instant,
    camera: Camera,
    player_paused: bool,
    game_started: bool,
    lives: i32,
    score: u32,
    entity_manager: EntityManager,
    asset_manager: AssetManager,
    wave_manager: WaveManager
}

impl GameState {
    pub fn new(ctx: &mut Context, window_w: u32, window_h: u32) -> GameResult<GameState> {
        let play_space = PlaySpace::new(window_w as f32, window_h as f32);
        if let Ok(asset_manager) = AssetManager::new(ctx, window_w, window_h) {
            return Ok(GameState {
                last_update: Instant::now(),
                last_draw: Instant::now(),
                camera: Camera::new(window_w, window_h),
                player_paused: false,
                game_started: false,
                lives: STARTING_LIVES,
                score: 0,
                entity_manager: EntityManager::new(play_space.clone()),
                asset_manager,
                wave_manager: WaveManager::new(play_space.clone())
            });
        }
        Err(GameError::UnknownError("Failed to inialize game state! Game exiting..".to_string()))
    }

    fn update_game(&mut self) {
        if !self.is_game_paused() && !self.is_game_over() {
            self.entity_manager.update();
            self.wave_manager.update(&mut self.entity_manager);
            
            self.lives -= self.entity_manager.update_life_lost() as i32;
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
            if self.is_game_over() {
                self.draw_game_over_text(ctx);
            } else {
                if self.is_wave_complete() {
                    self.draw_next_level_text(ctx);
                }
            }
            self.draw_lives(ctx);
            self.draw_level(ctx);
            self.draw_score(ctx);
        }
    }

    fn draw_lives(&self, ctx: &mut Context) {
        let next_level_text = graphics::Text::new(ctx, 
            format!("Lives: {}", self.lives).as_str(),
            &self.asset_manager.med_splash_font
        ).unwrap();
        self.asset_manager.draw_top_left_text(
            ctx, next_level_text
        );
    }

    fn draw_level(&self, ctx: &mut Context) {
        let next_level_text = graphics::Text::new(ctx, 
            format!("Level: {}", self.wave_manager.get_wave_level()).as_str(),
            &self.asset_manager.med_splash_font
        ).unwrap();
        self.asset_manager.draw_top_centered_text(
            ctx, next_level_text
        );
    }

    fn draw_score(&self, ctx: &mut Context) {
        let mut score_string = format!("{}", self.score);
        while score_string.len() < 8 {
            score_string.insert(0, '0');
        }
        let next_level_text = graphics::Text::new(ctx, 
            format!("Score: {}", score_string).as_str(),
            &self.asset_manager.med_splash_font
        ).unwrap();
        self.asset_manager.draw_top_right_text(
            ctx, next_level_text
        );
    }

    fn draw_next_level_text(&self, ctx: &mut Context) {
        let next_level_text = graphics::Text::new(ctx, 
            format!("Press SPACE to start level {}!",
            self.wave_manager.get_wave_level() + 1).as_str(),
            &self.asset_manager.med_splash_font
        ).unwrap();
        self.asset_manager.draw_bottom_centered_text(
            ctx, next_level_text
        );
    }

    fn draw_game_start_text(&self, ctx: &mut Context) {
        let title_text = graphics::Text::new(ctx, "Arcade Shooter", &self.asset_manager.large_splash_font).unwrap();
        let start_text = graphics::Text::new(ctx, "Press SPACE to start!", &self.asset_manager.med_splash_font).unwrap();
        self.asset_manager.draw_centered_text(
            ctx, title_text
        );
        self.asset_manager.draw_bottom_centered_text(
            ctx, start_text
        );
    }

    fn draw_game_over_text(&self, ctx: &mut Context) {
        let game_over_text = graphics::Text::new(ctx, "Game Over", &self.asset_manager.large_splash_font).unwrap();
        self.asset_manager.draw_centered_text(
            ctx, game_over_text
        );
    }

    fn is_game_over(&self) -> bool { self.lives <= 0 }

    fn is_game_paused(&self) -> bool { !self.game_started || self.player_paused }

    fn get_interpolation_value(&self) -> f32 {
        if self.is_game_paused() {
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
        match keycode {
            Keycode::W => self.entity_manager.player_move_cancel(0),
            Keycode::S => self.entity_manager.player_move_cancel(1),
            Keycode::D => self.entity_manager.player_move_cancel(2),
            Keycode::A => self.entity_manager.player_move_cancel(3),
            Keycode::Escape => self.player_paused = !self.player_paused,
            Keycode::Space => {
                self.game_started = true;
                if self.is_wave_complete() {
                    self.wave_manager.set_to_progress_level();
                }
            },
            _ => {}
        }
    }
}
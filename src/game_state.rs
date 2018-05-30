use std::time::{Instant, Duration};
use ggez::*;
use ggez::event::{Keycode, Mod};
use super::entity_manager::*;
use super::asset_manager::*;

const MAX_UPDATES_PER_SECOND: u32 = 20;
const MS_PER_UPDATE: u64 = ((1.0/MAX_UPDATES_PER_SECOND as f64)*1000.0) as u64;

pub struct GameState {
    last_update: Instant,
    entity_manager: EntityManager,
    asset_manager: AssetManager
}

impl GameState {
    pub fn new(mut ctx: &mut Context) -> GameResult<GameState> {
        if let Ok(asset_manager) = AssetManager::new(ctx) {
            return Ok(GameState {
                last_update: Instant::now(),
                entity_manager: EntityManager::new(),
                asset_manager
            });
        }
        Err(GameError::UnknownError("Failed to inialize game state! Game exiting..".to_string()))
    }
}

fn get_interpolation_value(game_state: &GameState) -> f32 {
    (game_state.last_update.elapsed().subsec_nanos() / 1_000_000) as f32 / (MS_PER_UPDATE as f32)
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
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);
        let interpolation_value = get_interpolation_value(self);
        self.entity_manager.draw(ctx, interpolation_value);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        
    }
}
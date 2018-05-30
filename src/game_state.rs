use std::time::{Instant, Duration};
use ggez::*;
use ggez::event::{Keycode, Mod};
use super::entity_manager::*;

const MAX_UPDATES_PER_SECOND: u32 = 20;
const MS_PER_UPDATE: u64 = ((1.0/MAX_UPDATES_PER_SECOND as f64)*1000.0) as u64;

pub struct GameState {
    last_update: Instant,
    entity_manager: EntityManager
}

impl GameState {
    pub fn new(mut _ctx: &mut Context) -> GameResult<GameState> {
        Ok(GameState {
            last_update: Instant::now(),
            entity_manager: EntityManager::new()
        })
    }
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
        self.entity_manager.draw(ctx);
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
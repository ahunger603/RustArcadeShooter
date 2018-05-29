use ggez::*;
use ggez::event::{Keycode, Mod};
use super::entity_manager::*;

pub struct GameState {
    entity_manager: EntityManager
}

impl GameState {
    pub fn new(mut _ctx: &mut Context) -> GameResult<GameState> {
        Ok(GameState {
            entity_manager: EntityManager::new()
        })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);
        
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
use ggez::*;
use super::asset_manager::*;
use super::entity::*;
use super::player::*;

pub struct EntityManager {
    player: Player,
    enemies: Vec<Box<Entity>>
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            player: Player::new(),
            enemies: vec![]
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32) {
        self.player.draw(asset_manager, ctx, interpolation_value);
    }
}
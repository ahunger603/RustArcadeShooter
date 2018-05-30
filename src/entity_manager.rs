use ggez::*;
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

    pub fn draw(&self, ctx: &mut Context, interpolation_value: f32) {
        self.player.draw(ctx, interpolation_value);
    }
}
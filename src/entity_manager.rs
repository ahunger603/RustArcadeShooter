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

    fn update(&mut self) {
        
    }

    fn draw(&self, ctx: &mut Context) {

    }
}
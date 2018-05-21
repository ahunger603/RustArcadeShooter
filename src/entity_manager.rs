use ggez::*;
use super::player::*;


pub struct EntityManager {
    player: Player
}

impl EntityManager {
    fn update(&mut self) -> () {
        self.player.update();
    }

    fn draw(&self, ctx: &mut Context) -> () {

    }
}
use ggez::*;
use super::entity::*;
use super::body::*;


pub struct Player {
    body: Body
}

impl Entity for Player {
    fn update(&mut self) -> () {

    }

    fn draw(&self, ctx: &mut Context) -> () {

    }

    fn get_body(&self) -> Option<Body> {
        return Some(self.body.clone());
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

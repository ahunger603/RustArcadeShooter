use ggez::*;
use super::entity::*;
use super::body::*;


pub struct Player {
    body: Body
}

impl Player {
    pub fn new() -> Player {
        Player {
            body: Body::new(0.0, 0.0, 10.0, 10.0, true)
        }
    }
}

impl Entity for Player {
    fn update(&mut self) {

    }

    fn draw(&self, ctx: &mut Context) {
        
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

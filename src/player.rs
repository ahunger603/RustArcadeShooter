use ggez::*;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;

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

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32) {
        let body = &self.body;
        graphics::draw(ctx, &asset_manager.player, Point2::new(body.pos.x, body.pos.y), 0.0);
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

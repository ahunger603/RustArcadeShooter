use std::f32;
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
            body: Body::new(300.0, 300.0, 10.0, 10.0, f32::consts::PI/2.0, true)
        }
    }
}

impl Entity for Player {
    fn update(&mut self) {
        
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32) {
        let image_dim = asset_manager.player.get_dimensions();
        let body = &self.body;
        graphics::draw_ex(
            ctx,
            &asset_manager.player,
            graphics::DrawParam {
                dest: Point2::new(body.pos.x, body.pos.y),
                rotation: body.rotation,
                offset: Point2::new(0.5, 0.5),
                .. Default::default()
            }
        ).unwrap();
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

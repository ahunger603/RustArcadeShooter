use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;

const ENEMY_DIRECTION: f32 = f32::consts::PI;
const MOVE_SPEED_NORMAL: f32 = 5.0;

pub struct Enemy {
    is_dead: bool,
    asset_key: String,
    body: Body
}

impl Enemy {
    fn new(x: f32, y: f32, asset_key: String, move_speed: f32) -> Enemy {
        let mut body = Body::new(x, y, 132.0, 128.0, 0.5, 0.5, (f32::consts::PI*3.0)/2.0, true);
        body.velocity = Vector2::new(move_speed, ENEMY_DIRECTION);
        Enemy {
            is_dead: false,
            asset_key,
            body
        }
    }

    pub fn new_drone(x:f32, y: f32) -> Enemy {
        Enemy::new(x, y, "drone1".to_string(), MOVE_SPEED_NORMAL)
    }
}

impl Entity for Enemy {
    fn update(&mut self) {
        self.body.update_pos();
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        asset_manager.draw_asset(self.asset_key.clone(), ctx, self.body.get_default_draw_param(interpolation_value, camera));
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }

    fn is_dead(&self) -> bool {
        self.is_dead
    }

    fn set_dead(&mut self) {
        self.is_dead = true;
    }
}

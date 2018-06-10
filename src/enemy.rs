use std::f32;
use ggez::*;
use nalgebra::Vector2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;
use super::unit::*;

const ENEMY_DIRECTION: f32 = f32::consts::PI;
const MOVE_SPEED_NORMAL: f32 = 5.0;

pub enum EnemyType {
    NormalDrone
}

pub struct Enemy {
    unit: Unit
}

impl Enemy {
    fn new(x: f32, y: f32, asset_key: String, move_speed: f32) -> Enemy {
        let mut body = Body::new(x, y, 132.0, 128.0, 0.5, 0.5, (f32::consts::PI*3.0)/2.0, true);
        body.velocity = Vector2::new(move_speed, ENEMY_DIRECTION);
        Enemy {
            unit: Unit::new(body, asset_key, 1, 1, true)
        }
    }

    pub fn new_drone(x:f32, y: f32) -> Enemy {
        Enemy::new(x, y, "drone1".to_string(), MOVE_SPEED_NORMAL)
    }

    pub fn create_enemy_by_key(enemy_type: EnemyType, x:f32, y: f32) -> Option<Enemy> {
        match enemy_type {
            EnemyType::NormalDrone => Some(Enemy::new_drone(x, y)),
            _ => None
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self) {
        self.unit.update();
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        self.unit.draw(asset_manager, ctx, interpolation_value, camera);
    }

    fn get_body(&self) -> Option<Body> {
        self.unit.get_body()
    }

    fn set_body(&mut self, body: Body) {
        self.unit.set_body(body)
    }

    fn is_dead(&self) -> bool {
        self.unit.is_dead
    }

    fn set_dead(&mut self) {
        self.unit.set_dead()
    }
}

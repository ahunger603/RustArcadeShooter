use std::f32;
use ggez::*;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;
use super::unit::*;

pub struct Partical {
    unit: Unit
}

impl Partical {
    fn new(x: f32, y: f32, scale_x: f32, scale_y: f32, rotation: f32, asset_key: String, sheet_w: u32, sheet_h: u32) -> Partical {
        Partical {
            unit: Unit::new(
                Body::new(x, y, 0.0, 0.0, scale_x, scale_y, rotation, false),
                asset_key, sheet_w, sheet_h, false
            )
        }
    }

    pub fn new_drone_death(x:f32, y: f32) -> Partical {
        Partical::new(x, y, 1.5, 1.5, f32::consts::PI/2.0, "explosion1".to_string(), 8, 8)
    }
}

impl Entity for Partical {
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
use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;

pub struct Projectile {
    body: Body
}

impl Projectile {
    pub fn new() -> Drone {
        Drone {
            body: Body::new(500.0, 300.0, 132.0, 128.0, 0.5, 0.5, (f32::consts::PI*3.0)/2.0, true)
        }
    }

    fn get_draw_param(&self, interpolation_value: f32, camera: &Camera) -> graphics::DrawParam  {
        let body = &self.body;
        let movement_vector = body.get_movement_vector();
        let view_position = camera.get_view_position(&Vector2::new(
                body.pos.x + movement_vector[0]*interpolation_value,
                body.pos.y + movement_vector[1]*interpolation_value
            )
        );
        graphics::DrawParam {
            dest: Point2::new(view_position.x, view_position.y),
            rotation: body.rotation,
            scale: Point2::new(body.scale.x, body.scale.y),
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        }
    }
}

impl Entity for Projectile {
    fn update(&mut self) {
        
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        graphics::draw_ex(
            ctx,
            &asset_manager.drone1,
            self.get_draw_param(interpolation_value, camera)
        ).unwrap();
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

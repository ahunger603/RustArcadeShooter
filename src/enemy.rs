use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;

pub struct Enemy {
    asset_key: String,
    body: Body
}

impl Enemy {
    fn new(x: f32, y: f32, asset_key: String) -> Enemy {
        Enemy {
            asset_key,
            body: Body::new(x, y, 132.0, 128.0, 0.5, 0.5, (f32::consts::PI*3.0)/2.0, true)
        }
    }

    pub fn new_drone(x:f32, y: f32) -> Enemy {
        Enemy::new(x, y, "drone1".to_string())
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

impl Entity for Enemy {
    fn update(&mut self) {
        
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        asset_manager.draw_asset(self.asset_key.clone(), ctx, self.get_draw_param(interpolation_value, camera));
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

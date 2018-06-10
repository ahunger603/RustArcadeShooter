use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;

pub struct Partical {
    is_dead: bool,
    asset_key: String,
    sheet_w: u32,
    sheet_h: u32,
    animation_progress: u32,
    body: Body
}

impl Partical {
    fn new(x: f32, y: f32, scale_x: f32, scale_y: f32, rotation: f32, asset_key: String, sheet_w: u32, sheet_h: u32) -> Partical {
        Partical {
            is_dead: false,
            asset_key,
            sheet_w,
            sheet_h,
            animation_progress: 0,
            body: Body::new(x, y, 0.0, 0.0, scale_x, scale_y, rotation, false)
        }
    }

    pub fn new_drone_death(x:f32, y: f32) -> Partical {
        Partical::new(x, y, 1.5, 1.5, f32::consts::PI/2.0, "explosion1".to_string(), 8, 8)
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
            src: graphics::Rect {
                x: (self.animation_progress % self.sheet_w) as f32 / self.sheet_w as f32,
                y: (self.animation_progress / self.sheet_w) as f32 / self.sheet_h as f32,
                w: 1.0/self.sheet_w as f32,
                h: 1.0/self.sheet_h as f32
            },
            dest: Point2::new(view_position.x, view_position.y),
            rotation: body.rotation,
            scale: Point2::new(body.scale.x, body.scale.y),
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        }
    }
}

impl Entity for Partical {
    fn update(&mut self) {
        if self.animation_progress == (self.sheet_w * self.sheet_h) {
            self.set_dead();
        }
        else {
            self.animation_progress += 1;
        }
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

    fn is_dead(&self) -> bool {
        self.is_dead
    }

    fn set_dead(&mut self) {
        self.is_dead = true;
    }
}
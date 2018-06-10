use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;

pub struct Unit {
    pub is_dead: bool,
    pub asset_key: String,
    pub body: Body,
    sheet_w: u32,
    sheet_h: u32,
    animation_repeats: bool,
    animation_progress: u32
}

impl Unit {
    pub fn new(
            x: f32, 
            y: f32,
            w: f32,
            h: f32,
            scale_x: f32,
            scale_y: f32,
            collidable: bool,
            rotation: f32,
            asset_key: String,
            sheet_w: u32,
            sheet_h: u32,
            animation_repeats: bool) -> Unit {
        Unit {
            is_dead: false,
            asset_key,
            sheet_w,
            sheet_h,
            animation_repeats,
            animation_progress: 0,
            body: Body::new(x, y, w, h, scale_x, scale_y, rotation, false)
        }
    }

    fn get_default_draw_param(&self, interpolation_value: f32, camera: &Camera) -> graphics::DrawParam  {
        let view_position = self.body.get_view_position(interpolation_value, camera);
        graphics::DrawParam {
            src: graphics::Rect {
                x: (self.animation_progress % self.sheet_w) as f32 / self.sheet_w as f32,
                y: (self.animation_progress / self.sheet_w) as f32 / self.sheet_h as f32,
                w: 1.0/self.sheet_w as f32,
                h: 1.0/self.sheet_h as f32
            },
            dest: Point2::new(view_position.x, view_position.y),
            rotation: self.body.rotation,
            scale: Point2::new(self.body.scale.x, self.body.scale.y),
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        }
    }

    fn update_animation(&mut self) {
        if self.animation_progress == (self.sheet_w * self.sheet_h)-1  {
            if self.animation_repeats {
                self.animation_progress = 0;
            } else {
                self.set_dead();
            }
        }
        else {
            self.animation_progress += 1;
        }
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.body.velocity = velocity;
    }
}

impl Entity for Unit {
    fn update(&mut self) {
        self.update_animation();
        self.body.update_pos();
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        if !self.is_dead {
            asset_manager.draw_asset(self.asset_key.clone(), ctx, self.get_default_draw_param(interpolation_value, camera));
        }
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
use std::f32;
use ggez::*;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;

pub struct Player {
    move_dir: [bool; 4], //up, down, left, right
    body: Body
}

impl Player {
    pub fn new() -> Player {
        Player {
            move_dir: [false; 4],
            body: Body::new(300.0, 300.0, 10.0, 10.0, f32::consts::PI/2.0, true)
        }
    }

    fn get_draw_param(&self, interpolation_value: f32) -> graphics::DrawParam  {
        let body = &self.body;
        graphics::DrawParam {
            dest: Point2::new(body.pos.x, body.pos.y),
            rotation: body.rotation,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        }
    }

    fn get_movement_vector(&self) -> [i16; 2] {
        let mut movement_vec = [0; 2];
        if self.move_dir[0] {
            movement_vec[0] += 1;
        }
        if self.move_dir[1] {
            movement_vec[0] -= 1;
        }
        if self.move_dir[2] {
            movement_vec[1] += 1;
        }
        if self.move_dir[3] {
            movement_vec[1] -= 1;
        }
        movement_vec
    }

    pub fn move_dir(&mut self, dir: u16) {
        if dir > 4 {
            return;
        }
        self.move_dir[dir as usize] = true;
    }

    pub fn move_dir_cancel(&mut self, dir: u16) {
        if dir > 4 {
            return;
        }
        self.move_dir[dir as usize] = false;
    }
}

impl Entity for Player {
    fn update(&mut self) {
        println!("{:?}", self.get_movement_vector());
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32) {
        graphics::draw_ex(
            ctx,
            &asset_manager.player,
            self.get_draw_param(interpolation_value)
        ).unwrap();
    }

    fn get_body(&self) -> Option<Body> {
        Some(self.body.clone())
    }

    fn set_body(&mut self, body: Body) {
        self.body = body;
    }
}

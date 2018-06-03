use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;

pub struct Player {
    movement_speed: f32,
    move_dir: [bool; 4], //up, down, left, right
    body: Body
}

impl Player {
    pub fn new() -> Player {
        Player {
            movement_speed: 8.0,
            move_dir: [false; 4],
            body: Body::new(300.0, 300.0, 10.0, 10.0, f32::consts::PI/2.0, true)
        }
    }

    fn get_draw_param(&self, interpolation_value: f32) -> graphics::DrawParam  {
        let body = &self.body;
        let movement_vector = body.get_movement_vector();
        graphics::DrawParam {
            dest: Point2::new(body.pos.x + movement_vector[0]*interpolation_value, body.pos.y - movement_vector[1]*interpolation_value),
            rotation: body.rotation,
            offset: Point2::new(0.5, 0.5),
            .. Default::default()
        }
    }

    fn get_direction_vector(&self) -> [f32; 2] {
        let mut dir_vec: [f32; 2] = [0.0; 2];
        if self.move_dir[0] {
            dir_vec[0] += 1.0;
        }
        if self.move_dir[1] {
            dir_vec[0] -= 1.0;
        }
        if self.move_dir[2] {
            dir_vec[1] += 1.0;
        }
        if self.move_dir[3] {
            dir_vec[1] -= 1.0;
        }
        dir_vec
    }

    fn get_movement_velocity(&self) -> Vector2<f32> {
        let dir_vec = self.get_direction_vector();
        let move_speed = if dir_vec[0] != 0.0 || dir_vec[1] != 0.0 {
            self.movement_speed
        } else {
            0.0
        };
        let direction_angle = dir_vec[0].atan2(dir_vec[1]);
        Vector2::new(move_speed, direction_angle)
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
        self.body.velocity = self.get_movement_velocity();
        let movement_vector = self.body.get_movement_vector();
        self.body.pos.x += movement_vector[0];
        self.body.pos.y -= movement_vector[1];
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

use std::time::{Instant, Duration};
use std::f32;
use ggez::*;
use nalgebra::Vector2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;
use super::unit::*;


pub struct Player {
    pub last_death: Instant,
    movement_speed: f32,
    move_dir: [bool; 4], //up, down, left, right
    unit: Unit
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            last_death: Instant::now(),
            movement_speed: 8.0,
            move_dir: [false; 4],
            unit: Unit::new(
                Body::new(x, y, 136.0, 96.0, 0.75, 0.75, f32::consts::PI/2.0, true),
                "player".to_string(), 1, 1, true
            )
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

    pub fn set_alive(&mut self) {
        self.unit.is_dead = false;
    }
}

impl Entity for Player {
    fn update(&mut self) {
        let movement_velocity = self.get_movement_velocity();
        self.unit.set_velocity(movement_velocity);
        self.unit.update();
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        self.unit.draw(asset_manager, ctx, interpolation_value, camera);
    }

    fn get_body(&self) -> Body {
        self.unit.get_body()
    }

    fn set_body(&mut self, body: Body) {
        self.unit.set_body(body)
    }

    fn is_dead(&self) -> bool {
        self.unit.is_dead
    }

    fn set_dead(&mut self) {
        self.last_death = Instant::now();
        self.unit.set_dead()
    }
}

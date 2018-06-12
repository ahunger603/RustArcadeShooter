/* Copyright (c) 2018 Ashton Hunger
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms. */

use nalgebra::Vector2;

pub struct Camera {
    pub pos: Vector2<f32>, //x, y
    pub size: Vector2<u32>, //width, height
}

impl Camera {
    pub fn new(w: u32, h: u32) -> Camera {
        Camera {
            pos: Vector2::new(0.0, 0.0),
            size: Vector2::new(w, h)
        }
    }
    
    pub fn get_view_position(&self, pos: &Vector2<f32>) -> Vector2<f32> {
        Vector2::new(
            pos.x - self.pos.x,
            self.size[1] as f32 - (pos.y - self.pos.y)
        )
    }
}
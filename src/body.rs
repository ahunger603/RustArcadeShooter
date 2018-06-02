extern crate nalgebra;

use nalgebra::Vector2;

pub struct Body {
    pub pos: Vector2<f32>, //x, y
    pub size: Vector2<f32>, //width, height
    pub velocity: Vector2<f32>, //speed, angle
    pub rotation: f32,
    pub collidable: bool
}

impl Body {
    pub fn new(x: f32, y: f32, width: f32, height: f32, rotation: f32, collidable: bool) -> Body {
        Body {
            pos: Vector2::new(x, y),
            size: Vector2::new(width, height),
            collidable,
            velocity: Vector2::new(0.0, 0.0),
            rotation
        }
    }

    pub fn get_movement_vector(&self) -> Vector2<f32> {
        Vector2::new(
            self.velocity[1].cos()*self.velocity[0],
            self.velocity[1].sin()*self.velocity[0]
        )
    }
}

impl Clone for Body {
    fn clone(&self) -> Self {
        Body {
            pos: Vector2::new(self.pos.x, self.pos.y),
            size: Vector2::new(self.size.x, self.size.y),
            collidable: self.collidable,
            velocity: Vector2::new(self.velocity.x, self.velocity.y),
            rotation: self.rotation
        }
    }
}

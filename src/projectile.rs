use std::f32;
use ggez::*;
use nalgebra::Vector2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;
use super::unit::*;

pub struct Projectile {
    unit: Unit,
    player_owned: bool
}

impl Projectile {
    pub fn new(x: f32, y: f32, player_owned: bool) -> Projectile {
        let rotation = if player_owned {f32::consts::PI/2.0} else {(f32::consts::PI*3.0)/2.0};
        let mut body = Body::new(x, y, 64.0, 32.0, 0.5, 0.5, rotation, true);
        body.velocity = Vector2::new(15.0, if player_owned {0.0} else {f32::consts::PI});

        Projectile {
            unit: Unit::new(body, "projectile1".to_string(), 1, 1, true),
            player_owned
        }
    }
    
    pub fn is_player_owned(&self) -> bool {
        self.player_owned
    }
}

impl Entity for Projectile {
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

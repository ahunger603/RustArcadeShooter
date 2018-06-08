use std::f32;
use ggez::*;
use nalgebra::Vector2;
use nalgebra::Point2;
use super::entity::*;
use super::body::*;
use super::asset_manager::*;
use super::camera::*;

pub struct Projectile {
    is_dead: bool,
    body: Body,
    player_owned: bool
}

impl Projectile {
    pub fn new(x: f32, y: f32, player_owned: bool) -> Projectile {
        let rotation = if player_owned {
            f32::consts::PI/2.0
        } else {
            (f32::consts::PI*3.0)/2.0
        };
        let mut body = Body::new(x, y, 64.0, 32.0, 0.5, 0.5, rotation, true);
        body.velocity = Vector2::new(15.0, 0.0);

        Projectile {
            is_dead: false,
            body,
            player_owned
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

    pub fn is_player_owned(&self) -> bool {
        self.player_owned
    }
}

impl Entity for Projectile {
    fn update(&mut self) {
        let movement_vector = self.body.get_movement_vector();
        self.body.pos.x += movement_vector[0];
        self.body.pos.y += movement_vector[1];
    }

    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        asset_manager.draw_asset("projectile1".to_string(), ctx, self.get_draw_param(interpolation_value, camera));
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

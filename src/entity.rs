use ggez::*;
use super::body::*;
use super::asset_manager::*;

pub trait Entity {
    fn update(&mut self);
    fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32);
    fn get_body(&self) -> Option<Body>;
    fn set_body(&mut self, body: Body);
}
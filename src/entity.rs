use ggez::*;
use super::body::*;

pub trait Entity {
    fn update(&mut self);
    fn draw(&self, ctx: &mut Context, interpolation_value: f32);
    fn get_body(&self) -> Option<Body>;
    fn set_body(&mut self, body: Body);
}
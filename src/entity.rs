use ggez::*;
use super::body::*;

pub trait Entity {
    fn update(&mut self);
    fn draw(&self, ctx: &mut Context);
    fn get_body(&self) -> Option<Body>;
    fn set_body(&mut self, body: Body);
}
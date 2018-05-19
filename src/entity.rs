use ggez::*;

pub trait Entity {
    fn update();
    fn draw(ctx: &mut Context);
}
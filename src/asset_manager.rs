use ggez::{graphics, Context, GameResult};


pub struct AssetManager {
    pub player: graphics::Image,
    pub drone1: graphics::Image,
    pub projectile1: graphics::Image
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> GameResult<AssetManager> {
        Ok(AssetManager {
            player: graphics::Image::new(ctx, "/playerFighter.png").unwrap(),
            drone1: graphics::Image::new(ctx, "/drone1.png").unwrap(),
            projectile1: graphics::Image::new(ctx, "/projectile1.png").unwrap()
        })
    }
}
use ggez::{graphics, Context, GameResult};


pub struct AssetManager {
    pub player: graphics::Image
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> GameResult<AssetManager> {
        Ok(AssetManager {
            player: graphics::Image::new(ctx, "/playerFighter.png").unwrap()
        })
    }
}
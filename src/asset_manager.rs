use ggez::{graphics, Context, GameResult};


pub struct AssetManager {
    pub player: graphics::Image,
    pub drone1: graphics::Image,
    pub projectile1: graphics::Image,
    pub explosion1: graphics::Image
}

impl AssetManager {
    pub fn new(ctx: &mut Context) -> GameResult<AssetManager> {
        Ok(AssetManager {
            player: graphics::Image::new(ctx, "/playerFighter.png").unwrap(),
            drone1: graphics::Image::new(ctx, "/drone1.png").unwrap(),
            projectile1: graphics::Image::new(ctx, "/projectile1.png").unwrap(),
            explosion1: graphics::Image::new(ctx, "/explosion1.png").unwrap()
        })
    }

    pub fn draw_asset(&self, asset_key: String, ctx: &mut Context, draw_param: graphics::DrawParam) {
        graphics::draw_ex(
            ctx,
            match asset_key.as_str() {
                "player" => &self.player,
                "drone1" => &self.drone1,
                "projectile1" => &self.projectile1,
                "explosion1" => &self.explosion1,
                _ => &self.projectile1
            },
            draw_param
        ).unwrap();
    }

    pub fn get_asset_dim(&self, asset_key: String) -> graphics::Rect {
        match asset_key.as_str() {
            "player" => &self.player,
            "drone1" => &self.drone1,
            "projectile1" => &self.projectile1,
            "explosion1" => &self.explosion1,
            _ => &self.projectile1
        }.get_dimensions()
    }
}
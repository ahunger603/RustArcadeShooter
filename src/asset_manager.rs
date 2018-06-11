use ggez::{graphics, Context, GameResult};
use nalgebra::{Point2};


pub struct AssetManager {
    window_w: u32,
    window_h: u32,
    pub large_splash_font: graphics::Font,
    pub med_splash_font: graphics::Font,
    pub player: graphics::Image,
    pub drone1: graphics::Image,
    pub projectile1: graphics::Image,
    pub explosion1: graphics::Image
}

impl AssetManager {
    pub fn new(ctx: &mut Context, window_w: u32, window_h: u32) -> GameResult<AssetManager> {
        Ok(AssetManager {
            window_w,
            window_h,
            large_splash_font: graphics::Font::new(ctx, "/fonts/OpenSans-ExtraBold.ttf", 48).unwrap(),
            med_splash_font: graphics::Font::new(ctx, "/fonts/OpenSans-Bold.ttf", 24).unwrap(),
            player: graphics::Image::new(ctx, "/playerFighter.png").unwrap(),
            drone1: graphics::Image::new(ctx, "/drone1.png").unwrap(),
            projectile1: graphics::Image::new(ctx, "/projectile1.png").unwrap(),
            explosion1: graphics::Image::new(ctx, "/explosion1.png").unwrap()
        })
    }

    pub fn get_asset(&self, asset_key: String) -> &graphics::Image {
        match asset_key.as_str() {
            "player" => &self.player,
            "drone1" => &self.drone1,
            "projectile1" => &self.projectile1,
            "explosion1" => &self.explosion1,
            _ => &self.projectile1
        }
    }

    pub fn draw_asset(&self, asset_key: String, ctx: &mut Context, draw_param: graphics::DrawParam) {
        graphics::draw_ex(
            ctx,
            self.get_asset(asset_key),
            draw_param
        ).unwrap();
    }

    fn draw_anchored_text(ctx: &mut Context, text: &graphics::Text, anchor: Point2<f32>) {
        graphics::draw(ctx, text, anchor, 0.0).unwrap();
    }

    pub fn draw_centered_text(&self, ctx: &mut Context, text: graphics::Text) {
        AssetManager::draw_anchored_text(ctx, &text, Point2::new(
            (self.window_w / 2) as f32 - (text.width() / 2) as f32,
            (self.window_h / 2) as f32 - (text.height() / 2) as f32
        ))
    }

    pub fn draw_bottom_centered_text(&self, ctx: &mut Context, text: graphics::Text) {
        AssetManager::draw_anchored_text(ctx, &text, Point2::new(
            (self.window_w / 2) as f32 - (text.width() / 2) as f32,
            self.window_h as f32 - (text.height() as f32) - 10.0
        ))
    }

    pub fn draw_top_centered_text(&self, ctx: &mut Context, text: graphics::Text) {
        AssetManager::draw_anchored_text(ctx, &text, Point2::new(
            (self.window_w / 2) as f32 - (text.width() / 2) as f32, 2.0
        ))
    }

    pub fn draw_top_left_text(&self, ctx: &mut Context, text: graphics::Text) {
        AssetManager::draw_anchored_text(ctx, &text, Point2::new(5.0, 2.0))
    }

    pub fn draw_top_right_text(&self, ctx: &mut Context, text: graphics::Text) {
        AssetManager::draw_anchored_text(ctx, &text, Point2::new(
            self.window_w as f32 - (text.width() as f32) - 5.0,
            2.0
        ))
    }

    pub fn get_asset_dim(&self, asset_key: String) -> graphics::Rect {
        self.get_asset(asset_key).get_dimensions()
    }
}
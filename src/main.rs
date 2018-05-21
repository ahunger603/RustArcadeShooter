extern crate ggez;
extern crate ncollide;
extern crate nalgebra;

use ggez::*;
use ggez::conf::*;

mod game_state;
mod body;
mod entity_manager;
mod player;

fn main() {
    let cb = ContextBuilder::new("RustArcadeShooter", "Infinity")
        .window_setup(conf::WindowSetup::default().title("Rust Arcade Shooter"))
        .window_mode(conf::WindowMode::default()
            .dimensions(640, 480)
            .fullscreen_type(FullscreenType::Off)
        );

    let ctx = &mut cb.build().unwrap();

    let game_state = &mut game_state::GameState::new(ctx).unwrap();
    
    event::run(ctx, game_state).unwrap();
}

/* Copyright (c) 2018 Ashton Hunger
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms. */

extern crate ggez;
extern crate ncollide;
extern crate nalgebra;
extern crate rand;

use std::env;
use std::path;
use ggez::*;
use ggez::conf::*;

mod game_event_handler;
mod body;
mod entity;
mod entity_manager;
mod asset_manager;
mod player;
mod enemy;
mod camera;
mod projectile;
mod particals;
mod unit;
mod wave_manager;
mod play_space;

const WINDOW_W: u32 = 640;
const WINDOW_H: u32 = 480;

fn get_context_builder() -> Option<ContextBuilder> {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut cb = ContextBuilder::new("RustArcadeShooter", "Infinity")
            .window_setup(conf::WindowSetup::default().title("Rust Arcade Shooter"))
            .window_mode(conf::WindowMode::default()
                .dimensions(WINDOW_W, WINDOW_H)
                .fullscreen_type(FullscreenType::Off)
                .vsync(false)
            );
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        cb = cb.add_resource_path(path);

        return Some(cb);
    }
    None
}

fn main() {
    if let Some(cb) = get_context_builder() {
        let ctx = &mut cb.build().unwrap();

        let game_state = &mut game_event_handler::GameEventHandler::new(ctx, WINDOW_W, WINDOW_H).unwrap();

        event::run(ctx, game_state).unwrap();
    } else {
        eprintln!("Failed to create context builder! Game exiting..");
    }
}

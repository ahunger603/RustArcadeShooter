use ggez::*;
use nalgebra::{Isometry2, Vector2, Point};
use ncollide::shape::Cuboid2;
use ncollide::bounding_volume;
use ncollide::bounding_volume::BoundingVolume;

use super::asset_manager::*;
use super::entity::*;
use super::player::*;
use super::drone::*;
use super::camera::*;

struct Projectile {
    projectile: Box<Entity>,
    player_owned: bool
}

pub struct EntityManager {
    player: Player,
    projectiles: Vec<Projectile>,
    enemies: Vec<Box<Entity>>
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            player: Player::new(),
            projectiles: vec![],
            enemies: vec![Box::new(Drone::new())]
        }
    }

    pub fn update(&mut self) {
        self.player.update();
        for enemy in &mut self.enemies {
            enemy.update();
        }
        self.collision_resolution();
    }

    fn create_entity_collision_area(entity: &Entity) -> Option<bounding_volume::AABB<Point<f32, nalgebra::U2>>> {
        if let Some(body) = entity.get_body() {
            return if body.collidable {
                Some(bounding_volume::aabb(
                    &Cuboid2::new(body.get_scaled_size()),
                    &Isometry2::new(
                        Vector2::new(body.pos.x, body.pos.y), 
                        0.0)
                    )
                )
            } else {
                None
            }
        }
        None
    }

    fn collision_resolution(&mut self) {
        if let Some(player_col_area) = EntityManager::create_entity_collision_area(&self.player) {
            for enemy in &mut self.enemies {
                EntityManager::resolve_player_enemy_collision(&player_col_area, enemy);
            }
        }
    }

    fn resolve_player_enemy_collision(player_col_area: &bounding_volume::AABB<Point<f32, nalgebra::U2>>, enemy: &Box<Entity>) {
        if let Some(enemy_col_area) = EntityManager::create_entity_collision_area(&(*(*enemy))) {
            if player_col_area.intersects(&enemy_col_area) {
                println!("COLLISION");
            }
        }
    }

    pub fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        self.player.draw(asset_manager, ctx, interpolation_value, camera);
        for enemy in &self.enemies {
            enemy.draw(asset_manager, ctx, interpolation_value, camera);
        }
    }

    pub fn player_move(&mut self, dir: u16) {
        self.player.move_dir(dir);
    }

    pub fn player_move_cancel(&mut self, dir: u16) {
        self.player.move_dir_cancel(dir);
    }
}
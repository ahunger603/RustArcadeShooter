use ggez::*;
use std::time::{Instant};
use nalgebra::{Isometry2, Vector2, Point, Point2, Id};
use ncollide::shape::Cuboid2;
use ncollide::bounding_volume;
use ncollide::bounding_volume::BoundingVolume;
use ncollide::query::PointQuery;

use super::asset_manager::*;
use super::entity::*;
use super::player::*;
use super::enemy::*;
use super::camera::*;
use super::projectile::*;
use super::particals::*;
use super::play_space::*;
use super::game_event_handler::GameState;

pub struct EntityManager;

impl EntityManager {
    pub fn is_player_alive(game_state: &GameState) -> bool {
        !game_state.player.is_dead()
    }

    pub fn respawn_player(game_state: &mut GameState) {
        let mut body = game_state.player.get_body();
        body.pos = Vector2::new(game_state.play_space.player_area.w / 3.0, game_state.play_space.player_area.h / 2.0);
        game_state.player.set_body(body);
        game_state.player.set_alive();
    }

    pub fn get_player_last_death(game_state: &GameState) -> Instant {
        game_state.player.last_death
    }

    pub fn add_enemy(game_state: &mut GameState, enemy: Enemy) {
        game_state.enemies.push(enemy);
    }

    pub fn get_enemy_count(game_state: &GameState) -> u32 {
        game_state.enemies.len() as u32
    }

    pub fn update(game_state: &mut GameState) {
        game_state.player.update();
        for enemy in &mut game_state.enemies {
            enemy.update();
        }
        for projectile in game_state.projectiles.iter_mut(){
            projectile.update();
        } 
        for partical in game_state.particals.iter_mut(){
            partical.update();
        } 
        EntityManager::collision_resolution(game_state);
        EntityManager::update_clean_up(game_state);
    }

    pub fn update_life_lost(game_state: &mut GameState) -> u32 {
        let mut lost = 0;
        for enemy in &mut game_state.enemies {
            if let Some(enemy_area) = EntityManager::create_entity_collision_area(enemy) {
                if game_state.play_space.life_loss_area_aabb.contains(&enemy_area) {
                    lost += 1;
                    enemy.set_dead();
                }
            }
        }
        lost
    }

    fn create_entity_collision_area(entity: &Entity) -> Option<bounding_volume::AABB<Point<f32, nalgebra::U2>>> {
        let body = entity.get_body();
        if body.collidable {
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

    fn ship_hit_by_projectile(ship: &mut Entity, projectile: &mut Projectile, particals: &mut Vec<Partical>) {
        projectile.set_dead(); 
        EntityManager::ship_death(ship, particals);
    }

    fn ship_death(ship: &mut Entity, particals: &mut Vec<Partical>) {
        ship.set_dead(); 
        let ship_body = ship.get_body();
        particals.push(Partical::new_drone_death(ship_body.pos.x + (ship_body.size.x / 1.5), ship_body.pos.y));
    }

    fn collision_resolution(game_state: &mut GameState) {
        if let Some(player_col_area) = EntityManager::create_entity_collision_area(&game_state.player) {
            if EntityManager::is_player_alive(game_state) {
                for enemy in &mut game_state.enemies {
                    if EntityManager::is_col_area_entity_collision(&player_col_area, enemy) {
                        //Player hit by ship
                        EntityManager::ship_death(&mut game_state.player, &mut game_state.particals);
                        EntityManager::ship_death(enemy, &mut game_state.particals);
                    }
                }
            }

            for projectile in game_state.projectiles.iter_mut(){
                if let Some(projectile_col_area) = EntityManager::create_entity_collision_area(projectile) {
                    if projectile.is_player_owned() {
                        for enemy in &mut game_state.enemies {
                            if EntityManager::is_col_area_entity_collision(&projectile_col_area, enemy) {
                                //Enemy hit by projectile
                                EntityManager::ship_hit_by_projectile(enemy, projectile, &mut game_state.particals);
                                game_state.score += 150;
                            }
                        }
                    } else {
                        if projectile_col_area.intersects(&player_col_area) {
                            //Player hit by projectile
                            EntityManager::ship_hit_by_projectile(&mut game_state.player, projectile, &mut game_state.particals);
                        }
                    }
                }
            }
        }
    }

    fn retain_entity(play_space: &PlaySpace, entity: &Entity) -> bool {
        if entity.is_dead() {
            false
        } else {
            let body_pos = entity.get_body().pos;
            if play_space.entity_area_aabb.contains_point(&Id::new(), &Point2::new(body_pos.x, body_pos.y)) {
                true
            } else {
                false
            }
        }
    }

    fn update_clean_up(game_state: &mut GameState) {
        let play_space = game_state.play_space.clone();
        game_state.enemies.retain(|enemy| EntityManager::retain_entity(&play_space, enemy));
        game_state.particals.retain(|partical| EntityManager::retain_entity(&play_space, partical));
        game_state.projectiles.retain(|projectile| EntityManager::retain_entity(&play_space, projectile));
    }

    fn is_col_area_entity_collision(col_area: &bounding_volume::AABB<Point<f32, nalgebra::U2>>, entity: &Enemy) -> bool {
        if let Some(entity_col_area) = EntityManager::create_entity_collision_area(entity) {
            if col_area.intersects(&entity_col_area) {
                return true;
            }
        }
        false
    }

    pub fn draw(game_state: &GameState, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        for projectile in game_state.projectiles.iter(){
            projectile.draw(asset_manager, ctx, interpolation_value, camera);
        }
        for enemy in &game_state.enemies {
            enemy.draw(asset_manager, ctx, interpolation_value, camera);
        }
        game_state.player.draw(asset_manager, ctx, interpolation_value, camera);

        for partical in &game_state.particals {
            partical.draw(asset_manager, ctx, interpolation_value, camera);
        }
    }

    pub fn player_fire(game_state: &mut GameState) {
        if EntityManager::is_player_alive(game_state) {
            let player_body = game_state.player.get_body();
            game_state.projectiles.push(
                Projectile::new(player_body.pos.x, player_body.pos.y, true)
            );
        }
    }

    pub fn enemy_fire(game_state: &mut GameState, x: f32, y: f32) {
        game_state.projectiles.push(
            Projectile::new(x, y, false)
        );
    }

    pub fn player_move(game_state: &mut GameState, dir: u16) {
        if EntityManager::is_player_alive(game_state) {
            game_state.player.move_dir(dir);
        }
    }

    pub fn player_move_cancel(game_state: &mut GameState, dir: u16) {
        if EntityManager::is_player_alive(game_state) {
            game_state.player.move_dir_cancel(dir);
        }
    }
}
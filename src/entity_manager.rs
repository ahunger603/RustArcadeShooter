use ggez::*;
use nalgebra::{Isometry2, Vector2, Point};
use ncollide::shape::Cuboid2;
use ncollide::bounding_volume;
use ncollide::bounding_volume::BoundingVolume;

use super::asset_manager::*;
use super::entity::*;
use super::player::*;
use super::enemy::*;
use super::camera::*;
use super::projectile::*;
use super::particals::*;

pub struct EntityManager {
    player: Player,
    projectiles: Vec<Projectile>,
    enemies: Vec<Enemy>,
    particals: Vec<Partical>
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            player: Player::new(),
            projectiles: vec![],
            enemies: vec![Enemy::new_drone(500.0, 300.0)],
            particals: vec![]
        }
    }

    pub fn update(&mut self) {
        self.player.update();
        for enemy in &mut self.enemies {
            enemy.update();
        }
        for projectile in self.projectiles.iter_mut(){
            projectile.update();
        } 
        for partical in self.particals.iter_mut(){
            partical.update();
        } 
        self.collision_resolution();
        self.update_clean_up();
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

    fn enemy_hit(enemy: &mut Enemy, projectile: &mut Projectile, particals: &mut Vec<Partical>) {
        enemy.set_dead();
        projectile.set_dead(); 
        if let Some(body) = enemy.get_body() {
            particals.push(Partical::new_drone_death(body.pos.x + (body.size.x / 1.5), body.pos.y));
        }
    }

    fn collision_resolution(&mut self) {
        if let Some(player_col_area) = EntityManager::create_entity_collision_area(&self.player) {
            for enemy in &mut self.enemies {
                if EntityManager::is_col_area_entity_collision(&player_col_area, enemy) {
                    println!("PLAYER COLLISION");
                }
            }

            for projectile in self.projectiles.iter_mut(){
                if let Some(projectile_col_area) = EntityManager::create_entity_collision_area(projectile) {
                    if projectile.is_player_owned() {
                        for enemy in &mut self.enemies {
                            if EntityManager::is_col_area_entity_collision(&projectile_col_area, enemy) {
                                  EntityManager::enemy_hit(enemy, projectile, &mut self.particals); 
                            }
                        }
                    } else {
                        if projectile_col_area.intersects(&player_col_area) {
                            println!("ENEMY PROJECTILE COLLISION");
                        }
                    }
                }
            }
        }
    }

    fn clean_up_enemies(&mut self) {
        let mut new_enemy_list: Vec<Enemy> = vec![];
        while self.enemies.len() > 0 {
            if let Some(enemy) = self.enemies.pop() {
                if !enemy.is_dead() {
                    new_enemy_list.push(enemy);
                }
            }
            else {
                break;
            }
        }
        self.enemies = new_enemy_list;
    }

    fn clean_up_particals(&mut self) {
        let mut new_partical_list: Vec<Partical> = vec![];
        while self.particals.len() > 0 {
            if let Some(partical) = self.particals.pop() {
                if !partical.is_dead() {
                    new_partical_list.push(partical);
                }
            }
            else {
                break;
            }
        }
        self.particals = new_partical_list;
    }

    fn clean_up_projectiles(&mut self) {
        let mut new_projectile_list: Vec<Projectile> = vec![];
        while self.projectiles.len() > 0 {
            if let Some(projectile) = self.projectiles.pop() {
                if !projectile.is_dead() {
                    new_projectile_list.push(projectile);
                }
            }
            else {
                break;
            }
        }
        self.projectiles = new_projectile_list;
    }

    fn update_clean_up(&mut self) {
        self.clean_up_enemies();
        self.clean_up_particals();
        self.clean_up_projectiles();
    }

    fn is_col_area_entity_collision(col_area: &bounding_volume::AABB<Point<f32, nalgebra::U2>>, entity: &Enemy) -> bool {
        if let Some(entity_col_area) = EntityManager::create_entity_collision_area(entity) {
            if col_area.intersects(&entity_col_area) {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, asset_manager: &AssetManager, ctx: &mut Context, interpolation_value: f32, camera: &Camera) {
        for projectile in self.projectiles.iter(){
            projectile.draw(asset_manager, ctx, interpolation_value, camera);
        }
        for enemy in &self.enemies {
            enemy.draw(asset_manager, ctx, interpolation_value, camera);
        }
        self.player.draw(asset_manager, ctx, interpolation_value, camera);

        for partical in &self.particals {
            partical.draw(asset_manager, ctx, interpolation_value, camera);
        }
    }

    pub fn player_fire(&mut self) {
        if let Some(player_body) = self.player.get_body() {
            self.projectiles.push(
                Projectile::new(player_body.pos.x, player_body.pos.y, true)
            );
        }
    }

    pub fn player_move(&mut self, dir: u16) {
        self.player.move_dir(dir);
    }

    pub fn player_move_cancel(&mut self, dir: u16) {
        self.player.move_dir_cancel(dir);
    }
}
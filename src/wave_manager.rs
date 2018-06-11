use std::time::{Instant, Duration};
use nalgebra::Vector2;
use rand::{Rng, thread_rng};

use super::entity_manager::*;
use super::enemy::*;
use super::play_space::*;

struct Wave {
    pub spawn_rate: u32,
    pub spawn_delay_ms: u64,
    pub remaining_enemies: Vec<EnemyType>
}

pub struct WaveManager {
    progress_wave: bool,
    spawn_origin: Vector2<f32>,
    spawn_range: f32,
    current_wave_level: u32,
    current_wave: Wave,
    last_spawn: Instant
}

impl WaveManager {
    pub fn new(play_space: PlaySpace) -> WaveManager {
        WaveManager {
            progress_wave: false,
            spawn_origin: Vector2::new(play_space.player_area.w + 20.0, play_space.player_area.h / 2.0),
            spawn_range: play_space.player_area.h - 100.0,
            current_wave_level: 1,
            current_wave: WaveManager::create_wave(1),
            last_spawn: Instant::now()
        }
    }

    fn get_random_spawn_point(&self) -> Vector2<f32> {
        let rand_rang: f32 = thread_rng().gen();
        Vector2::new(self.spawn_origin.x - 25.0, self.spawn_origin.y + self.spawn_range*(rand_rang - 0.5))
    }

    fn spawn(&mut self, entity_manager: &mut EntityManager) {
        for _i in 0..(self.current_wave.spawn_rate) {
            if let Some(enemy_type) = self.current_wave.remaining_enemies.pop() {
                let spawn_point = self.get_random_spawn_point();
                if let Some(enemy) = Enemy::create_enemy_by_key(enemy_type, spawn_point.x, spawn_point.y) {
                    entity_manager.add_enemy(enemy);
                }
            }
        }
    }

    fn update_wave_level(&mut self) {
        if self.wave_spawn_complete() && self.progress_wave {
            self.current_wave_level += 1; 
            self.current_wave = WaveManager::create_wave(self.current_wave_level);
            self.progress_wave = false;
        }
    }

    pub fn update(&mut self, entity_manager: &mut EntityManager) {
        self.update_wave_level();
        if self.last_spawn.elapsed() > Duration::from_millis(self.current_wave.spawn_delay_ms) {
            self.spawn(entity_manager);
            self.last_spawn = Instant::now();
        }
    }

    pub fn wave_spawn_complete(&self) -> bool {
        self.current_wave.remaining_enemies.len() == 0
    }

    pub fn set_to_progress_level(&mut self) {
        self.progress_wave = true;
    }

    pub fn get_wave_level(&self) -> u32 {
        self.current_wave_level
    }

    fn create_wave(wave_level: u32) -> Wave {
        Wave {
            spawn_rate: 1,
            spawn_delay_ms: 1000,
            remaining_enemies: vec![
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone
            ]
        }
    }
}
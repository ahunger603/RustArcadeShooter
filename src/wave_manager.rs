use std::time::{Instant, Duration};
use nalgebra::Vector2;
use rand::{Rng, thread_rng};
use super::entity_manager::*;
use super::enemy::*;

struct Wave {
    pub spawn_rate: u32,
    pub spawn_delay_ms: u64,
    pub remaining_enemies: Vec<EnemyType>
}

pub struct WaveManager {
    spawn_origin: Vector2<f32>,
    spawn_range: f32,
    current_wave_level: u32,
    current_wave: Wave,
    last_spawn: Instant
}

impl WaveManager {
    pub fn new(window_w: u32, window_h: u32) -> WaveManager {
        WaveManager {
            spawn_origin: Vector2::new(window_w as f32 + 200.0, window_h as f32 / 2.0),
            spawn_range: (window_h - 100) as f32,
            current_wave_level: 0,
            current_wave: WaveManager::create_wave(0),
            last_spawn: Instant::now()
        }
    }

    pub fn get_random_spawn_point(&self) -> Vector2<f32> {
        let rand_rang: f32 = thread_rng().gen();
        Vector2::new(self.spawn_origin.x, self.spawn_origin.y + self.spawn_range*(rand_rang - 0.5))
    }

    pub fn spawn(&mut self, entity_manager: &mut EntityManager) {
        for _i in 0..(self.current_wave.spawn_rate) {
            if let Some(enemy_type) = self.current_wave.remaining_enemies.pop() {
                let spawn_point = self.get_random_spawn_point();
                if let Some(enemy) = Enemy::create_enemy_by_key(enemy_type, spawn_point.x, spawn_point.y) {
                    entity_manager.add_enemy(enemy);
                }
            }
        }
    }

    pub fn update(&mut self, entity_manager: &mut EntityManager) {
        if self.last_spawn.elapsed() > Duration::from_millis(self.current_wave.spawn_delay_ms) {
            self.spawn(entity_manager);
            self.last_spawn = Instant::now();
        }
    }

    pub fn wave_spawn_complete(&self) -> bool {
        self.current_wave.remaining_enemies.len() == 0
    }

    fn create_wave(wave_level: u32) -> Wave {
        Wave {
            spawn_rate: 1,
            spawn_delay_ms: 1000,
            remaining_enemies: vec![
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone,
                EnemyType::NormalDrone
            ]
        }
    }
}
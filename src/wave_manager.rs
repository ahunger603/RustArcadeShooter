use std::time::{Instant, Duration};
use super::entity_manager::*;
use super::enemy::*;

struct Wave {
    pub spawn_rate: u32,
    pub spawn_delay_ms: u64,
    pub remaining_enemies: Vec<EnemyType>
}

pub struct WaveManager {
    current_wave_level: u32,
    current_wave: Wave,
    last_spawn: Instant
}

impl WaveManager {
    pub fn new() -> WaveManager {
        WaveManager {
            current_wave_level: 0,
            current_wave: WaveManager::create_wave(0),
            last_spawn: Instant::now()
        }
    }

    pub fn spawn(&mut self, entity_manager: &mut EntityManager) {
        for _i in 0..(self.current_wave.spawn_rate) {
            if let Some(enemy_type) = self.current_wave.remaining_enemies.pop() {
                if let Some(enemy) = Enemy::create_enemy_by_key(enemy_type, 700.0, 300.0) {
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
            spawn_delay_ms: 500,
            remaining_enemies: vec![
                EnemyType::NORMAL_DRONE,
                EnemyType::NORMAL_DRONE,
                EnemyType::NORMAL_DRONE,
                EnemyType::NORMAL_DRONE,
                EnemyType::NORMAL_DRONE
            ]
        }
    }
}
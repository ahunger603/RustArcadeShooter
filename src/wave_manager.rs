use std::time::{Instant, Duration};
use nalgebra::Vector2;
use rand::{Rng, thread_rng};

use super::entity_manager::*;
use super::enemy::*;
use super::play_space::*;
use super::game_event_handler::GameState;

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

    fn spawn(&mut self, game_state: &mut GameState) {
        for _i in 0..(self.current_wave.spawn_rate) {
            if let Some(enemy_type) = self.current_wave.remaining_enemies.pop() {
                let spawn_point = self.get_random_spawn_point();
                if let Some(enemy) = Enemy::create_enemy_by_key(enemy_type, spawn_point.x, spawn_point.y) {
                    EntityManager::add_enemy(game_state, enemy);
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

    pub fn update(&mut self, game_state: &mut GameState) {
        self.update_wave_level();
        if self.last_spawn.elapsed() > Duration::from_millis(self.current_wave.spawn_delay_ms) {
            self.spawn(game_state);
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
        let mut enemies: Vec<EnemyType> = Vec::new();
        for _i in 0..(wave_level*3) {
            enemies.push(EnemyType::NormalDrone)
        }
        Wave {
            spawn_rate: 1,
            spawn_delay_ms: 1000 - (wave_level as u64 * 100),
            remaining_enemies: enemies
        }
    }
}
/* Copyright (c) 2018 Ashton Hunger
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms. */

use ggez::graphics::{Rect};
use nalgebra::{Isometry2, Vector2, Point, U2};
use ncollide::shape::Cuboid2;
use ncollide::bounding_volume;

const ENTITY_AREA_BUFFER_SZ: f32 = 100.0;

pub struct PlaySpace {
    pub player_area: Rect,
    pub entity_area: Rect,
    pub player_area_aabb: bounding_volume::AABB<Point<f32, U2>>,
    pub entity_area_aabb: bounding_volume::AABB<Point<f32, U2>>,
    pub life_loss_area_aabb: bounding_volume::AABB<Point<f32, U2>>
}

impl PlaySpace {
    pub fn new(window_w: f32, window_h: f32) -> PlaySpace {
        let player_area = Rect {
            x: 0.0,
            y: window_h,
            w: window_w,
            h: window_h
        };
        let entity_area = Rect {
            x: player_area.x - ENTITY_AREA_BUFFER_SZ,
            y: player_area.y,
            w: player_area.w + (ENTITY_AREA_BUFFER_SZ*2.0),
            h: player_area.h
        };
        PlaySpace {
            player_area,
            entity_area,
            player_area_aabb: PlaySpace::create_aabb_from_rect(&player_area),
            entity_area_aabb: PlaySpace::create_aabb_from_rect(&entity_area),
            life_loss_area_aabb: PlaySpace::create_aabb_from_rect(&Rect {
                x: -window_w,
                y: window_h,
                w: window_w,
                h: window_h
            })
        }
    }

    fn create_aabb_from_rect(rect: &Rect) -> bounding_volume::AABB<Point<f32, U2>> {
        bounding_volume::aabb(&Cuboid2::new(Vector2::new(rect.w, rect.h)),&Isometry2::new(Vector2::new(rect.x, rect.y), 0.0))
    }
}

impl Clone for PlaySpace {
    fn clone(&self) -> Self {
        PlaySpace {
            player_area: self.player_area.clone(),
            entity_area: self.entity_area.clone(),
            player_area_aabb: self.player_area_aabb.clone(),
            entity_area_aabb: self.entity_area_aabb.clone(),
            life_loss_area_aabb: self.life_loss_area_aabb.clone()
        }
    }
}


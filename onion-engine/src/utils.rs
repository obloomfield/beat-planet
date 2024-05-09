use bevy::render::color::Color;

use crate::types::{Note, HitType};

const PERFECT_MS: i32 = 20;
const GREAT_MS: i32 = 50;
const GOOD_MS: i32 = 30;
const MISS_MS: i32 = 200;

pub const TOTAL_MS: i32 = PERFECT_MS + GREAT_MS + GOOD_MS + MISS_MS;

const PERFECT_SCORE: i32 = 1000;
const GREAT_SCORE: i32 = 500;
const GOOD_SCORE: i32 = 250;
const MISS_SCORE: i32 = -20;

pub fn calculate_hit(hit_time: i32, note_time: i32) -> HitType {
    print!("offset: {}, ", hit_time - note_time);
    let diff = (hit_time - note_time).abs();
    if diff <= PERFECT_MS {
        HitType::Perfect
    } else if diff <= PERFECT_MS + GREAT_MS {
        HitType::Great
    } else if diff <= PERFECT_MS + GREAT_MS + GOOD_MS {
        HitType::Good
    } else if diff <= TOTAL_MS {
        HitType::Miss
    } else {
        HitType::None 
    } 
}

pub fn calculate_score(hit: HitType) -> i32 {
    match hit {
        HitType::Perfect => PERFECT_SCORE,
        HitType::Great => GREAT_SCORE,
        HitType::Good => GOOD_SCORE,
        HitType::Miss => MISS_SCORE,
        HitType::None => 0,
    }
}

pub fn hit_color(hit: HitType) -> Color {
    match hit {
        HitType::Perfect => Color::rgb(0.0, 10.0, 0.0),
        HitType::Great => Color::rgb(0.0, 0.0, 10.0),
        HitType::Good => Color::rgb(10.0, 10.0, 0.0),
        HitType::Miss => Color::rgb(10.0, 0.0, 0.0),
        HitType::None => Color::rgb(1.0, 1.0, 1.0),
    }
}

// either hit or a miss, no further away.
pub fn active_notes(notes: &Vec<Note>, hit_time: f32) -> Vec<usize> {
    let mut filtered_notes = Vec::new();
    for (i, note) in notes.iter().enumerate() {
      let elapsed = (hit_time - note.time).abs();
      if elapsed < TOTAL_MS as f32 {
        filtered_notes.push(i);
      }
    }
    filtered_notes
}

pub fn decay_rgb(init_color: &Color, decay_rate: f32, min_val: f32, delta_time: f32) -> Color {
    Color::rgb(
        (init_color.r() - decay_rate * delta_time).max(min_val),
        (init_color.g() - decay_rate * delta_time).max(min_val),
        (init_color.b() - decay_rate * delta_time).max(min_val),
    )
}
use crate::types::{Note, HitType};

const PERFECT_MS: i32 = 20;
const GREAT_MS: i32 = 50;
const GOOD_MS: i32 = 80;
const MISS_MS: i32 = 100;

const TOTAL_MS: i32 = PERFECT_MS + GREAT_MS + GOOD_MS + MISS_MS;

const PERFECT_SCORE: i32 = 1000;
const GREAT_SCORE: i32 = 500;
const GOOD_SCORE: i32 = 250;
const MISS_SCORE: i32 = -20;

pub fn calculate_hit(hit_time: i32, note_time: i32) -> HitType {
    println!("hit_time: {}, note_time: {}", hit_time, note_time);
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

// either hit or a miss, no further away.
pub fn active_notes(notes: &Vec<Note>, hit_time: f32) -> Vec<Note> {
    let mut filtered_notes = Vec::new();
    for note in notes.iter() {
      let elapsed = (hit_time - note.time).abs();
      if elapsed < TOTAL_MS as f32 {
        filtered_notes.push(note.clone());
      }
    }
    filtered_notes
}

// pub fn notes_from_string(note_string: String) -> Vec<Note> {
//     let mut notes = Vec::new();
//     for line in note_string.lines() {
//         let parts: Vec<&str> = line.split(",").collect();
//         let lane = parts[0].parse::<usize>().unwrap();
//         let time = parts[1].parse::<f32>().unwrap();
//         notes.push(Note {
//             entity: None,
//             material: Default::default(),
//             lane,
//             time,
//         });
//     }
//     notes
// }
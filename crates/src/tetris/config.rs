use wasm_bindgen::prelude::wasm_bindgen;

use crate::tetris::locks::{LockDelay, LockDelayType};

#[wasm_bindgen]
pub struct Config {
    pub current_score: u32,
    pub current_level: u32,
    pub current_rows_completed: u32,

    pub width: u32,
    pub height: u32,
    pub next_goal_multiplier: u32,
    pub soft_drop_speed_multiplier: f64,
    pub speed_multiplier: f64,
    pub max_piece_queue_size: usize,
    pub piece_rotation_wait_time: f64,

    pub lock_delay_type: LockDelayType,
    pub lock_delay_timer: f64,
    pub lock_delay_max_resets: u32,

    pub entry_delay: f64,
}

impl Config {

    pub fn get_lock_delay(&self) -> LockDelay {
        LockDelay::from(self)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 10,
            height: 25,
            current_level: 1,
            current_score: 0,
            current_rows_completed: 0,
            next_goal_multiplier: 5,
            soft_drop_speed_multiplier: 0.05,
            speed_multiplier: 0.0007,
            max_piece_queue_size: 7,
            piece_rotation_wait_time: 250.0,
            lock_delay_type: LockDelayType::StepReset,
            lock_delay_timer: 500.0,
            lock_delay_max_resets: 15,
            entry_delay: 0.0,
        }
    }
}

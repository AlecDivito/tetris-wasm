use wasm_bindgen::prelude::wasm_bindgen;

use crate::tetris::{action::Action, config::Config};

/*
Lock Delay Types
Immediate Lock (Classic)

Piece waits a fixed number of frames before locking
TGM2: 15 frames, TGM3: 8 frames in Shirase Mode
You'd need a counter that decrements each frame
Entry Reset (Puyo-style)

Fixed delay, but timer pauses while piece falls, resets on new piece
Requires tracking whether the piece is falling vs. stationary
Step Reset (Sega-style)


Move Reset (Tetris Guideline)


*/
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum LockDelayType {
    /// The modern Tetris standard. Any valid move or rotation completely resets the
    /// lock timer, but it is capped by a total action limit to prevent players from
    /// stalling forever.
    CappedReset = "CAPPED_RESET",
    /// Used in older games like Tetris The Grand Master. The lock timer only resets
    /// if the piece drops down to a lower row (a lower Y-coordinate) than it has
    /// previously reached during that lock cycle.
    StepReset = "STEP_RESET",
    /// The retro standard seen in NES Tetris or Game Boy Tetris. There is no lock
    /// delay at all. The piece locks the exact millisecond it touches a surface.
    Immediate = "IMMEDIATE",
    /// An older variation of Infinite Lock Delay. It resets the timer on every
    /// single move or rotation, but features no maximum action cap. This allows
    /// players to stall a piece indefinitely at the bottom of the board.
    MoveReset = "MOVE_RESET",
}

const MAX_TIME_REDUCTION: f64 = 200.0;

pub struct LockDelay {
    reset_type: LockDelayType,
    max_allowed_ground_time: f64, // in milliseconds
    max_allowed_timer_resets: u32,
    accumulated_lock_time: f64,
    reset_count: u32,
}

impl LockDelay {
    pub fn reset(&mut self) {
        self.accumulated_lock_time = 0.0;
        self.reset_count = 0;
    }

    pub fn handle_move(&mut self) {
        match self.reset_type {
            LockDelayType::MoveReset  => {
                self.accumulated_lock_time = 0.0;
                self.reset_count = 0;
            }
            LockDelayType::CappedReset  => {
                if self.reset_count < self.max_allowed_timer_resets {
                    self.accumulated_lock_time = 0.0;
                    self.reset_count += 1 as u32;
                }
            }
            _ => {}
        }
    }

    pub fn handle_advance(&mut self) {
        match self.reset_type {
            LockDelayType::StepReset => {
                self.reset();
            }
            _ => {}
        }
    }

    pub fn is_locked(&self) -> bool {
        match self.reset_type {
            LockDelayType::Immediate => true,
            LockDelayType::MoveReset | LockDelayType::StepReset => {
                return self.accumulated_lock_time >= self.max_allowed_ground_time
            }
            LockDelayType::CappedReset => todo!(),
            _ => true,
        }
    }

    pub fn update(&mut self, elapsed_time: f64) {
        self.accumulated_lock_time += if elapsed_time > MAX_TIME_REDUCTION {
            MAX_TIME_REDUCTION
        } else {
            elapsed_time
        };
    }
}

impl From<&Config> for LockDelay {
    fn from(value: &Config) -> Self {
        Self {
            max_allowed_ground_time: value.lock_delay_timer,
            reset_type: value.lock_delay_type,
            max_allowed_timer_resets: value.lock_delay_max_resets,
            accumulated_lock_time: 0.0,
            reset_count: 0,
        }
    }
}

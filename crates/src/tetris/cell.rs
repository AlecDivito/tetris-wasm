use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use wasm_bindgen::prelude::wasm_bindgen;

/// A Cell is a byte representation of a possible pieces value
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
    EMPTY = 7,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'I' => Cell::I,
            'O' => Cell::O,
            'T' => Cell::T,
            'S' => Cell::S,
            'Z' => Cell::Z,
            'J' => Cell::J,
            'L' => Cell::L,
            _ => Cell::EMPTY,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            Cell::I => "I",
            Cell::O => "O",
            Cell::T => "T",
            Cell::S => "S",
            Cell::Z => "Z",
            Cell::J => "J",
            Cell::L => "L",
            Cell::EMPTY => " ",
        };
        write!(f, "{}", piece)
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I => write!(f, "I"),
            Self::O => write!(f, "O"),
            Self::T => write!(f, "T"),
            Self::S => write!(f, "S"),
            Self::Z => write!(f, "Z"),
            Self::J => write!(f, "J"),
            Self::L => write!(f, "L"),
            Self::EMPTY => write!(f, " "),
        }
    }
}

impl Cell {
    /// pick a random cell
    pub fn random() -> Cell {
        let piece = (js_sys::Math::random() * 6.0).round() as i32;
        match piece {
            0 => Cell::I,
            1 => Cell::O,
            2 => Cell::T,
            3 => Cell::S,
            4 => Cell::Z,
            5 => Cell::J,
            6 => Cell::L,
            _ => Cell::EMPTY,
        }
    }

    pub fn bag_of_cells() -> Vec<Cell> {
        vec![
            Cell::I,
            Cell::O,
            Cell::T,
            Cell::S,
            Cell::Z,
            Cell::J,
            Cell::L,
        ]
    }

    /// Get the cell represented in a 4x4 grid in a 1D array
    #[rustfmt::skip]
    pub fn get_cells(&self) -> Vec<Cell> {
        match self {
            Cell::O => vec![Cell::O, Cell::O,
                            Cell::O, Cell::O],
            Cell::I => vec![Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY,
                            Cell::I,     Cell::I,     Cell::I,     Cell::I,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
            Cell::T => vec![Cell::EMPTY, Cell::T,     Cell::EMPTY,
                            Cell::T,     Cell::T,     Cell::T,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
            Cell::S => vec![Cell::EMPTY, Cell::S,     Cell::S,
                            Cell::S,     Cell::S,     Cell::EMPTY,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
            Cell::Z => vec![Cell::Z,     Cell::Z,     Cell::EMPTY,
                            Cell::EMPTY, Cell::Z,     Cell::Z,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
            Cell::L => vec![Cell::EMPTY, Cell::EMPTY, Cell::L,
                            Cell::L,     Cell::L,     Cell::L,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
            Cell::J => vec![Cell::J,     Cell::EMPTY, Cell::EMPTY,
                            Cell::J,     Cell::J,     Cell::J,
                            Cell::EMPTY, Cell::EMPTY, Cell::EMPTY],
            _ => vec![Cell::EMPTY]
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum RandomizerType {
    /// Shuffles all 7 unique pieces like a deck of cards.
    /// Hands them out and then shuffles a new batch.
    BagOf7 = "BAG_OF_7",
    /// Roll out a random piece, check against memory history
    /// of last pieces dealt. re-rolls if it finds a match.
    /// Last 4 pieces can't be duplicated unless finding a
    /// pieces fails 4 times.
    HistoryBased = "HISTORY_BASED",
    /// Every time a piece spawns, game selects an index from
    /// 0 to 6 with zero regard for what came before
    Memoryless = "MEMORYLESS",
    /// Massive pool containing 5 copied of each of the 7 pieces
    PoolShuffling = "POOL_SHUFFLING",
}

pub struct PieceQueue {
    ty: RandomizerType,
    history: Vec<Cell>,
    queue: VecDeque<Cell>,
}

impl PieceQueue {
    pub fn new(ty: RandomizerType) -> Self {
        let mut this = Self {
            ty,
            history: Vec::new(),
            queue: VecDeque::new(),
        };
        this.requeue();
        this
    }

    pub fn shift(&mut self) -> Cell {
        let cell = self.queue.pop_front().unwrap();
        self.history.push(cell);
        if self.queue.len() < 7 {
            self.requeue()
        }
        cell
    }

    fn requeue(&mut self) {
        match self.ty {
            RandomizerType::BagOf7 => self.queue.extend(Self::shuffle(Cell::bag_of_cells())),
            RandomizerType::HistoryBased => {
                if self.history.len() < 7 {
                    self.queue.extend(Self::shuffle(Cell::bag_of_cells()));
                } else {
                    let mut selected_piece = None;
                    let last_pieces = &self.history[self.history.len() - 5..self.history.len() - 1];
                    for _ in 0..4 {
                        let cell = Cell::random();
                        if !last_pieces.contains(&cell) {
                            selected_piece = Some(cell);
                            break;
                        }
                    }
                    if let Some(selected) = selected_piece {
                        self.queue.push_back(selected);
                    } else {
                        self.queue.push_back(Cell::random());
                    }
                }
            }
            RandomizerType::PoolShuffling => self
                .queue
                .extend(Self::shuffle(Cell::bag_of_cells().repeat(5))),
            _ => {
                for _ in 0..7 {
                    self.queue.push_back(Cell::random())
                }
            }
        }
        if matches!(
            self.ty,
            RandomizerType::BagOf7 | RandomizerType::HistoryBased | RandomizerType::PoolShuffling
        ) {
            if self.history.len() == 0 {
                for _ in 0..7 {
                    let front = self.queue.front().unwrap();
                    if [Cell::S, Cell::Z, Cell::O].contains(front) {
                        let first_cell = self.queue.pop_front().unwrap();
                        self.queue.push_back(first_cell);
                    }
                }
            }
        }
    }

    pub fn as_ptr(&mut self) -> *const Cell {
        self.queue.make_contiguous().as_ptr()
    }

    /// Random generator for next piece position
    /// Read More: https://tetris.fandom.com/wiki/Random_Generator
    #[cfg(target_arch = "wasm32")]
    fn shuffle<T>(mut array: Vec<T>) -> Vec<T> {
        for i in (0..array.len()).rev() {
            let mut j = (js_sys::Math::random() * ((i as f64) + 1.0)).round() as usize;
            if j >= array.len() {
                j = array.len() - 1;
            }
            array.swap(i, j);
        }
        array
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn shuffle<T>(mut array: Vec<T>) -> Vec<T> {
        // for i in (0..array.len()).rev() {
        //     let mut j = (rand::random_range(0.0..1.0) as f64 * ((i as f64) + 1.0)).round() as usize;
        //     if j >= array.len() {
        //         j = array.len() - 1;
        //     }
        //     array.swap(i, j);
        // }
        array
    }
}

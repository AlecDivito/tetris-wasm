use std::convert::TryFrom;

use super::cell::Cell;
use super::game_timer::Timer;
use super::point::Point;
use super::rotation::{Direction, Rotation};

/// Piece tracks where the falling piece is relative to the game board.
/// Piece depends on tetris for information on when to do certain events
pub struct Piece {
    timer: Timer,
    record_timer: f64,
    reset_timer: bool,
    cells: Vec<Cell>,
    rotation: Rotation,
    cell: Cell,
    position: Point,
}

impl Piece {
    /// Create a piece that is a random value
    pub fn random() -> Piece {
        let cell = Cell::random();
        let cells = cell.get_cells();
        let rotation = Rotation::NORTH;
        let position = match cell {
            Cell::O => Point { x: 4, y: 3 },
            Cell::I => Point { x: 3, y: 3 },
            _ => Point { x: 3, y: 3 },
        };

        Piece {
            timer: Timer::new(),
            record_timer: 0.0,
            reset_timer: false,
            cells,
            rotation,
            cell,
            position,
        }
    }

    /// Create a new piece using a cell
    pub fn new(cell: Cell) -> Piece {
        let cells = cell.get_cells();
        let rotation = Rotation::NORTH;
        let position = match cell {
            Cell::O => Point { x: 4, y: 2 },
            Cell::I => Point { x: 3, y: 2 },
            _ => Point { x: 3, y: 2 },
        };

        Piece {
            timer: Timer::new(),
            record_timer: 0.0,
            reset_timer: false,
            cells,
            rotation,
            cell,
            position,
        }
    }

    /// Update the moving piece
    pub fn update(&mut self, elapsed_time: f64) {
        if self.reset_timer {
            self.record_timer = 0.0;
            self.reset_timer = false;
        }
        self.record_timer = self.record_timer + self.timer.get_elapsed_time(elapsed_time);
    }

    pub fn get_record_timer(&self) -> f64 {
        self.record_timer
    }

    pub fn reset_timer(&mut self) {
        self.reset_timer = true;
    }

    pub fn get_index(&self, row: i32, col: i32) -> usize {
        (self.get_bounding_box_size() * row + col) as usize
    }

    #[rustfmt::skip]
    pub fn get_rotation_wall_kick_offsets(&self, next_direction: Direction) -> [(i32, i32); 5] {
        match self.cell {
            Cell::T | Cell::S | Cell::Z | Cell::J | Cell::L => {
                match (self.rotation, next_direction) {
                    (Rotation::NORTH, Direction::Right) => [(0, 0), (-1, 0), (-1, 1),  (0, -2), (-1, -2)],
                    (Rotation::NORTH, Direction::Left) =>  [(0, 0), (1, 0),  (1, -1),  (0, 2),  (1, 2)],
                    (Rotation::EAST, Direction::Right) =>  [(0, 0), (1, 0),  (1, -1),  (0, 2),  (1, 2)],
                    (Rotation::EAST, Direction::Left) =>   [(0, 0), (-1, 0), (-1, 1),  (0, -2), (-1, -2)],
                    (Rotation::SOUTH, Direction::Right) => [(0, 0), (1, 0),  (1, 1),   (0, -2), (1, -2)],
                    (Rotation::SOUTH, Direction::Left) =>  [(0, 0), (-1, 0), (-1, -1), (0, 2),  (-1, 2)],
                    (Rotation::WEST, Direction::Right) =>  [(0, 0), (-1, 0), (-1, -1), (0, 2),  (-1, 2)],
                    (Rotation::WEST, Direction::Left) =>   [(0, 0), (1, 0),  (1, 1),   (0, -2), (1, -2)],
                }
            }
            Cell::I => match (self.rotation, next_direction) {
                (Rotation::NORTH, Direction::Right) => [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (Rotation::NORTH, Direction::Left) =>  [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (Rotation::EAST, Direction::Right) =>  [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                (Rotation::EAST, Direction::Left) =>   [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (Rotation::SOUTH, Direction::Right) => [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (Rotation::SOUTH, Direction::Left) =>  [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (Rotation::WEST, Direction::Right) =>  [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (Rotation::WEST, Direction::Left) =>   [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
            },
            Cell::O =>     [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
            Cell::EMPTY => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.rotate(Direction::Left);
    }

    /**
     * rotations and stuff
     * https://www.youtube.com/watch?v=Atlr5vvdchY
     */
    pub fn rotate_clockwise(&mut self) {
        self.rotate(Direction::Right);
    }

    fn rotate(&mut self, direction: Direction) {
        if self.record_timer > 0.5 {
            self.reset_timer = true;
        } else {
            return;
        }

        let box_size = self.get_bounding_box_size();
        if box_size == 2 {
            return;
        }

        println!("{:?} {:?}", self.rotation, direction);

        if self.cell == Cell::I {
            self.rotate_i(direction);
            return;
        }

        // all other blocks
        let pivot = self.get_piece_origin();
        let mut moves: Vec<(i32, i32)> = Vec::with_capacity(4);
        for row in 0..box_size {
            for col in 0..box_size {
                let index = self.get_index(row, col);
                if self.cells[index] == Cell::EMPTY {
                    continue;
                }

                let rotated_vector_x = col - pivot.x;
                let rotated_vector_y = row - pivot.y;

                let rotation_matrix = match direction {
                    Direction::Right => (1, -1),
                    Direction::Left => (-1, 1),
                };

                let transformed_vector_x = rotation_matrix.1 * rotated_vector_x;
                let transformed_vector_y = rotation_matrix.0 * rotated_vector_y;

                let new_local_x = pivot.x + transformed_vector_x;
                let new_local_y = pivot.y + transformed_vector_y;

                moves.push((new_local_x, new_local_y));
            }
        }

        for i in 0..self.get_cells().len() {
            self.set_cell(i, Cell::EMPTY);
        }

        for (x, y) in &moves {
            let index = self.get_index(*x, *y);
            self.cells[index] = self.cell;
        }
        self.rotation.rotate(direction);
    }

    pub fn get_t_spin_indicies(&self) -> [(i32, i32); 2] {
        match self.rotation {
            Rotation::NORTH => [(0, 0), (0, 2)],
            Rotation::EAST => [(2, 0), (2, 2)],
            Rotation::SOUTH => [(0, 2), (2, 2)],
            Rotation::WEST => [(0, 0), (0, 2)],
        }
    }

    fn rotate_i(&mut self, direction: Direction) {
        let mut moves: Vec<(i32, i32)> = Vec::with_capacity(4);

        let px = 3;
        let py = 3;

        for row in 0..4 {
            for col in 0..4 {
                let index = self.get_index(row, col);
                if self.cells[index] == Cell::EMPTY {
                    continue;
                }

                // Convert to doubled coordinates
                let dx = col * 2 - px;
                let dy = row * 2 - py;

                // Rotate about (1.5, 1.5)
                let (rdx, rdy) = match direction {
                    Direction::Right => (dy, -dx),
                    Direction::Left => (-dy, dx),
                };

                // Convert back to cell coordinates
                let new_col = (px + rdx) / 2;
                let new_row = (py + rdy) / 2;

                moves.push((new_col, new_row));
            }
        }

        // Clear old cells
        for cell in &mut self.cells {
            *cell = Cell::EMPTY;
        }

        // Place rotated cells
        for (col, row) in moves {
            let index = self.get_index(row, col);
            self.cells[index] = Cell::I;
        }

        self.rotation.rotate(direction);
    }

    pub fn advance(&mut self) {
        self.position.y = self.position.y + 1;
    }

    pub fn move_piece(&mut self, direction: Direction) {
        if self.record_timer > 50.0 {
            self.reset_timer = true;
            match direction {
                Direction::Left => self.position.x -= 1,
                Direction::Right => self.position.x += 1,
            };
        }
    }

    pub fn force_move_piece(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        };
    }

    fn get_piece_origin(&self) -> Point {
        match self.cell {
            Cell::I => match self.rotation {
                Rotation::NORTH => Point { x: 3, y: 3 },
                Rotation::WEST => Point { x: 3, y: 3 },
                Rotation::SOUTH => Point { x: 3, y: 3 },
                Rotation::EAST => Point { x: 3, y: 3 },
            },
            _ => Point { x: 1, y: 1 },
        }
    }

    pub fn get_bounding_box_size(&self) -> i32 {
        match self.cell {
            Cell::I => 4,
            Cell::O => 2,
            _ => 3,
        }
    }

    pub fn get_piece(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_type(&self) -> Cell {
        self.cell
    }

    pub(crate) fn get_position_ref(&mut self) -> &mut Point {
        &mut self.position
    }

    pub fn get_position(&self) -> Point {
        self.position.clone()
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn get_cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell(&self, row: i32, col: i32) -> Cell {
        self.cells[self.get_index(row, col)]
    }

    pub fn set_cell(&mut self, index: usize, cell: Cell) {
        self.cells[index] = cell;
    }

    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    pub fn get_timer(&self) -> f64 {
        self.record_timer
    }

    pub fn set_timer(&mut self, timer: f64) {
        self.record_timer = timer
    }
}

impl TryFrom<&str> for Piece {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cells = Vec::new();
        for line in value.trim().lines() {
            for char in line.trim().chars() {
                cells.push(Cell::from(char));
            }
        }

        let (_, real_cells): (Vec<_>, Vec<_>) = cells.iter().partition(|e| **e == Cell::EMPTY);
        if real_cells.is_empty() {
            return Err(());
        }
        if !real_cells.iter().all(|c| *c == real_cells[0]) {
            return Err(());
        }

        Ok(Piece {
            timer: Timer::new(),
            record_timer: 0.0,
            reset_timer: false,
            cells,
            rotation: Rotation::NORTH,
            cell: real_cells[0],
            position: Point { x: 0, y: 0 },
        })
    }
}

#[cfg(test)]
mod test {

    use std::convert::TryFrom;

    use crate::tetris::cell::Cell;
    use crate::tetris::piece::Piece;

    #[test]
    pub fn cell_i_rotation() {
        let t1 = r#"
        _I__
        _I__
        _I__
        _I__
        "#;

        let t2 = r#"
        ____
        ____
        IIII
        ____
        "#;

        let t3 = r#"
        __I_
        __I_
        __I_
        __I_
        "#;

        let t4 = r#"
        ____
        IIII
        ____
        ____
        "#;

        let mut piece = Piece::new(Cell::I);
        let tests = [t1, t2, t3, t4];
        for i in 0..tests.len() {
            let t = tests[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }

        let tests_reversed = [t3, t2, t1, t4];
        for i in 0..tests_reversed.len() {
            let t = tests_reversed[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_counter_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }
    }

    #[test]
    pub fn cell_t_rotation() {
        let t1 = r#"
        _T_
        TT_
        _T_
        "#;

        let t2 = r#"
        ___
        TTT
        _T_
        "#;

        let t3 = r#"
        _T_
        _TT
        _T_
        "#;

        let t4 = r#"
        _T_
        TTT
        ___
        "#;

        let mut piece = Piece::new(Cell::T);
        let tests = [t1, t2, t3, t4];
        for i in 0..tests.len() {
            let t = tests[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }

        let tests_reversed = [t3, t2, t1, t4];
        for i in 0..tests_reversed.len() {
            let t = tests_reversed[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_counter_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }
    }

    #[test]
    pub fn cell_s_rotation() {
        let t1 = r#"
        S__
        SS_
        _S_
        "#;

        let t2 = r#"
        ___
        _SS
        SS_
        "#;

        let t3 = r#"
        _S_
        _SS
        __S
        "#;

        let t4 = r#"
        _SS
        SS_
        ___
        "#;

        let mut piece = Piece::new(Cell::S);
        let tests = [t1, t2, t3, t4];
        for i in 0..tests.len() {
            let t = tests[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }

        let tests_reversed = [t3, t2, t1, t4];
        for i in 0..tests_reversed.len() {
            let t = tests_reversed[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_counter_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }
    }

    #[test]
    pub fn cell_z_rotation() {
        let t1 = r#"
        _Z_
        ZZ_
        Z__
        "#;

        let t2 = r#"
        ___
        ZZ_
        _ZZ
        "#;

        let t3 = r#"
        __Z
        _ZZ
        _Z_
        "#;

        let t4 = r#"
        ZZ_
        _ZZ
        ___
        "#;

        let mut piece = Piece::new(Cell::Z);
        let tests = [t1, t2, t3, t4];
        for i in 0..tests.len() {
            let t = tests[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }

        let tests_reversed = [t3, t2, t1, t4];
        for i in 0..tests_reversed.len() {
            let t = tests_reversed[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_counter_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }
    }

    #[test]
    pub fn cell_l_rotation() {
        let t1 = r#"
        LL_
        _L_
        _L_
        "#;

        let t2 = r#"
        ___
        LLL
        L__
        "#;

        let t3 = r#"
        _L_
        _L_
        _LL
        "#;

        let t4 = r#"
        __L
        LLL
        ___
        "#;

        let mut piece = Piece::new(Cell::L);
        let tests = [t1, t2, t3, t4];
        for i in 0..tests.len() {
            let t = tests[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }

        let tests_reversed = [t3, t2, t1, t4];
        for i in 0..tests_reversed.len() {
            let t = tests_reversed[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_counter_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }
    }

    #[test]
    pub fn cell_j_rotation() {
        let t1 = r#"
        _J_
        _J_
        JJ_
        "#;

        let t2 = r#"
        ___
        JJJ
        __J
        "#;

        let t3 = r#"
        _JJ
        _J_
        _J_
        "#;

        let t4 = r#"
        J__
        JJJ
        ___
        "#;

        let mut piece = Piece::new(Cell::J);
        let tests = [t1, t2, t3, t4];
        for i in 0..tests.len() {
            let t = tests[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }

        let tests_reversed = [t3, t2, t1, t4];
        for i in 0..tests_reversed.len() {
            let t = tests_reversed[i];
            let test_piece = Piece::try_from(t).unwrap();
            piece.set_timer(1000.0);
            piece.rotate_counter_clockwise();
            assert_eq!(test_piece.cells, piece.cells, "Failed on test {}", i);
        }
    }
}

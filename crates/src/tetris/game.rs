use wasm_bindgen::prelude::wasm_bindgen;

use crate::tetris::config::Config;
use crate::tetris::locks::LockDelay;
use crate::tetris::scoring::{ScoreB2BType, TSpin};

use super::action::Action;
use super::cell::Cell;
use super::game_timer::GameTimer;
use super::piece::Piece;
use super::point::Point;
use super::rotation::Direction;

#[cfg(target_arch = "wasm32")]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! log {
    ($($t:tt)*) => {
        println!($($t)*);
    };
}

#[wasm_bindgen]
pub struct Game {
    game: GameTimer,
    soft_drop: bool,
    can_hard_drop: bool,
    rows_completed: u32,
    level: u32,
    score: u32,
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    piece_queue: Vec<Cell>,
    piece: Piece,
    shadow_piece_position: Point,
    can_swap_piece: bool,
    hold_piece: Cell,
    game_over: bool,
    combo_count: u32,
    config: Config,
    actions: Vec<Action>,
    score_b2b_type: ScoreB2BType,
    lock_delay: LockDelay,
}

/// try to remove the first element of the array
pub fn shift<T>(array: &mut Vec<T>) -> Option<T> {
    array.reverse();
    let value = array.pop();
    array.reverse();
    value
}

#[wasm_bindgen]
impl Game {
    /// Create a new tetris game
    pub fn new() -> Self {
        Self::from_config(Config::default())
    }

    pub fn from_config(config: Config) -> Self {
        let game = GameTimer::new();
        let score = config.current_level;
        let rows_completed = config.current_rows_completed;
        let level = config.current_level;
        let width = config.width as i32;
        let height = config.height as i32;
        let soft_drop = false;
        let game_over = false;

        let cells = (0..width * height).map(|_i| Cell::EMPTY).collect();

        let mut piece_queue = Cell::random_piece_queue();
        piece_queue.append(&mut Cell::random_piece_queue());

        let mut piece = match shift(&mut piece_queue) {
            Some(x) => Piece::new(x),
            None => Piece::random(),
        };
        piece.advance();
        let shadow_piece_position = Point::new(0, 0);
        let hold_piece = Cell::EMPTY;
        let can_swap_piece = true;
        let can_hard_drop = true;
        let lock_delay = config.get_lock_delay();

        Game {
            game,
            soft_drop,
            can_hard_drop,
            rows_completed,
            level,
            score,
            width,
            height,
            cells,
            piece,
            piece_queue,
            shadow_piece_position,
            can_swap_piece,
            hold_piece,
            game_over,
            combo_count: 0,
            config,
            actions: vec![],
            score_b2b_type: ScoreB2BType::None,
            lock_delay,
        }
    }

    /// take in an array of byte events to apply to the game.
    pub fn byte_event_handler(&mut self, events: &mut [u8]) {
        events.sort();
        let actions = events.iter().map(|e| Action::from(*e)).collect::<Vec<_>>();
        self.event_handler(&actions)
    }

    pub fn touch_event_handler(&mut self, target_x_pos: i32, _target_y_pos: i32) {
        if target_x_pos > self.piece.get_position().x {
            while self.can_piece_go_right() && target_x_pos > self.get_piece_position().x {
                self.piece.force_move_piece(Direction::Right);
            }
        } else if target_x_pos < self.piece.get_position().x {
            while self.can_piece_go_left() && target_x_pos < self.get_piece_position().x {
                self.piece.force_move_piece(Direction::Left);
            }
        }
    }

    /// Update the tetris board
    pub fn update(&mut self, elapsed_time: f64) -> bool {
        if self.game.is_paused() || self.game_over {
            return false;
        }

        self.game.update(elapsed_time);
        self.piece.update(elapsed_time);
        if !self.can_piece_advance() {
            self.lock_delay.update(elapsed_time);
        }

        let mut result = false;
        if self.game.can_update_game(self.update_speed()) {
            self.game.reset();
            if self.advance_if_possible() {
                self.lock_delay.handle_advance();
                if self.soft_drop {
                    self.score += 1;
                }
                result = false;
            } else {
                // Handle lock delays
                if !self.lock_delay.is_locked() {
                    result = false
                } else {
                    self.merge_piece_into_board();
                    self.can_swap_piece = true;
                    if self.is_topped_out() {
                        self.game_over = true;
                    } else {
                        self.update_board();
                        self.get_next_piece();
                        self.piece.advance();
                        if self.soft_drop {
                            self.score += 1
                        }
                    }
                    result = true;
                }
            }
            self.soft_drop = false;
        }
        self.update_shadow_piece_position();
        result
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    /// Get the current score of the running game
    pub fn get_score(&self) -> u32 {
        self.score
    }

    /// Get the width of the game board
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Get the height of the game board
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Get the offset height to make the game field
    pub fn get_offset_height(&self) -> i32 {
        5
    }

    /// Return the current level of the game board
    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Return the number of rows that have been completed
    pub fn get_rows_completed(&self) -> u32 {
        self.rows_completed
    }

    /// Get the current piece that is currently being held
    pub fn get_hold_piece(&self) -> Cell {
        self.hold_piece
    }

    /// Get the cells that are in queue to go next
    pub fn get_queued_pieces(&self) -> *const Cell {
        self.piece_queue.as_ptr()
    }

    /// Return a pointer to the first element in the boards vector
    pub fn get_cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Get the cells that make up the falling piece
    pub fn get_pieces(&self) -> *const Cell {
        self.piece.get_piece()
    }

    /// Get the pieces bounding box size
    pub fn get_piece_bounding_box(&self) -> i32 {
        self.piece.get_bounding_box_size()
    }

    /// Get the actives pieces type
    pub fn get_piece_type(&self) -> Cell {
        self.piece.get_type()
    }

    /// Get the world coordinates of the active piece position
    pub fn get_piece_position(&self) -> Point {
        self.piece.get_position()
    }

    /// Get the position of the shadow piece
    pub fn get_shadow_piece_position(&self) -> Point {
        self.shadow_piece_position.clone()
    }
}

impl Game {
    /// Return the next level's goal
    ///
    /// The next levels goal is always the current level * 5
    fn get_next_level_goal(&self) -> u32 {
        self.level * self.config.next_goal_multiplier
    }

    /// Update the game state based on the input events provided
    pub fn event_handler(&mut self, events: &[Action]) {
        self.actions = events.to_vec();
        self.actions.sort();
        if !events.contains(&Action::HardDrop) {
            self.can_hard_drop = true;
        }

        for action in self.actions.clone() {
            let stop_updating = match action {
                Action::HardDrop => self.hard_drop(),
                Action::HoldPiece => self.hold_piece(),
                Action::RotateClockWise => self.rotate(Direction::Right),
                Action::RotateCounterClockWise => self.rotate(Direction::Left),
                Action::MoveLeft => self.move_piece(Direction::Left),
                Action::MoveRight => self.move_piece(Direction::Right),
                Action::SoftDrop => self.enable_soft_drop(),
                Action::ToggleRunning => self.game.toggle_pause(),
                _ => false,
            };
            if stop_updating {
                break;
            }
        }
    }

    /// Convert row and column to a position inside of the
    /// board array
    pub(crate) fn get_index(&self, row: i32, col: i32) -> usize {
        ((self.width * row) + col) as usize
    }

    /// Merge piece into board of cells
    pub(crate) fn merge_piece_into_board(&mut self) {
        for row in 0..self.piece.get_bounding_box_size() {
            for col in 0..self.piece.get_bounding_box_size() {
                let cell = self.piece.get_cells()[self.piece.get_index(row, col)];
                if cell == Cell::EMPTY {
                    continue;
                }
                let world_coord = Point {
                    x: self.piece.get_position().x + col,
                    y: self.piece.get_position().y + row,
                };

                let index = self.get_index(world_coord.y, world_coord.x);
                self.cells[index] = cell;
            }
        }
    }

    fn advance_if_possible(&mut self) -> bool {
        if self.can_piece_advance() {
            self.piece.advance();
            true
        } else {
            false
        }
    }

    /// Check if piece can advance on the board
    ///
    /// make sure that all blocks under a cell is empty,
    /// otherwise return false to stop advancement
    fn can_piece_advance(&self) -> bool {
        let mut can_piece_still_advance = vec![];
        // 1. find the lowest point on the shape
        for row in 0..self.piece.get_bounding_box_size() {
            for col in 0..self.piece.get_bounding_box_size() {
                let local_index = self.piece.get_index(row, col);
                if self.piece.get_cells()[local_index] != Cell::EMPTY {
                    // 2. convert the local lowest row into world coordinates
                    let world_coord = Point {
                        x: self.piece.get_position().x + col,
                        y: self.piece.get_position().y + row,
                    };

                    // 3. check if piece will pass the border if it goes down 1 more
                    if world_coord.y + 1 >= self.height {
                        can_piece_still_advance.push(false);
                        break;
                    }

                    // 4. now that the row is in world coordinates, check if there is a piece
                    //    under this one in the game cells
                    let world_index = self.get_index(world_coord.y + 1, world_coord.x);
                    if self.cells[world_index] != Cell::EMPTY {
                        can_piece_still_advance.push(false);
                        break;
                    }
                    can_piece_still_advance.push(true)
                }
            }
        }

        return can_piece_still_advance.iter().any(|f| *f == false);
    }

    /// Check if a piece can move right
    fn can_piece_go_right(&self) -> bool {
        // 1. find the rightest point on the shape
        for row in 0..self.piece.get_bounding_box_size() {
            for col in 0..self.piece.get_bounding_box_size() {
                let local_index = self.piece.get_index(row, col);
                if self.piece.get_cells()[local_index] != Cell::EMPTY {
                    // 2. convert the local lowest row into world coordinates
                    let world_coord = Point {
                        x: self.piece.get_position().x + col,
                        y: self.piece.get_position().y + row,
                    };

                    // 3. now that the row is in world coordinates, check if there is a piece
                    //    to the right of the game cell
                    let world_index = self.get_index(world_coord.y, world_coord.x + 1);
                    if self.cells[world_index] != Cell::EMPTY {
                        return false;
                    }

                    // 4. check if piece will pass the border if it goes down 1 more
                    if world_coord.x + 1 >= self.width {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    /// Check if a piece can move left
    fn can_piece_go_left(&self) -> bool {
        // 1. find the rightest point on the shape
        let mut allowed = vec![];
        for row in 0..self.piece.get_bounding_box_size() {
            for col in 0..self.piece.get_bounding_box_size() {
                let local_index = self.piece.get_index(row, col);
                if self.piece.get_cells()[local_index] == Cell::EMPTY {
                    continue;
                }
                // 2. convert the local lowest row into world coordinates
                let world_coord = Point {
                    x: self.piece.get_position().x + col,
                    y: self.piece.get_position().y + row,
                };

                // 3. check if piece will pass the border if it goes down 1 more
                if world_coord.x - 1 < 0 {
                    allowed.push(false);
                }

                // 4. now that the row is in world coordinates, check if there is a piece
                //    to the right of the game cell
                let world_index = self.get_index(world_coord.y, world_coord.x - 1);
                if self.cells[world_index] != Cell::EMPTY {
                    allowed.push(false);
                }
            }
        }
        return allowed.iter().all(|v| *v == true);
    }

    fn get_t_spin(&mut self) -> TSpin {
        if self.piece.get_type() != Cell::T {
            return TSpin::None;
        }
        if !self.actions.contains(&Action::RotateClockWise)
            && !self.actions.contains(&Action::RotateCounterClockWise)
        {
            return TSpin::None;
        }

        let mut indices = vec![];
        for row in [0, 2] {
            for col in [0, 2] {
                let world_x = self.piece.get_position_ref().x + col;
                let world_y = self.piece.get_position_ref().y + row;
                let world_index = self.get_index(world_x, world_y);
                if self.cells[world_index] != Cell::EMPTY
                    && world_x > 0
                    && world_x < self.width
                    && world_y > 0
                    && world_y < self.height
                {
                    indices.push((row, col));
                    continue;
                }
            }
        }

        let t_spin_indices = self.piece.get_t_spin_indicies();
        if indices.len() < 3 {
            TSpin::None
        } else if t_spin_indices.iter().all(|f| indices.contains(f)) {
            TSpin::Full
        } else {
            TSpin::Mini
        }
    }

    fn update_board(&mut self) {
        // 1. find all removable rows
        let mut removable_rows = Vec::new();
        for row in (0..self.height).rev() {
            if self.is_row_full(row) {
                removable_rows.push(row);
            }
        }

        // 2. if no rows are removable, return
        if removable_rows.len() == 0 {
            self.combo_count = 0;
            self.score_b2b_type = ScoreB2BType::None;
            return;
        }

        let t_spin = self.get_t_spin();

        // 2. calculate points
        let mut b2b_score_type = ScoreB2BType::None;
        let base_score = match (removable_rows.len(), t_spin) {
            // Single or (Mini T-Spin)
            (0, TSpin::None) => 0,
            (1, TSpin::None) => 100,
            (2, TSpin::None) => 300,
            (3, TSpin::None) => 500,
            (4, TSpin::None) => {
                b2b_score_type = ScoreB2BType::Tetris;
                800
            }

            (0, TSpin::Mini) => 100,
            (1, TSpin::Mini) => 200,
            (2, TSpin::Mini) => {
                b2b_score_type = ScoreB2BType::MiniTSpinDouble;
                400
            }

            (0, TSpin::Full) => 400,
            (1, TSpin::Full) => {
                b2b_score_type = ScoreB2BType::TSpinSingle;
                800
            }
            (2, TSpin::Full) => {
                b2b_score_type = ScoreB2BType::TSpinDouble;
                1200
            }
            (3, TSpin::Full) => {
                b2b_score_type = ScoreB2BType::TSpinTriple;
                1600
            }

            _ => 0,
        };
        let multiplier = if self.score_b2b_type != ScoreB2BType::None {
            1.5
        } else {
            1.0
        };
        self.score += ((base_score as f64 * multiplier) as u32 + self.combo_count) * self.level;
        self.score_b2b_type = b2b_score_type;

        self.combo_count += 50;

        // update level if rows_completed passed a threshold
        self.rows_completed = self.rows_completed + (removable_rows.len() as u32);
        if self.rows_completed > self.get_next_level_goal() {
            self.level = self.level + 1;
        }

        // 3. remove all rows and push higher pieces down
        for row in &removable_rows {
            for col in 0..self.width {
                let index = self.get_index(*row, col);
                self.cells[index] = Cell::EMPTY;
            }
        }

        // 4. pull all piece above row down one
        for _ in &removable_rows {
            for row in (0..self.height - 1).rev() {
                // move bricks down one
                if !self.is_row_empty(row + 1) {
                    continue;
                }
                for col in 0..self.width {
                    let old_index = self.get_index(row, col);
                    let new_index = self.get_index(row + 1, col);
                    self.cells.swap(old_index, new_index);
                }
            }
        }
    }

    pub(crate) fn update_shadow_piece_position(&mut self) {
        let mut world_y = 0;
        for row in self.piece.get_position().y..self.height {
            let mut passes = Vec::new();
            for local_row in 0..self.piece.get_bounding_box_size() {
                for local_col in 0..self.piece.get_bounding_box_size() {
                    // 1. check if the local piece position is empty
                    if self.piece.get_cell(local_row, local_col) == Cell::EMPTY {
                        passes.push(true);
                        continue;
                    }

                    // The cell is an actual value, now we need to paste it onto the board
                    // First check is if it's off the screen
                    if row + local_row >= self.height {
                        passes.push(false);
                        continue;
                    }

                    if self.piece.get_position().x + local_col >= self.width {
                        passes.push(false);
                        continue;
                    }

                    // Check if the piece intersects with another piece
                    let world_index =
                        self.get_index(row + local_row, self.piece.get_position().x + local_col);
                    if self.cells[world_index] == Cell::EMPTY {
                        passes.push(true);
                    } else {
                        passes.push(false);
                    }
                }
            }

            if passes.iter().all(|x| *x == true) {
                world_y = row;
            } else {
                break;
            }
        }
        self.shadow_piece_position = Point {
            x: self.piece.get_position().x,
            y: world_y,
        };
    }

    fn is_row_full(&self, row: i32) -> bool {
        for col in 0..self.width {
            let index = self.get_index(row, col);
            if self.cells[index] == Cell::EMPTY {
                return false;
            }
        }
        return true;
    }

    fn is_row_empty(&self, row: i32) -> bool {
        for col in 0..self.width {
            let index = self.get_index(row, col);
            if self.cells[index] != Cell::EMPTY {
                return false;
            }
        }
        return true;
    }

    fn rotate(&mut self, direction: Direction) -> bool {
        if self.piece.get_record_timer() > self.config.piece_rotation_wait_time {
            self.piece.reset_timer();
        } else {
            return false;
        }
        if self.piece.get_type() == Cell::O {
            return false;
        }

        let mut origin_move = None;
        let srs_rotations = self.piece.get_rotation_wall_kick_offsets(direction);
        let timer = self.piece.get_timer();
        match direction {
            Direction::Right => self.piece.rotate_clockwise(),
            Direction::Left => self.piece.rotate_counter_clockwise(),
        }

        // 1. Check if the piece fits into it's new rotation
        for (x, y) in srs_rotations {
            let mut passes = Vec::new();
            for local_row in 0..self.piece.get_bounding_box_size() {
                for local_col in 0..self.piece.get_bounding_box_size() {
                    if self.piece.get_cell(local_row, local_col) == Cell::EMPTY {
                        passes.push(true);
                        continue;
                    }

                    let world_row = self.piece.get_position().x + local_col + (-1 * x);
                    if world_row >= self.width || world_row < 0 {
                        passes.push(false);
                        continue;
                    }

                    let world_col = self.piece.get_position().y + local_row + (-1 * y);
                    if world_col >= self.height || world_col < 0 {
                        passes.push(false);
                        continue;
                    }

                    let world_index = self.get_index(world_col, world_row);
                    if self.cells[world_index] == Cell::EMPTY {
                        passes.push(true);
                    } else {
                        passes.push(false);
                    }
                }
            }
            if passes.iter().all(|v| *v == true) {
                origin_move = Some((x, y));
                break;
            }
        }

        if let Some((x, y)) = origin_move {
            self.lock_delay.handle_move();
            self.piece.get_position_ref().x += -1 * x;
            self.piece.get_position_ref().y += -1 * y;
        } else {
            self.piece.set_timer(timer);
            match direction {
                Direction::Right => self.piece.rotate_counter_clockwise(),
                Direction::Left => self.piece.rotate_clockwise(),
            }
            self.piece.set_timer(timer);
        }
        return false;
    }

    fn move_piece(&mut self, direction: Direction) -> bool {
        let can_move = match direction {
            Direction::Left => self.can_piece_go_left(),
            Direction::Right => self.can_piece_go_right(),
        };
        if can_move {
            self.lock_delay.handle_move();
            self.piece.move_piece(direction);
        }
        false
    }

    fn hard_drop(&mut self) -> bool {
        if self.can_hard_drop {
            self.can_hard_drop = false;
            self.score += ((self.height - self.piece.get_position().y) * 2) as u32;
            self.piece.set_position(self.shadow_piece_position);
            self.game.update_asap();
            return true;
        } else {
            return false;
        }
    }

    fn hold_piece(&mut self) -> bool {
        if !self.can_swap_piece {
            return true;
        }

        if self.hold_piece == Cell::EMPTY {
            self.hold_piece = self.piece.get_type();
            self.get_next_piece();
        } else {
            let new_piece = self.hold_piece;
            self.hold_piece = self.piece.get_type();
            self.piece = Piece::new(new_piece);
        }
        self.can_swap_piece = false;
        true
    }

    fn get_next_piece(&mut self) {
        self.piece = match shift(&mut self.piece_queue) {
            Some(x) => Piece::new(x),
            None => Piece::random(),
        };
        if self.piece_queue.len() <= self.config.max_piece_queue_size {
            self.piece_queue.append(&mut Cell::random_piece_queue());
        }
    }

    fn is_topped_out(&self) -> bool {
        for col in 0..self.width {
            let index = self.get_index(4, col);
            if self.cells[index] != Cell::EMPTY {
                return true;
            }
        }
        false
    }

    fn update_speed(&self) -> f64 {
        let update_speed = if self.soft_drop {
            self.config.soft_drop_speed_multiplier
        } else {
            let level = (self.level as f64) - 1.0;
            (0.8 - (level * self.config.speed_multiplier)).powf(level)
        };
        update_speed * 1000.0
    }

    fn enable_soft_drop(&mut self) -> bool {
        self.soft_drop = true;
        false
    }

    pub fn override_cell(&mut self, index: usize, cell: Cell) {
        self.cells[index] = cell;
    }

    #[cfg(test)]
    pub(crate) fn get_bounding_box_index(&self, row: i32, col: i32) -> usize {
        ((self.piece.get_bounding_box_size() * row) + col) as usize
    }

    #[cfg(test)]
    pub(crate) fn get_cell_vec(&self) -> &[Cell] {
        &self.cells
    }

    #[cfg(test)]
    pub(crate) fn get_piece(&mut self) -> &mut Piece {
        &mut self.piece
    }

    #[cfg(test)]
    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = piece;
    }

    #[cfg(test)]
    pub fn print(&self) {
        let piece_bb = self.piece.get_bounding_box_size();
        let cells = self.get_cell_vec();
        for y in 0..self.height {
            print!("{}{}|", y, if y < 10 { " " } else { "" });
            for x in 0..self.width {
                if self.piece.get_position().x <= x
                    && self.piece.get_position().x + piece_bb > x
                    && self.piece.get_position().y <= y
                    && self.piece.get_position().y + piece_bb > y
                {
                    let local_x = x - self.piece.get_position().x;
                    let local_y = y - self.piece.get_position().y;
                    let cell = self
                        .piece
                        .get_cells()
                        .get(self.get_bounding_box_index(local_y, local_x))
                        .unwrap();
                    if *cell != Cell::EMPTY {
                        print!("{}|", cells[self.get_index(y, x)])
                    } else {
                        print!("{}|", cells[self.get_index(y, x)])
                    }
                } else if self.shadow_piece_position.x <= x
                    && self.shadow_piece_position.x + piece_bb > x
                    && self.shadow_piece_position.y <= y
                    && self.shadow_piece_position.y + piece_bb > y
                {
                    let local_x = x - self.shadow_piece_position.x;
                    let local_y = y - self.shadow_piece_position.y;
                    let cell = self
                        .piece
                        .get_cells()
                        .get(self.get_bounding_box_index(local_y, local_x))
                        .unwrap();
                    if *cell != Cell::EMPTY {
                        print!("S|")
                    } else {
                        print!("{}|", cells[self.get_index(y, x)])
                    }
                } else {
                    print!("{}|", cells[self.get_index(y, x)])
                }
            }
            println!();
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut cells = Vec::new();
        for line in value.trim().lines() {
            for char in line.trim().chars() {
                cells.push(Cell::from(char));
            }
        }

        let mut game = Game::new();

        for i in 0..cells.len() {
            game.override_cell(i, cells[i]);
        }

        game
    }
}

#[cfg(test)]
mod test {

    use crate::tetris::action::Action;
    use crate::tetris::cell::Cell;
    use crate::tetris::game::Game;
    use crate::tetris::piece::Piece;
    use crate::tetris::point::Point;
    use crate::tetris::rotation::Rotation;

    fn action(game: &mut Game, action: Action) {
        game.get_piece().set_timer(1000.0);
        game.event_handler(&mut [action]);
    }

    fn actions(game: &mut Game, actions: &[Action]) {
        let mut vec_actions = actions.to_vec();
        game.get_piece().set_timer(1000.0);
        game.event_handler(&mut vec_actions);
    }

    fn new_piece(cell: Cell, x: i32, y: i32, rotation: Rotation) -> Piece {
        let mut piece = Piece::new(cell);
        piece.set_timer(1000.0);
        piece.set_position(Point::new(x, y));
        match rotation {
            Rotation::NORTH => {}
            Rotation::EAST => piece.rotate_clockwise(),
            Rotation::SOUTH => {
                piece.rotate_clockwise();
                piece.rotate_clockwise()
            }
            Rotation::WEST => piece.rotate_counter_clockwise(),
        }
        piece
    }

    fn render_board(game: &mut Game) {
        game.update_shadow_piece_position();
        game.merge_piece_into_board();
    }

    #[test]
    pub fn test_rotation_left() {
        let mut game = Game::new();
        game.set_piece(new_piece(Cell::I, -2, 5, Rotation::WEST));
        actions(&mut game, &[Action::RotateClockWise, Action::MoveLeft]);
        actions(&mut game, &[Action::RotateClockWise, Action::MoveLeft]);
        actions(&mut game, &[Action::RotateClockWise]);
        render_board(&mut game);

        let cells = game.get_cell_vec();
        let position = game.get_piece_position();
        game.print();

        assert_eq!(position.x, 0);
        assert_eq!(position.y, 5);

        assert_eq!(cells[game.get_index(7, 0)], Cell::I);
        assert_eq!(cells[game.get_index(7, 1)], Cell::I);
        assert_eq!(cells[game.get_index(7, 2)], Cell::I);
        assert_eq!(cells[game.get_index(7, 3)], Cell::I);
    }

    #[test]
    pub fn test_rotation_left_2() {
        let mut game = Game::new();
        let mut piece = Piece::new(Cell::I);
        let moves = [Action::RotateClockWise, Action::MoveLeft];
        piece.set_position(Point::new(0, 5));
        game.set_piece(piece);

        actions(&mut game, &moves);
        actions(&mut game, &moves);
        actions(&mut game, &moves);
        actions(&mut game, &moves);
        actions(&mut game, &moves);
        actions(&mut game, &moves);

        let position = game.get_piece_position();
        assert_eq!(position.x, 0);
        assert_eq!(position.y, 5);
        game.merge_piece_into_board();
        let cells = game.get_cell_vec();
        assert_eq!(cells[game.get_index(7, 0)], Cell::I);
        assert_eq!(cells[game.get_index(7, 1)], Cell::I);
        assert_eq!(cells[game.get_index(7, 2)], Cell::I);
        assert_eq!(cells[game.get_index(7, 3)], Cell::I);
    }

    #[test]
    pub fn test_rotation_right() {
        let mut game = Game::new();
        game.set_piece(new_piece(Cell::I, game.get_width() - 4, 5, Rotation::NORTH));

        // start flat
        action(&mut game, Action::RotateClockWise);
        action(&mut game, Action::RotateClockWise);
        action(&mut game, Action::RotateClockWise);
        action(&mut game, Action::MoveRight);
        action(&mut game, Action::MoveRight);
        let position = game.get_piece_position();
        assert_eq!(position.x, game.get_width() - 3);
        assert_eq!(position.y, 5);
        action(&mut game, Action::RotateClockWise);

        render_board(&mut game);
        game.print();

        let cells = game.get_cell_vec();
        let max = game.get_width();
        let position = game.get_piece_position();
        assert_eq!(position.x, game.get_width() - 4);
        assert_eq!(position.y, 5);

        assert_eq!(cells[game.get_index(6, max - 1)], Cell::I);
        assert_eq!(cells[game.get_index(6, max - 2)], Cell::I);
        assert_eq!(cells[game.get_index(6, max - 3)], Cell::I);
        assert_eq!(cells[game.get_index(6, max - 4)], Cell::I);
    }

    #[test]
    pub fn test_shadow_always_on_floor() {
        let mut game = Game::new();

        let mut piece = Piece::new(Cell::I);
        piece.set_position(Point::new(3, 5));
        game.set_piece(piece);
        game.update_shadow_piece_position();
        game.merge_piece_into_board();

        let mut piece = Piece::new(Cell::J);
        piece.set_position(Point::new(2, 6));
        game.set_piece(piece);
        game.update_shadow_piece_position();
        game.merge_piece_into_board();

        game.print();

        let cells = game.get_cell_vec();
        let shadow_position = game.get_shadow_piece_position();

        assert_eq!(cells[game.get_index(6, 3)], Cell::I);
        assert_eq!(cells[game.get_index(6, 4)], Cell::I);
        assert_eq!(cells[game.get_index(6, 5)], Cell::I);
        assert_eq!(cells[game.get_index(6, 6)], Cell::I);

        assert_eq!(cells[game.get_index(6, 2)], Cell::J);
        assert_eq!(cells[game.get_index(7, 2)], Cell::J);
        assert_eq!(cells[game.get_index(7, 3)], Cell::J);
        assert_eq!(cells[game.get_index(7, 4)], Cell::J);

        assert_eq!(shadow_position.x, 2);
        assert_eq!(shadow_position.y, 23);
    }

    #[test]
    pub fn test_shadow_always_on_floor_2() {
        let mut game = Game::from(
            r#" 
        __________
        ____IIIIII
        "#,
        );

        let mut piece = Piece::new(Cell::L);
        piece.set_timer(1000.0);
        piece.rotate_counter_clockwise();
        piece.set_position(Point::new(2, 0));
        game.set_piece(piece);
        game.update_shadow_piece_position();
        game.merge_piece_into_board();

        game.print();

        let cells = game.get_cell_vec();
        let shadow_position = game.get_shadow_piece_position();

        assert_eq!(cells[game.get_index(0, 3)], Cell::L);
        assert_eq!(cells[game.get_index(1, 3)], Cell::L);
        assert_eq!(cells[game.get_index(2, 3)], Cell::L);
        assert_eq!(cells[game.get_index(2, 4)], Cell::L);

        assert_eq!(shadow_position.x, 2);
        assert_eq!(shadow_position.y, 22);
    }

    #[test]
    pub fn test_l_rotation_1() {
        let mut game = Game::from(
            r#" 
        LLLLL LLLL
        LLL    LLL
        LLLLL LLLL
        LLLLL LLLL
        "#,
        );

        game.set_piece(new_piece(Cell::I, 3, 0, Rotation::NORTH));
        action(&mut game, Action::RotateCounterClockWise);
        render_board(&mut game);
        game.print();

        let cells = game.get_cell_vec();
        assert_eq!(cells[game.get_index(0, 5)], Cell::I);
        assert_eq!(cells[game.get_index(1, 5)], Cell::I);
        assert_eq!(cells[game.get_index(2, 5)], Cell::I);
        assert_eq!(cells[game.get_index(3, 5)], Cell::I);
    }

    #[test]
    pub fn test_l_rotation_2() {
        let mut game = Game::from(
            r#" 
        LLLLL LLLL
        LLLLL LLLL
        LLL    LLL
        LLLLL LLLL
        "#,
        );
        game.set_piece(new_piece(Cell::I, 3, 0, Rotation::WEST));
        action(&mut game, Action::RotateCounterClockWise);
        render_board(&mut game);
        game.print();

        let cells = game.get_cell_vec();
        assert_eq!(cells[game.get_index(2, 3)], Cell::I);
        assert_eq!(cells[game.get_index(2, 4)], Cell::I);
        assert_eq!(cells[game.get_index(2, 5)], Cell::I);
        assert_eq!(cells[game.get_index(2, 6)], Cell::I);
    }

    #[test]
    pub fn test_cant_move_left_right() {
        let mut game = Game::from(
            r#"
        LLLLL LLLL
        LLL    LLL
        LLLLL LLLL
        LLLLL LLLL
        "#,
        );

        game.set_piece(new_piece(Cell::I, 3, 0, Rotation::EAST));
        actions(&mut game, &[Action::MoveRight, Action::MoveLeft]);
        render_board(&mut game);
        game.print();

        let cells = game.get_cell_vec();
        assert_eq!(cells[game.get_index(0, 5)], Cell::I);
        assert_eq!(cells[game.get_index(1, 5)], Cell::I);
        assert_eq!(cells[game.get_index(2, 5)], Cell::I);
        assert_eq!(cells[game.get_index(3, 5)], Cell::I);
    }

    #[test]
    pub fn test_wall_kick_rotation() {
        let mut game = Game::from(
            r#" 
        __________
        ____II____
        _____III__
        ______IIII
        _III___III
        II____IIII
        IIII__IIII
        IIIII_IIII
        "#,
        );

        let mut piece = Piece::new(Cell::J);
        piece.set_position(Point::new(3, 2));
        game.set_piece(piece);
        action(&mut game, Action::RotateClockWise);
        game.update_shadow_piece_position();
        game.merge_piece_into_board();
        game.print();

        let cells = game.get_cell_vec();
        assert_eq!(cells[game.get_index(4, 5)], Cell::J);
        assert_eq!(cells[game.get_index(5, 5)], Cell::J);
        assert_eq!(cells[game.get_index(6, 5)], Cell::J);
        assert_eq!(cells[game.get_index(6, 4)], Cell::J);
    }
}

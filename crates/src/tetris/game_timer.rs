#[derive(Debug)]
pub struct Timer {
    last_time: f64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { last_time: 0.0 }
    }

    pub fn get_elapsed_time(&mut self, current_time: f64) -> f64 {
        let elapsed_time = current_time - self.last_time;
        self.last_time = current_time;
        elapsed_time
    }
}

/// Game Timer tracks all of the games timing and timing state
pub struct GameTimer {
    timer: Timer,
    record_time: f64,
    update_now: bool,
    paused: bool,
}

impl GameTimer {
    /// Create a new game
    pub fn new() -> GameTimer {
        GameTimer {
            timer: Timer::new(),
            record_time: 0.0,
            update_now: false,
            paused: false,
        }
    }

    /// Update the time state
    pub fn update(&mut self, current_time: f64) {
        self.record_time = self.record_time + self.timer.get_elapsed_time(current_time);
    }

    /// Check it's if the game should update
    /// Make sure enough time has passed before dropping piece one more level
    pub fn can_update_game(&mut self, update_speed: f64) -> bool {
        let result = self.get_record_time() > update_speed || self.update_now;
        if self.update_now {
            self.update_now = false;
        }
        result
    }

    /// No matter what, update the game on the next update
    pub fn update_asap(&mut self) {
        self.update_now = true;
    }

    /// restart the game state timer
    pub fn reset(&mut self) {
        self.record_time = 0.0;
    }

    /// Get the amount of seconds in game that have passed
    pub fn get_record_time(&self) -> f64 {
        self.record_time
    }

    /// Toggle pausing the game on and off
    pub fn toggle_pause(&mut self) -> bool {
        self.paused = !self.paused;
        if self.is_running() {
            self.reset();
        }
        true
    }

    /// Check if the game is currently running
    pub fn is_running(&self) -> bool {
        !self.paused
    }

    /// Check if the game is currently paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

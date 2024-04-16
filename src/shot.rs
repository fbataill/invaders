use crate::{frame::{Drawable, Frame}, NUM_ROWS};
use rusty_time::Timer;
use std::time::Duration;

pub struct Shot {
    player_shot: bool,
    pub exploding: bool,
    timer: Timer,
    pub x: usize,
    pub y: usize,
}

impl Shot {
    pub fn new(x: usize, y: usize, player_shot: bool) -> Self {
        Self {
            x,
            y,
            player_shot,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),
        }
    }
    pub fn update(&mut self, mut delta: Duration) {
        if self.player_shot == false {
            delta = delta/2;
        }    
        self.timer.tick(delta);
        if self.timer.finished() && !self.exploding {
            if self.player_shot == true {
                if self.y > 0 {
                    self.y -= 1;
                }
            }
            else {
                if self.y < NUM_ROWS {
                    self.y += 1;
                }
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(Duration::from_millis(250));
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.finished()) || (self.y == 0) || (self.y == NUM_ROWS)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        if self.player_shot == true {
            frame[self.x][self.y] = if self.exploding { '*' } else { '|' };
        } else {
            frame[self.x][self.y] = if self.exploding { '*' } else { 'V' };
        }
    }
}

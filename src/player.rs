use crate::{frame::Drawable, invaders::Invaders, shot::Shot, NUM_COLS, NUM_ROWS};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn shoot(&mut self) {
        if self.shots.len() < 5 {
            self.shots.push(Shot::new(self.x, self.y - 1));
        } else {
        }
    }

    pub fn update(&mut self, delta: std::time::Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hit(&mut self, invaders: &mut Invaders) {
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    shot.explode();
                }
            }
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.y][self.x] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}

use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    vec![vec![" "; NUM_COLS]; NUM_ROWS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

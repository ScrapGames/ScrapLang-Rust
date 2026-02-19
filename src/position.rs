#[derive(Clone, Copy)]
pub struct Position {
    pub idx: usize,
    pub line: usize,
    pub line_idx: usize
}

impl Position {
    pub fn new(idx: usize, line: usize, line_idx: usize) -> Self {
        Position { idx, line, line_idx }
    }
}
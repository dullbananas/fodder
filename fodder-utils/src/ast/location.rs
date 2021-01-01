//pub type Position = combine::stream::position::SourcePosition;
/// Both `row` and `col` start at 1
#[derive(Copy, Clone)]
pub struct Position {
    pub row: u32,
    pub col: u32,
}


impl Position {
    pub fn next_col(mut self) -> Position {
        self.col += 1;
        self
    }
}


pub struct Located<T> {
    start: Position,
    end: Position,
    inner: T,
}

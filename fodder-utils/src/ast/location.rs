/// Both `row` and `col` start at 1
#[derive(Clone)]
pub struct Position {
    pub offset: usize,
    pub row: usize,
    pub col: usize,
}


pub struct Located<T> {
    start: Position,
    end: Position,
    inner: T,
}

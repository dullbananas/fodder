pub struct Span<T> {
    pub start: Position,
    pub end: Position,
    pub value: Box<T>,
}


pub struct Position {
    pub row: u16,
    pub col: u16,
}

use super::{Span};


pub enum Tuple<T> {
    Unit,
    Two(Span<T>, Span<T>),
    Three(Span<T>, Span<T>, Span<T>),
}

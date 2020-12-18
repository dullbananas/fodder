use super::{Span};


pub struct Case<P, E> {
    pub value: Span<E>,
    pub branches: Vec<Branch<P, E>>,
}


pub struct Branch<P, E> {
    pub pattern: P,
    pub body: Span<E>,
}

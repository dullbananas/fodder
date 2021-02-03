pub enum Tuple<T> {
    Unit,
    Tuple {
        first: T,
        second: T,
        others: Vec<T>,
    }
}


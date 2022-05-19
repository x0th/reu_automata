use std::fmt;

enum CellState {
    Dead,
    Alive
}

struct Grid {
    grid: Vec<Vec<CellState>>,
}

impl Grid {

}

// Stump implementation to print Grid with {} formatter, almost perfect!
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This is a grid!")
    }
}

fn main() {

}
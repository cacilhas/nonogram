#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Cell {
    #[default]
    Closed,
    Yes,
    No,
}

impl From<bool> for Cell {
    fn from(value: bool) -> Self {
        if value {
            Cell::Yes
        } else {
            Cell::No
        }
    }
}

impl From<Cell> for bool {
    fn from(value: Cell) -> Self {
        value == Cell::Yes
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    /// Shift the position.
    pub fn shift(self, dx: i32, dy: i32) -> Pos {
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn neighbors4(self) -> [Self; 4] {
        [
            self.shift(1, 0),
            self.shift(0, 1),
            self.shift(-1, 0),
            self.shift(0, -1),
        ]
    }
}

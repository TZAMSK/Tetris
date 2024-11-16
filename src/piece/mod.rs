use crate::engine::Offset;
pub struct Piece {
    pub kind: Kind,
}
impl Piece {
    // All pieces have 4 cells
    const CELL_COUNT: usize = 4;
}
// Variants of pieces
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}
impl Kind {
    // List of all variants
    pub const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::L,
        Self::J,
        Self::S,
        Self::Z,
    ];
    // Coords of every kind
    pub fn cells(&self) -> [Offset; Piece::CELL_COUNT] {
        match self {
            Kind::O => &[(1, 1), (1, 2), (2, 1), (2, 2)],
            Kind::I => &[(0, 2), (1, 2), (2, 2), (3, 2)],
            Kind::T => &[(0, 1), (1, 1), (2, 1), (1, 2)],
            Kind::L => &[(0, 1), (1, 1), (2, 1), (2, 2)],
            Kind::J => &[(0, 2), (0, 1), (1, 1), (2, 1)],
            Kind::S => &[(0, 1), (1, 1), (1, 2), (2, 2)],
            Kind::Z => &[(0, 2), (1, 2), (1, 1), (2, 1)],
        }
        .map(Offset::from)
    }
}

use crate::engine::{Coordinate, Matrix, Offset};
use cgmath::{EuclideanSpace, Zero};

pub struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    // All pieces have 4 cells
    const CELL_COUNT: usize = 4;

    // A list of tuples of coords of the 4 cells
    pub fn cells(&self) -> Option<[Coordinate; Self::CELL_COUNT]> {
        let offsets = self.kind.cells().map(self.rotator()).map(self.positioner());

        let mut coords = [Coordinate::origin(); Self::CELL_COUNT];

        for (offset, coord_slot) in offsets.into_iter().zip(&mut coords) {
            let positive_offset = offset.cast::<usize>()?;

            let coord = Coordinate::from_vec(positive_offset);

            if Matrix::in_bounds(coord) {
                *coord_slot = coord;
            } else {
                return None;
            }
        }

        Some(coords)
    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        |cell| match self.kind {
            Kind::O => cell,
            _ => {
                let grid_offset = self.rotation.intrinsic_offset() * (self.kind.grid_size() - 1);
                cell * self.rotation + grid_offset
            }
        }
    }

    fn positioner(&self) -> impl Fn(Offset) -> Offset {
        let position = self.position;
        move |cell| cell + position
    }
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
    // A list of all variants
    pub const ALL: [Self; 7] = [
        Self::O,
        Self::I,
        Self::T,
        Self::L,
        Self::J,
        Self::S,
        Self::Z,
    ];

    // Coords of all kinds
    pub fn cells(&self) -> [Offset; Piece::CELL_COUNT] {
        match self {
            Kind::O => &[(1, 1), (1, 2), (2, 1), (2, 2)],
            Kind::I => &[(0, 2), (1, 2), (2, 2), (3, 2)],
            Kind::T => &[(0, 1), (1, 1), (2, 1), (1, 2)],
            Kind::L => &[(0, 1), (1, 1), (2, 1), (2, 2)],
            Kind::J => &[(0, 2), (0, 1), (1, 1), (2, 1)],
            Kind::S => &[(1, 1), (1, 1), (1, 2), (2, 2)],
            Kind::Z => &[(0, 2), (1, 2), (1, 1), (2, 1)],
        }
        // map transforms each element of an iterator into a new form
        .map(Offset::from) // from converts to that type (here Offset)
    }

    fn grid_size(&self) -> isize {
        match self {
            Kind::I => 4,
            _ => 3,
        }
    }
}

// All variants of rotation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rotation {
    N,
    E,
    S,
    W,
}

impl Rotation {
    fn intrinsic_offset(&self) -> Offset {
        match self {
            Self::N => Offset::zero(),
            Self::E => Offset::new(0, 1),
            Self::S => Offset::new(1, 1),
            Self::W => Offset::new(1, 0),
        }
    }
}

// Multiplicator (*)
impl std::ops::Mul<Rotation> for Offset {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            // Multiplies by -1
            Rotation::N => self,
            Rotation::S => Self::new(-self.x, -self.y),
            Rotation::E => Self::new(self.y, -self.x),
            Rotation::W => Self::new(-self.y, self.x),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn z_piece_positioning() {
        let z = Piece {
            kind: Kind::Z,
            position: Offset::new(5, 6),
            rotation: Rotation::W,
        };

        assert_eq!(
            z.cells(),
            Some([(5, 6), (5, 7), (6, 7), (6, 8)].map(Coordinate::from))
        );
    }
}

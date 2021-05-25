#[derive(Copy, Clone, PartialEq, Debug, PartialOrd)]
/// H3 digit representing ijk+ axes direction.
/// Values will be within the lowest 3 bits of an integer.
pub enum Direction {
    /// H3 digit in center
    CENTER_DIGIT, //= 0,
    /// H3 digit in k-axes direction
    K_AXES_DIGIT, //= 1,
    /// H3 digit in j-axes direction
    J_AXES_DIGIT, //= 2,
    /// H3 digit in j == k direction
    JK_AXES_DIGIT, //= J_AXES_DIGIT | K_AXES_DIGIT, /* 3 */
    /// H3 digit in i-axes direction
    I_AXES_DIGIT, //= 4,
    /// H3 digit in i == k direction
    IK_AXES_DIGIT, //= I_AXES_DIGIT | K_AXES_DIGIT, /* 5 */
    /// H3 digit in i == j direction
    IJ_AXES_DIGIT, //= I_AXES_DIGIT | J_AXES_DIGIT, /* 6 */
    /// H3 digit in the invalid direction
    INVALID_DIGIT, //= 7,
                   // Valid digits will be less than this value. Same value as INVALID_DIGIT.
                   //NUM_DIGITS = INVALID_DIGIT
}

impl Direction {
    /// All valid directions for enumeration
    pub(crate) const VALID_DIRECTIONS: [Direction; 7] = [
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ];

    /// Rotates indexing digit 60 degrees counter-clockwise. Returns result.
    ///
    /// @param digit Indexing digit (between 1 and 6 inclusive)
    pub(crate) fn rotate60ccw(&self) -> Self {
        match self {
            Direction::K_AXES_DIGIT => Direction::IK_AXES_DIGIT,
            Direction::IK_AXES_DIGIT => Direction::I_AXES_DIGIT,
            Direction::I_AXES_DIGIT => Direction::IJ_AXES_DIGIT,
            Direction::IJ_AXES_DIGIT => Direction::J_AXES_DIGIT,
            Direction::J_AXES_DIGIT => Direction::JK_AXES_DIGIT,
            Direction::JK_AXES_DIGIT => Direction::K_AXES_DIGIT,

            Direction::CENTER_DIGIT | Direction::INVALID_DIGIT => *self,
        }
    }

    /// Rotates indexing digit 60 degrees clockwise. Returns result.
    ///
    /// @param digit Indexing digit (between 1 and 6 inclusive)
    pub(crate) fn rotate60cw(&self) -> Self {
        match self {
            Direction::K_AXES_DIGIT => Direction::JK_AXES_DIGIT,
            Direction::JK_AXES_DIGIT => Direction::J_AXES_DIGIT,
            Direction::J_AXES_DIGIT => Direction::IJ_AXES_DIGIT,
            Direction::IJ_AXES_DIGIT => Direction::I_AXES_DIGIT,
            Direction::I_AXES_DIGIT => Direction::IK_AXES_DIGIT,
            Direction::IK_AXES_DIGIT => Direction::K_AXES_DIGIT,

            Direction::CENTER_DIGIT | Direction::INVALID_DIGIT => *self,
        }
    }
}

macro_rules! from_dir {
    ($t : ty) => {
        impl From<$t> for Direction {
            fn from(v: $t) -> Self {
                match v {
                    0 => Direction::CENTER_DIGIT,
                    1 => Direction::K_AXES_DIGIT,
                    2 => Direction::J_AXES_DIGIT,
                    3 => Direction::JK_AXES_DIGIT,
                    4 => Direction::I_AXES_DIGIT,
                    5 => Direction::IK_AXES_DIGIT,
                    6 => Direction::IJ_AXES_DIGIT,
                    _ => Direction::INVALID_DIGIT,
                }
            }
        }

        impl From<Direction> for $t {
            fn from(d: Direction) -> $t {
                match d {
                    Direction::CENTER_DIGIT => 0,
                    Direction::K_AXES_DIGIT => 1,
                    Direction::J_AXES_DIGIT => 2,
                    Direction::JK_AXES_DIGIT => 3,
                    Direction::I_AXES_DIGIT => 4,
                    Direction::IK_AXES_DIGIT => 5,
                    Direction::IJ_AXES_DIGIT => 6,
                    Direction::INVALID_DIGIT => 7,
                }
            }
        }

        /*

        impl From<&Direction> for $t {
            fn from(d: &Direction) -> $t {
                match d {
                    Direction::CENTER_DIGIT => 0,
                    Direction::K_AXES_DIGIT => 1,
                    Direction::J_AXES_DIGIT => 2,
                    Direction::JK_AXES_DIGIT => 3,
                    Direction::I_AXES_DIGIT => 4,
                    Direction::IK_AXES_DIGIT => 5,
                    Direction::IJ_AXES_DIGIT => 6,
                    Direction::INVALID_DIGIT => 7,
                }
            }
        }

               impl Into<$t> for &Direction {
                   fn into(self) -> $t {
                       match self {
                           Direction::CENTER_DIGIT => 0,
                           Direction::K_AXES_DIGIT => 1,
                           Direction::J_AXES_DIGIT => 2,
                           Direction::JK_AXES_DIGIT => 3,
                           Direction::I_AXES_DIGIT => 4,
                           Direction::IK_AXES_DIGIT => 5,
                           Direction::IJ_AXES_DIGIT => 6,
                           Direction::INVALID_DIGIT => 7,
                       }
                   }
               }
        */
    };
}

from_dir!(u64);
from_dir!(usize);

impl std::ops::AddAssign<usize> for Direction {
    fn add_assign(&mut self, rhs: usize) {
        assert_eq!(rhs, 1);

        *self = match self {
            Direction::CENTER_DIGIT => Self::K_AXES_DIGIT,
            Direction::K_AXES_DIGIT => Self::J_AXES_DIGIT,
            Direction::J_AXES_DIGIT => Self::JK_AXES_DIGIT,
            Direction::JK_AXES_DIGIT => Self::I_AXES_DIGIT,
            Direction::I_AXES_DIGIT => Self::IK_AXES_DIGIT,
            Direction::IK_AXES_DIGIT => Self::IJ_AXES_DIGIT,
            Direction::IJ_AXES_DIGIT => Self::INVALID_DIGIT,
            Direction::INVALID_DIGIT => Self::INVALID_DIGIT,
        };
    }
}

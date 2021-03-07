
/// H3 digit representing ijk+ axes direction.
///Values will be within the lowest 3 bits of an integer.
pub enum Direction{
    /// H3 digit in center 
    CENTER_DIGIT = 0,
    /// H3 digit in k-axes direction 
    K_AXES_DIGIT = 1,
    /// H3 digit in j-axes direction 
    J_AXES_DIGIT = 2,
    /// H3 digit in j == k direction 
    JK_AXES_DIGIT = J_AXES_DIGIT | K_AXES_DIGIT, /* 3 */
    /// H3 digit in i-axes direction 
    I_AXES_DIGIT = 4,
    /// H3 digit in i == k direction 
    IK_AXES_DIGIT = I_AXES_DIGIT | K_AXES_DIGIT, /* 5 */
    /// H3 digit in i == j direction
    IJ_AXES_DIGIT = I_AXES_DIGIT | J_AXES_DIGIT, /* 6 */
    /// H3 digit in the invalid direction 
    INVALID_DIGIT = 7,
    /// Valid digits will be less than this value. Same value as INVALID_DIGIT.
    NUM_DIGITS = INVALID_DIGIT
}

/// CoordIJK unit vectors corresponding to the 7 H3 digits.
const UNIT_VECS : &[CoordIJK; 7] = [
    CoordIJK{i:0, j:0, k:0},  // direction 0
    CoordIJK{i:0, j:0, k:1},  // direction 1
    CoordIJK{i:0, j:1, k:0},  // direction 2
    CoordIJK{i:0, j:1, k:1},  // direction 3
    CoordIJK{i:1, j:0, k:0},  // direction 4
    CoordIJK{i:1, j:0, k:1},  // direction 5
    CoordIJK{i:1, j:1, k:0}   // direction 6
];



impl Direction {

    pub fn unit_vec(&self) -> CoordIJK {
        match self {
            CENTER_DIGIT => CoordIJK{i:0, j:0, k:0},  // direction 0
            K_AXES_DIGIT => CoordIJK{i:0, j:0, k:1},  // direction 1
            J_AXES_DIGIT => CoordIJK{i:0, j:1, k:0},  // direction 2
            JK_AXES_DIGIT => CoordIJK{i:0, j:1, k:1},  // direction 3
            I_AXES_DIGIT => CoordIJK{i:1, j:0, k:0},  // direction 4
            IK_AXES_DIGIT => CoordIJK{i:1, j:0, k:1},  // direction 5
            IJ_AXES_DIGIT => CoordIJK{i:1, j:1, k:0}  , // direction 6
            _ => todo!("unit vec for unhandled direction")

        }
    }

    /// Rotates indexing digit 60 degrees counter-clockwise. Returns result.
    pub fn _rotate60ccw(&self)  -> Self {
        match self {
            K_AXES_DIGIT  => Self::IK_AXES_DIGIT,
            IK_AXES_DIGIT  => Self::I_AXES_DIGIT,
            I_AXES_DIGIT  => Self::IJ_AXES_DIGIT,
            IJ_AXES_DIGIT  => Self::J_AXES_DIGIT,
            J_AXES_DIGIT  => Self::JK_AXES_DIGIT,
            JK_AXES_DIGIT  => Self::K_AXES_DIGIT,
            _ => self,
        }
    }

    /// Rotates indexing digit 60 degrees clockwise. Returns result.
    pub fn _rotate60cw(&self) -> Self {
        match self {
            K_AXES_DIGIT  => Self::JK_AXES_DIGIT,
            JK_AXES_DIGIT  => Self::J_AXES_DIGIT,
            J_AXES_DIGIT  => Self::IJ_AXES_DIGIT,
            IJ_AXES_DIGIT  => Self::I_AXES_DIGIT,
            I_AXES_DIGIT  => Self::IK_AXES_DIGIT,
            IK_AXES_DIGIT  => Self::K_AXES_DIGIT,
            _ => self,
        }
    }
}

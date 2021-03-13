use std::ops;

use crate::{constants::M_SIN60, vec2d::Vec2d};

/// IJK hexagon coordinates
///
/// Each axis is spaced 120 degrees apart.
pub struct CoordIJK {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

/// @brief CoordIJK unit vectors corresponding to the 7 H3 digits.
const UNIT_VECS: [CoordIJK; 7] = [
    CoordIJK::new(0, 0, 0), // direction 0
    CoordIJK::new(0, 0, 1), // direction 1
    CoordIJK::new(0, 1, 0), // direction 2
    CoordIJK::new(0, 1, 1), // direction 3
    CoordIJK::new(1, 0, 0), // direction 4
    CoordIJK::new(1, 0, 1), // direction 5
    CoordIJK::new(1, 1, 0), // direction 6
];

impl CoordIJK {
    pub const ZERO: CoordIJK = CoordIJK::new(0, 0, 0);

    pub const fn new(i: i32, j: i32, k: i32) -> Self {
        Self { i, j, k }
    }

    /**
     * Returns whether or not two ijk coordinates contain exactly the same
     * component values.
     *
     * @param c1 The first set of ijk coordinates.
     * @param c2 The second set of ijk coordinates.
     * @return 1 if the two addresses match, 0 if they do not.
     */
    pub fn _ijkMatches(c1: &Self, c2: &Self) -> bool {
        todo!("Replace all invocations of _ijkMatches with ==")
    }

    /**
     * Normalizes ijk coordinates by setting the components to the smallest possible
     * values. Works in place.
     *
     * @param c The ijk coordinates to normalize.
     */
    pub fn normalize(&mut self) {
        // remove any negative values
        if self.i < 0 {
            self.j -= self.i;
            self.k -= self.i;
            self.i = 0;
        }

        if self.j < 0 {
            self.i -= self.j;
            self.k -= self.j;
            self.j = 0;
        }

        if self.k < 0 {
            self.i -= self.k;
            self.j -= self.k;
            self.k = 0;
        }

        // remove the min value if needed
        let mut min = self.i;
        if self.j < min {
            min = self.j;
        }
        if self.k < min {
            min = self.k;
        }
        if min > 0 {
            self.i -= min;
            self.j -= min;
            self.k -= min;
        }
    }
}

impl From<Vec2d> for CoordIJK {
    /**
     * Determine the containing hex in ijk+ coordinates for a 2D cartesian
     * coordinate vector (from DGGRID).
     *
     * @param v The 2D cartesian coordinate vector.
     * @param h The ijk+ coordinates of the containing hex.
     */
    fn from(v: Vec2d) -> Self {
        /*
        double a1, a2;
        double x1, x2;
        int m1, m2;
        double r1, r2;

        // quantize into the ij system and then normalize
        let k = 0;

        let a1 = v.x.abs();
        let a2 = v.y.abs();

        // first do a reverse conversion
        x2 = a2 / M_SIN60;
        x1 = a1 + x2 / 2.0L;

        // check if we have the center of a hex
        m1 = x1;
        m2 = x2;

        // otherwise round correctly
        r1 = x1 - m1;
        r2 = x2 - m2;

        if (r1 < 0.5L) {
            if (r1 < 1.0L / 3.0L) {
                if (r2 < (1.0L + r1) / 2.0L) {
                    h.i = m1;
                    h.j = m2;
                } else {
                    h.i = m1;
                    h.j = m2 + 1;
                }
            } else {
                if (r2 < (1.0L - r1)) {
                    h.j = m2;
                } else {
                    h.j = m2 + 1;
                }

                if ((1.0L - r1) <= r2 && r2 < (2.0 * r1)) {
                    h.i = m1 + 1;
                } else {
                    h.i = m1;
                }
            }
        } else {
            if (r1 < 2.0L / 3.0L) {
                if (r2 < (1.0L - r1)) {
                    h.j = m2;
                } else {
                    h.j = m2 + 1;
                }

                if ((2.0L * r1 - 1.0L) < r2 && r2 < (1.0L - r1)) {
                    h.i = m1;
                } else {
                    h.i = m1 + 1;
                }
            } else {
                if (r2 < (r1 / 2.0L)) {
                    h.i = m1 + 1;
                    h.j = m2;
                } else {
                    h.i = m1 + 1;
                    h.j = m2 + 1;
                }
            }
        }

        // now fold across the axes if necessary

        if (v.x < 0.0L) {
            if ((h.j % 2) == 0)  // even
            {
                long long int axisi = h.j / 2;
                long long int diff = h.i - axisi;
                h.i = h.i - 2.0 * diff;
            } else {
                long long int axisi = (h.j + 1) / 2;
                long long int diff = h.i - axisi;
                h.i = h.i - (2.0 * diff + 1);
            }
        }

        if (v.y < 0.0L) {
            h.i = h.i - (2 * h.j + 1) / 2;
            h.j = -1 * h.j;
        }

        h._ijkNormalize()
            */

        // quantize into the ij system and then normalize
        let k = 0;

        let a1: f64 = v.x.abs();
        let a2: f64 = v.y.abs();

        // first do a reverse conversion
        let x2: f64 = a2 / M_SIN60;
        let x1: f64 = a1 + x2 / 2.0;

        // check if we have the center of a hex
        let m1 = x1 as i32;
        let m2 = x2 as i32;

        // otherwise round correctly
        let r1: f64 = x1 - m1 as f64;
        let r2: f64 = x2 - m2 as f64;

        let mut i: i32;
        let mut j: i32;

        if r1 < 0.5 {
            if r1 < 1.0 / 3.0 {
                if r2 < (1.0 + r1) / 2.0 {
                    i = m1;
                    j = m2;
                } else {
                    i = m1;
                    j = m2 + 1;
                }
            } else {
                if r2 < (1.0 - r1) {
                    j = m2;
                } else {
                    j = m2 + 1;
                }

                if 1.0 - r1 <= r2 && r2 < (2.0 * r1) {
                    i = m1 + 1;
                } else {
                    i = m1;
                }
            }
        } else {
            if r1 < 2.0 / 3.0 {
                if r2 < (1.0 - r1) {
                    j = m2;
                } else {
                    j = m2 + 1;
                }

                if 2.0 * r1 - 1.0 < r2 && r2 < (1.0 - r1) {
                    i = m1;
                } else {
                    i = m1 + 1;
                }
            } else {
                if r2 < (r1 / 2.0) {
                    i = m1 + 1;
                    j = m2;
                } else {
                    i = m1 + 1;
                    j = m2 + 1;
                }
            }
        }

        // now fold across the axes if necessary

        if v.x < 0.0 {
            if j % 2 == 0 {
                // even
                let axisi = j / 2;
                let diff = i - axisi;
                i = i - 2 * diff;
            } else {
                let axisi = (j + 1) / 2;
                let diff = i - axisi;
                i = i - (2 * diff + 1);
            }
        }

        if v.y < 0.0 {
            i = i - (2 * j + 1) / 2;
            j = -1 * j;
        }

        let mut h = CoordIJK { i, j, k };
        h.normalize();
        h
    }
}

impl ops::Add for CoordIJK {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let i = self.i + other.i;
        let j = self.j + other.j;
        let k = self.k + other.k;

        Self { i, j, k }
    }
}

impl ops::Sub for CoordIJK {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let i = self.i - other.i;
        let j = self.j - other.j;
        let k = self.k - other.k;

        Self { i, j, k }
    }
}

impl ops::Mul<i32> for CoordIJK {
    type Output = Self;
    fn mul(self, factor: i32) -> Self {
        let i = self.i * factor;
        let j = self.j * factor;
        let k = self.k * factor;
        Self { i, j, k }
    }
}

impl ops::AddAssign for CoordIJK {
    fn add_assign(&mut self, other: Self) {
        self.i += other.i;
        self.j += other.j;
        self.k += other.k;
    }
}

impl ops::MulAssign<i32> for CoordIJK {
    fn mul_assign(&mut self, factor: i32) {
        self.i *= factor;
        self.j *= factor;
        self.k *= factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unitIjkToDigit() {
        let zero = CoordIJK::ZERO;
        let i = CoordIJK::new(1, 0, 0);
        let outOfRange = CoordIJK::new(2, 0, 0);
        let unnormalizedZero = CoordIJK::new(2, 2, 2);

        //assert!(_unitIjkToDigit(&zero) == CENTER_DIGIT, "Unit IJK to zero");
        //assert!(_unitIjkToDigit(&i) == I_AXES_DIGIT, "Unit IJK to I axis");
        //assert!(_unitIjkToDigit(&outOfRange) == INVALID_DIGIT, "Unit IJK out of range");
        //assert!(_unitIjkToDigit(&unnormalizedZero) == CENTER_DIGIT, "Unnormalized unit IJK to zero");
    }

    #[test]
    fn test_neighbor() {
        let ijk = CoordIJK::ZERO;

        let zero = CoordIJK::ZERO;
        let i = CoordIJK::new(1, 0, 0);

        //_neighbor(&ijk, CENTER_DIGIT);
        //assert!(_ijkMatches(&ijk, &zero), "Center neighbor is self");
        //_neighbor(&ijk, I_AXES_DIGIT);
        //assert!(_ijkMatches(&ijk, &i), "I neighbor as expected");
        //_neighbor(&ijk, INVALID_DIGIT);
        //assert!(_ijkMatches(&ijk, &i), "Invalid neighbor is self");
    }
}

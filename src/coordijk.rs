use std::ops;

use crate::{
    constants::{M_SIN60, M_SQRT3_2},
    coordij::CoordIJ,
    vec2d::Vec2d,
    Direction,
};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
/// IJK hexagon coordinates
///
/// Each axis is spaced 120 degrees apart.
pub struct CoordIJK {
    pub i: i32,
    pub j: i32,
    pub k: i32,
}

impl From<(i32, i32, i32)> for CoordIJK {
    fn from(ijk: (i32, i32, i32)) -> Self {
        let i = ijk.0;
        let j = ijk.1;
        let k = ijk.2;
        Self { i, j, k }
    }
}

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

    /// @brief CoordIJK unit vectors corresponding to the 7 H3 digits.
    const UNIT_VECS: [(CoordIJK, Direction); 7] = [
        (CoordIJK::new(0, 0, 0), Direction::CENTER_DIGIT), // direction 0
        (CoordIJK::new(0, 0, 1), Direction::K_AXES_DIGIT), // direction 1
        (CoordIJK::new(0, 1, 0), Direction::J_AXES_DIGIT), // direction 2
        (CoordIJK::new(0, 1, 1), Direction::JK_AXES_DIGIT), // direction 3
        (CoordIJK::new(1, 0, 0), Direction::I_AXES_DIGIT), // direction 4
        (CoordIJK::new(1, 0, 1), Direction::IK_AXES_DIGIT), // direction 5
        (CoordIJK::new(1, 1, 0), Direction::IJ_AXES_DIGIT), // direction 6
    ];

    /**
     * Determines the H3 digit corresponding to a unit vector in ijk coordinates.
     *
     * @param ijk The ijk coordinates; must be a unit vector.
     * @return The H3 digit (0-6) corresponding to the ijk unit vector, or
     * INVALID_DIGIT on failure.
     */
    pub(crate) fn _unitIjkToDigit(&self) -> Direction {
        let mut c = *self;
        c.normalize();

        for (unit, digit) in Self::UNIT_VECS.iter() {
            if *unit == c {
                return *digit;
            }
        }

        Direction::INVALID_DIGIT
    }

    /**
     * Find the normalized ijk coordinates of the indexing parent of a cell in a
     * counter-clockwise aperture 7 grid. Works in place.
     *
     * @param ijk The ijk coordinates.
     */
    pub(crate) fn _upAp7(&mut self) {
        // convert to CoordIJ
        let i = self.i - self.k;
        let j = self.j - self.k;

        self.i = ((3 * i - j) as f64 / 7.0).round() as i32;
        self.j = ((i + 2 * j) as f64 / 7.0).round() as i32;
        self.k = 0;
        self.normalize();
    }

    /**
     * Find the normalized ijk coordinates of the indexing parent of a cell in a
     * clockwise aperture 7 grid. Works in place.
     *
     * @param ijk The ijk coordinates.
     */
    pub(crate) fn _upAp7r(&mut self) {
        // convert to CoordIJ
        let i = self.i - self.k;
        let j = self.j - self.k;

        self.i = ((2 * i + j) as f64 / 7.0).round() as i32;
        self.j = ((3 * j - i) as f64 / 7.0).round() as i32;
        self.k = 0;
        self.normalize();
    }

    /**
     * Find the normalized ijk coordinates of the hex centered on the indicated
     * hex at the next finer aperture 7 counter-clockwise resolution. Works in
     * place.
     *
     * @param ijk The ijk coordinates.
     */
    pub(crate) fn _downAp7(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK::new(3, 0, 1) * self.i;
        let jVec = CoordIJK::new(1, 3, 0) * self.j;
        let kVec = CoordIJK::new(0, 1, 3) * self.k;

        *self = iVec + jVec + kVec;
        self.normalize();
    }

    /**
     * Find the normalized ijk coordinates of the hex centered on the indicated
     * hex at the next finer aperture 7 clockwise resolution. Works in place.
     *
     * @param ijk The ijk coordinates.
     */
    pub(crate) fn _downAp7r(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK::new(3, 1, 0) * self.i;
        let jVec = CoordIJK::new(0, 3, 1) * self.j;
        let kVec = CoordIJK::new(1, 0, 3) * self.k;

        *self = iVec + jVec + kVec;
        self.normalize();
    }

    /**
     * Find the normalized ijk coordinates of the hex centered on the indicated
     * hex at the next finer aperture 3 counter-clockwise resolution. Works in
     * place.
     *
     * @param ijk The ijk coordinates.
     */
    pub(crate) fn _downAp3(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK::new(2, 0, 1) * self.i;
        let jVec = CoordIJK::new(1, 2, 0) * self.j;
        let kVec = CoordIJK::new(0, 1, 2) * self.k;

        *self = iVec + jVec + kVec;
        self.normalize();
    }

    /**
     * Find the normalized ijk coordinates of the hex centered on the indicated
     * hex at the next finer aperture 3 clockwise resolution. Works in place.
     *
     * @param ijk The ijk coordinates.
     */
    pub(crate) fn _downAp3r(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK::new(2, 1, 0) * self.i;
        let jVec = CoordIJK::new(0, 2, 1) * self.j;
        let kVec = CoordIJK::new(1, 0, 2) * self.k;

        *self = iVec + jVec + kVec;
        self.normalize();
    }

    /**
     * Find the normalized ijk coordinates of the hex in the specified digit
     * direction from the specified ijk coordinates. Works in place.
     *
     * @param ijk The ijk coordinates.
     * @param digit The digit direction from the original ijk coordinates.
     */
    pub(crate) fn _neighbor(&mut self, digit: Direction) {
        if digit != Direction::CENTER_DIGIT && digit != Direction::INVALID_DIGIT {
            let unit = Self::UNIT_VECS.iter().find(|(_, d)| *d == digit).unwrap().0;

            *self += unit;
            self.normalize();
        }
    }

    /**
     * Given cube coords as doubles, round to valid integer coordinates. Algorithm
     * from https://www.redblobgames.com/grids/hexagons/#rounding
     * @param i   Floating-point I coord
     * @param j   Floating-point J coord
     * @param k   Floating-point K coord
     * @param ijk IJK coord struct, modified in place
     */
    pub(crate) fn cubeRound(i: f64, j: f64, k: f64) -> CoordIJK {
        let mut ri = i.round() as i32;
        let mut rj = j.round() as i32;
        let mut rk = k.round() as i32;

        let iDiff = (ri as f64 - i).abs();
        let jDiff = (rj as f64 - j).abs();
        let kDiff = (rk as f64 - k).abs();

        // Round, maintaining valid cube coords
        if iDiff > jDiff && iDiff > kDiff {
            ri = -rj - rk;
        } else if jDiff > kDiff {
            rj = -ri - rk;
        } else {
            rk = -ri - rj;
        }

        CoordIJK {
            i: ri,
            j: rj,
            k: rk,
        }
    }

    /// Convert IJK coordinates to cube coordinates, in place
    pub(crate) fn ijkToCube(&mut self) {
        self.i = -self.i + self.k;
        self.j = self.j - self.k;
        self.k = -self.i - self.j;
    }

    /// Convert cube coordinates to IJK coordinates, in place
    pub(crate) fn cubeToIjk(&mut self) {
        self.i = -self.i;
        self.k = 0;
        self.normalize();
    }

    /**
     * Find the center point in 2D cartesian coordinates of a hex.
     *
     * @param h The ijk coordinates of the hex.
     * @param v The 2D cartesian coordinates of the hex center point.
     */
    pub(crate) fn _ijkToHex2d(&self) -> Vec2d {
        let i = (self.i - self.k) as f64;
        let j = (self.j - self.k) as f64;

        let x = i - 0.5 * j;
        let y = j * M_SQRT3_2;
        Vec2d { x, y }
    }

    /**
     * Finds the distance between the two coordinates. Returns result.
     *
     * @param c1 The first set of ijk coordinates.
     * @param c2 The second set of ijk coordinates.
     */
    pub(crate) fn ijkDistance(&self, other: &Self) -> i32 {
        let mut diff = *self - *other;
        diff.normalize();

        let i = diff.i.abs();
        let j = diff.j.abs();
        let k = diff.k.abs();
        i.max(j).max(k)
    }

    /// Rotates ijk coordinates 60 degrees counter-clockwise. Works in place.
    pub(crate) fn _ijkRotate60ccw(&mut self) {
        // unit vector rotations
        let iVec = CoordIJK::new(1, 1, 0) * self.i;
        let jVec = CoordIJK::new(0, 1, 1) * self.j;
        let kVec = CoordIJK::new(1, 0, 1) * self.k;

        *self = iVec + jVec + kVec;
        self.normalize();
    }

    pub(crate) fn _ijkRotate60cw(&mut self) {
        // unit vector rotations
        let iVec = CoordIJK::new(1, 0, 1) * self.i;
        let jVec = CoordIJK::new(1, 1, 0) * self.j;
        let kVec = CoordIJK::new(0, 1, 1) * self.k;

        *self = iVec + jVec + kVec;
        self.normalize();
    }

    /**
     * Transforms coordinates from the IJK+ coordinate system to the IJ coordinate
     * system.
     *
     * @param ijk The input IJK+ coordinates
     * @param ij The output IJ coordinates
     */
    pub(crate) fn ijkToIj(&self) -> CoordIJ {
        CoordIJ {
            i: self.i - self.k,
            j: self.j - self.k,
        }
    }
}

impl From<&crate::coordij::CoordIJ> for CoordIJK {
    /**
     * Transforms coordinates from the IJ coordinate system to the IJK+ coordinate
     * system.
     *
     * @param ij The input IJ coordinates
     * @param ijk The output IJK+ coordinates
     */
    fn from(ij: &crate::coordij::CoordIJ) -> Self {
        let mut ijk = Self {
            i: ij.i,
            j: ij.j,
            k: 0,
        };

        ijk.normalize();
        ijk
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

        assert_eq!(
            zero._unitIjkToDigit(),
            Direction::CENTER_DIGIT,
            "Unit IJK to zero"
        );

        assert_eq!(
            i._unitIjkToDigit(),
            Direction::I_AXES_DIGIT,
            "Unit IJK to I axis"
        );

        assert_eq!(
            outOfRange._unitIjkToDigit(),
            Direction::INVALID_DIGIT,
            "Unit IJK out of range"
        );

        assert_eq!(
            unnormalizedZero._unitIjkToDigit(),
            Direction::CENTER_DIGIT,
            "Unnormalized unit IJK to zero"
        );
    }

    #[test]
    fn test_neighbor() {
        let mut ijk = CoordIJK::ZERO;
        let zero = CoordIJK::ZERO;
        let i = CoordIJK::new(1, 0, 0);

        ijk._neighbor(Direction::CENTER_DIGIT);
        assert_eq!(ijk, zero, "Center neighbor is self");

        ijk._neighbor(Direction::I_AXES_DIGIT);
        assert_eq!(ijk, i, "I neighbor as expected");

        ijk._neighbor(Direction::INVALID_DIGIT);
        assert_eq!(ijk, i, "Invalid neighbor is self");
    }

    #[test]
    fn ijkToIj_zero() {
        let ijk = CoordIJK::new(0, 0, 0);
        let ij = ijk.ijkToIj();

        assert_eq!(ij.i, 0, "ij.i zero");
        assert_eq!(ij.j, 0, "ij.j zero");

        let ijk = ij.ijToIjk();
        assert_eq!(ijk.i, 0, "ijk.i zero");
        assert_eq!(ijk.j, 0, "ijk.j zero");
        assert_eq!(ijk.k, 0, "ijk.k zero");
    }

    #[test]
    fn ijkToIj_roundtrip() {
        for dir in Direction::VALID_DIRECTIONS.iter() {
            let mut ijk = CoordIJK::default();
            ijk._neighbor(*dir);

            let ij = ijk.ijkToIj();
            let recovered = ij.ijToIjk();
            assert_eq!(ijk, recovered, "got same ijk coordinates back");
        }
    }

    #[test]
    fn ijkToCube_roundtrip() {
        for dir in Direction::VALID_DIRECTIONS.iter() {
            let mut ijk = CoordIJK::default();
            ijk._neighbor(*dir);

            let original = ijk.clone();

            ijk.ijkToCube();
            ijk.cubeToIjk();
            assert_eq!(ijk, original, "got same ijk coordinates back");
        }
    }

    #[test]
    fn _unitIjkToDigit() {
        let zero = CoordIJK::default();
        let i = CoordIJK::new(1, 0, 0);
        let outOfRange = CoordIJK::new(2, 0, 0);
        let unnormalizedZero = CoordIJK::new(2, 2, 2);

        assert_eq!(
            zero._unitIjkToDigit(),
            Direction::CENTER_DIGIT,
            "Unit IJK to zero"
        );
        assert_eq!(
            i._unitIjkToDigit(),
            Direction::I_AXES_DIGIT,
            "Unit IJK to I axis"
        );
        assert_eq!(
            outOfRange._unitIjkToDigit(),
            Direction::INVALID_DIGIT,
            "Unit IJK out of range"
        );
        assert_eq!(
            unnormalizedZero._unitIjkToDigit(),
            Direction::CENTER_DIGIT,
            "Unnormalized unit IJK to zero"
        );
    }

    #[test]
    fn neighbor() {
        let mut ijk = CoordIJK::default();
        let zero = CoordIJK::default();
        let i = CoordIJK::new(1, 0, 0);

        ijk._neighbor(Direction::CENTER_DIGIT);
        assert_eq!(ijk, zero, "Center neighbor is self");

        ijk._neighbor(Direction::I_AXES_DIGIT);
        assert_eq!(ijk, i, "I neighbor as expected");

        ijk._neighbor(Direction::INVALID_DIGIT);
        assert_eq!(ijk, i, "Invalid neighbor is self");
    }
}

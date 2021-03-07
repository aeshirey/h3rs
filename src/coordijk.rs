use std::{cmp, ops};

#[derive(Copy, Clone, Default)]
/// IJK hexagon coordinates
///Each axis is spaced 120 degrees apart.
pub struct CoordIJK {
    /// i component
    i: i32,
    /// j component
    j: i32,
    /// k component
    k: i32,
}

impl CoordIJK {
    pub const fn new(i: i32, j: i32, k: i32) -> Self {
        Self { i, j, k }
    }

    /// Sets an IJK coordinate to the specified component values.
    pub fn _setIJK(&mut self, i: i32, j: i32, k: i32) {
        self.i = i;
        self.j = j;
        self.k = k;
    }

    /// Uniformly scale ijk coordinates by a scalar. Works in place.
    pub fn _ijkScale(&mut self, factor: i32) {
        self.i *= factor;
        self.j *= factor;
        self.k *= factor;
    }

    /// Normalizes ijk coordinates by setting the components to the smallest possible values. Works in place.
    pub fn _ijkNormalize(&mut self) {
        // remove any negative values
        if (self.i < 0) {
            self.j -= self.i;
            self.k -= self.i;
            self.i = 0;
        }

        if (self.j < 0) {
            self.i -= self.j;
            self.k -= self.j;
            self.j = 0;
        }

        if (self.k < 0) {
            self.i -= self.k;
            self.j -= self.k;
            self.k = 0;
        }

        // remove the min value if needed
        let min = self.i;
        if (self.j < min) {
            min = self.j;
        }
        if (self.k < min) {
            min = self.k;
        }
        if (min > 0) {
            self.i -= min;
            self.j -= min;
            self.k -= min;
        }
    }

    /// Returns whether or not two ijk coordinates contain exactly the same component values.
    pub fn _ijkMatches(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.k == other.k
    }

    /// Add two ijk coordinates.
    pub fn _ijkAdd(&self, other: &Self) -> Self {
        let i = self.i + other.i;
        let j = self.j + other.j;
        let k = self.k + other.k;

        Self { i, j, k }
    }

    /// Subtract two ijk coordinates.
    pub fn _ijkSub(&self, other: &Self) -> Self {
        let i = self.i - other.i;
        let j = self.j - other.j;
        let k = self.k - other.k;

        Self { i, j, k }
    }

    /// Determines the H3 digit corresponding to a unit vector in ijk coordinates.
    ///
    ///@param ijk The ijk coordinates; must be a unit vector.
    ///@return The H3 digit (0-6) corresponding to the ijk unit vector, or
    ///INVALID_DIGIT on failure.
    pub fn _unitIjkToDigit(&self) -> Direction {
        let mut c = self.clone();
        c._ijkNormalize();

        todo!();

        /*
        let digit = Direction ::INVALID_DIGIT;
        for (Direction i = CENTER_DIGIT; i < NUM_DIGITS; i++) {
            if (c._ijkMatches(&UNIT_VECS[i])) {
                digit = i;
                break;
            }
        }

        digit
        */
    }

    /// Find the normalized ijk coordinates of the indexing parent of a cell in a counter-clockwise aperture 7 grid. Works in place.
    pub fn _upAp7(&mut self) {
        // convert to CoordIJ
        let i = self.i - self.k;
        let j = self.j - self.k;

        // TODO: confirm that Rust's rounding matches CPP's lroundl
        todo!()
        /*
            self.i = (int)lroundl((3 * i - j) / 7.0);
        self.j = (int)lroundl((i + 2 * j) / 7.0);
        self.k = 0;
        self._ijkNormalize();
        */
    }

    /// Find the center point in 2D cartesian coordinates of a hex.
    pub fn _ijkToHex2d(&self) -> Vec2d {
        let i = self.i - self.k;
        let j = self.j - self.k;

        let x = i as f64 - 0.5 * j as f64;
        let y = j * M_SQRT3_2;

        Vec2d { x, y }
    }

    /// Find the normalized ijk coordinates of the hex in the specified digit direction from the specified ijk coordinates. Works in place.
    pub fn _neighbor(&mut self, digit: Direction) {
        //if (digit > CENTER_DIGIT && digit < NUM_DIGITS) {
        //_ijkAdd(ijk, &UNIT_VECS[digit], ijk);
        let ijk = self + digit.unit_vecs();
        self += ijk;
        self._ijkNormalize();
        //}
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated hex at the next finer aperture 7 clockwise resolution. Works in place.
    pub fn _downAp7r(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK { i: 3, j: 1, k: 0 } * self.i;
        let jVec = CoordIJK { i: 0, j: 3, k: 1 } * self.j;
        let kVec = CoordIJK { i: 1, j: 0, k: 3 } * self.k;

        self = iVec + jVec + kVec;
        self._ijkNormalize();
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated hex at the next finer aperture 7 counter-clockwise resolution. Works in place.
    pub fn _downAp7(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK { i: 3, j: 0, k: 1 } * self.i;
        let jVec = CoordIJK { i: 1, j: 3, k: 0 } * self.j;
        let kVec = CoordIJK { i: 0, j: 1, k: 3 } * self.k;

        self = iVec + jVec + kVec;
        self._ijkNormalize();
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated hex at the next finer aperture 3 counter-clockwise resolution. Works in place.
    pub fn _downAp3(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK { i: 2, j: 0, k: 1 } * self.i;
        let jVec = CoordIJK { i: 1, j: 2, k: 0 } * self.j;
        let kVec = CoordIJK { i: 0, j: 1, k: 2 } * self.k;

        self = iVec + jVec + kVec;
        self._ijkNormalize();
    }

    /// Find the normalized ijk coordinates of the hex centered on the indicated hex at the next finer aperture 3 clockwise resolution. Works in place.
    pub fn _downAp3r(&mut self) {
        // res r unit vectors in res r+1
        let iVec = CoordIJK { i: 2, j: 1, k: 0 } * self.i;
        let jVec = CoordIJK { i: 0, j: 2, k: 1 } * self.j;
        let kVec = CoordIJK { i: 1, j: 0, k: 2 } * self.k;

        self = iVec + jVec + kVec;
        self._ijkNormalize();
    }

    /// Find the normalized ijk coordinates of the indexing parent of a cell in a clockwise aperture 7 grid. Works in place.
    pub fn _upAp7r(&mut self) {
        // convert to CoordIJ
        let i = self.i - self.k;
        let j = self.j - self.k;

        todo!()
        /*
        self.i = (int)lroundl((2 * i + j) / 7.0);
        self.j = (int)lroundl((3 * j - i) / 7.0);
        self.k = 0;
        self._ijkNormalize();
        */
    }

    /// Rotates ijk coordinates 60 degrees counter-clockwise. Works in place.
    pub fn _ijkRotate60ccw(&mut self) {
        // unit vector rotations
        let mut iVec = CoordIJK { i: 1, j: 1, k: 0 } * self.i;
        let mut jVec = CoordIJK { i: 0, j: 1, k: 1 } * self.j;
        let mut kVec = CoordIJK { i: 1, j: 0, k: 1 } * self.k;

        self = iVec + jVec + kVec;
        self._ijkNormalize();
    }

    /// Rotates ijk coordinates 60 degrees clockwise. Works in place.
    pub fn _ijkRotate60cw(&mut self) {
        // unit vector rotations
        let iVec = CoordIJK { i: 1, j: 0, k: 1 } * self.i;
        let jVec = CoordIJK { i: 1, j: 1, k: 0 } * self.j;
        let kVec = CoordIJK { i: 0, j: 1, k: 1 } * self.k;

        self = iVec + jVec + kVec;
        self._ijkNormalize();
    }

    /// Finds the distance between the two coordinates. Returns result.
    pub fn ijkDistance(&self, other: &Self) -> i32c {
        let mut diff = self - other;
        diff._ijkNormalize();

        let i = diff.i.abs();
        let j = diff.j.abs();
        let k = diff.k.abs();

        cmp::max(i, cmp::max(j, k))
    }

    /// Transforms coordinates from the IJK+ coordinate system to the IJ coordinate system.
    pub fn ijkToIj(&self) -> CoordIJ {
        let i = self.i - self.k;
        let j = self.j - self.k;
        CoordIJ { i, j }
    }

    /// Convert IJK coordinates to cube coordinates, in place
    pub fn ijkToCube(&mut self) {
        self.i = -self.i + self.k;
        self.j = self.j - self.k;
        self.k = -self.i - self.j;
    }

    /// Convert cube coordinates to IJK coordinates, in place
    pub fn cubeToIjk(&mut self) {
        self.i = -self.i;
        self.k = 0;
        self._ijkNormalize();
    }

    /**
     * Given cube coords as doubles, round to valid integer coordinates. Algorithm
     * from https://www.redblobgames.com/grids/hexagons/#rounding
     * @param i   Floating-point I coord
     * @param j   Floating-point J coord
     * @param k   Floating-point K coord
     * @param ijk IJK coord struct, modified in place
     */
    fn cubeRound(&mut self, i: f64, j: f64, k: f64) {
        let mut ri = i.round() as i32;
        let mut rj = j.round() as i32;
        let mut rk = k.round() as i32;

        let iDiff = (ri - i).abs();
        let jDiff = (rj - j).abs();
        let kDiff = (rk - k).abs();

        // Round, maintaining valid cube coords
        if (iDiff > jDiff && iDiff > kDiff) {
            ri = -rj - rk;
        } else if (jDiff > kDiff) {
            rj = -ri - rk;
        } else {
            rk = -ri - rj;
        }

        self.i = ri;
        self.j = rj;
        self.k = rk;
    }
}

impl ops::Add for CoordIJK {
    fn add(&self, other: Self) -> Self {
        let i = self.i + other.i;
        let j = self.j + other.j;
        let k = self.k + other.k;

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

impl ops::Sub for CoordIJK {
    fn sub(&self, other: Self) -> Self {
        let i = self.i - other.i;
        let j = self.j - other.j;
        let k = self.k - other.k;

        Self { i, j, k }
    }
}

impl ops::Mul<i32> for CoordIJK {
    fn mul(&self, factor: i32) -> Self {
        let i = self.i = factor;
        let j = self.j = factor;
        let k = self.k = factor;
        Self { i, j, k }
    }
}
impl ops::MulAssign<i32> for CoordIJK {
    fn mul_assign(&mut self, factor: i32) {
        self.i *= factor;
        self.j *= factor;
        self.k *= factor;
    }
}

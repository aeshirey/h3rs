use crate::constants::*;
use crate::faceijk::*;
use crate::geocoord::*;
use crate::h3index::{H3Index, Resolution};
use crate::CoordIJK;

#[derive(PartialEq, Default)]
pub struct Vec2d {
    /// x component,
    x: f64,
    /// y component
    y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculates the magnitude of a 2D cartesian vector.
    // @param v The 2D cartesian vector.
    // @return The magnitude of the vector.
    fn _v2dMag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /**
     * Whether two 2D vectors are equal. Does not consider possible false
     * negatives due to floating-point errors.
     * @param v1 First vector to compare
     * @param v2 Second vector to compare
     * @return Whether the vectors are equal
     */
    fn _v2dEquals(&self, other: &Self) -> bool {
        todo!()
    }

    /**
     * Finds the intersection between two lines. Assumes that the lines intersect
     * and that the intersection is not at an endpoint of either line.
     * @param p0 The first endpoint of the first line.
     * @param p1 The second endpoint of the first line.
     * @param p2 The first endpoint of the second line.
     * @param p3 The second endpoint of the second line.
     * @param inter The intersection point.
     */
    fn _v2dIntersect(p0: &Self, p1: &Self, p2: &Self, p3: &Self) -> Self {
        let s1 = Vec2d::new(p1.x - p0.x, p1.y - p0.y);
        let s2 = Vec2d::new(p3.x - p2.x, p3.y - p2.y);

        let t = (s2.x * (p0.y - p2.y) - s2.y * (p0.x - p2.x)) / (-s2.x * s1.y + s1.x * s2.y);

        Vec2D::new(p0.x + (t * s1.x), p0.y + (t * s1.y))
    }

    /// Determine the containing hex in ijk+ coordinates for a 2D cartesian coordinate vector (from DGGRID).
    pub fn _hex2dToCoordIJK(&self) -> CoordIJK {
        // quantize into the ij system and then normalize
        let k = 0;

        let a1: f64 = self.x.abs();
        let a2: f64 = self.y.abs();

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

        if self.x < 0.0 {
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

        if self.y < 0.0 {
            i = i - (2 * j + 1) / 2;
            j = -1 * j;
        }

        let h = CoordIJK { i, j, k };
        h._ijkNormalize();
        h
    }

    /// Determines the center point in spherical coordinates of a cell given by 2D hex coordinates on a particular icosahedral face.
    ///
    ///@param v The 2D hex coordinates of the cell.
    ///@param face The icosahedral face upon which the 2D hex coordinate system is
    ///            centered.
    ///@param res The H3 resolution of the cell.
    ///@param substrate Indicates whether or not this grid is actually a substrate
    ///       grid relative to the specified resolution.
    ///@param g The spherical coordinates of the cell center point.
    pub(crate) fn _hex2dToGeo(&self, face: i32, res: Resolution, substrate: bool) -> GeoCoord {
        // calculate (r, theta) in hex2d
        let mut r: f64 = self._v2dMag();

        if r < EPSILON {
            return faceCenterGeo[face];
        }

        let mut theta: f64 = f64::atan2(self.y, self.x);

        // scale for current resolution length u
        for _ in 0..res {
            r /= M_SQRT7;
        }

        // scale accordingly if this is a substrate grid
        if substrate {
            r /= 3.0;
            if H3Index::isResClassIII(res) {
                r /= M_SQRT7;
            }
        }

        r *= RES0_U_GNOMONIC;

        // perform inverse gnomonic scaling of r
        r = r.atan();

        // adjust theta for Class III
        // if a substrate grid, then it's already been adjusted for Class III
        if !substrate && H3Index::isResClassIII(res) {
            theta = _posAngleRads(theta + M_AP7_ROT_RADS);
        }

        // find theta as an azimuth
        theta = _posAngleRads(faceAxesAzRadsCII[face][0] - theta);

        // now find the point at (r,theta) from the face center
        GeoCoord::_geoAzDistanceRads(&faceCenterGeo[face], theta, r)
    }
}

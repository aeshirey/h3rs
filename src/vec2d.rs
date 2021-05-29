use crate::{GeoCoord, Resolution, constants::{self, M_SIN60}, coordijk::CoordIJK, faceCenterGeo, geocoord::{_geoAzDistanceRads, _posAngleRads}, vec3d::Vec3d};



/** @brief icosahedron face centers in x/y/z on the unit sphere */
pub(crate) const faceCenterPoint: [Vec3d; constants::NUM_ICOSA_FACES] = [
    Vec3d::new(0.2199307791404606, 0.6583691780274996, 0.7198475378926182), // face  0
    Vec3d::new(-0.2139234834501421, 0.1478171829550703, 0.9656017935214205), // face  1
    Vec3d::new(0.1092625278784797, -0.4811951572873210, 0.8697775121287253), // face  2
    Vec3d::new(0.7428567301586791, -0.3593941678278028, 0.5648005936517033), // face  3
    Vec3d::new(0.8112534709140969, 0.3448953237639384, 0.4721387736413930), // face  4
    Vec3d::new(-0.1055498149613921, 0.9794457296411413, 0.1718874610009365), // face  5
    Vec3d::new(-0.8075407579970092, 0.1533552485898818, 0.5695261994882688), // face  6
    Vec3d::new(-0.2846148069787907, -0.8644080972654206, 0.4144792552473539), // face  7
    Vec3d::new(0.7405621473854482, -0.6673299564565524, -0.0789837646326737), // face  8
    Vec3d::new(0.8512303986474293, 0.4722343788582681, -0.2289137388687808), // face  9
    Vec3d::new(-0.7405621473854481, 0.6673299564565524, 0.0789837646326737), // face 10
    Vec3d::new(-0.8512303986474292, -0.4722343788582682, 0.2289137388687808), // face 11
    Vec3d::new(0.1055498149613919, -0.9794457296411413, -0.1718874610009365), // face 12
    Vec3d::new(0.8075407579970092, -0.1533552485898819, -0.5695261994882688), // face 13
    Vec3d::new(0.2846148069787908, 0.8644080972654204, -0.4144792552473539), // face 14
    Vec3d::new(-0.7428567301586791, 0.3593941678278027, -0.5648005936517033), // face 15
    Vec3d::new(
        -0.8112534709140971,
        -0.3448953237639382,
        -0.4721387736413930,
    ), // face 16
    Vec3d::new(
        -0.2199307791404607,
        -0.6583691780274996,
        -0.7198475378926182,
    ), // face 17
    Vec3d::new(0.2139234834501420, -0.1478171829550704, -0.9656017935214205), // face 18
    Vec3d::new(-0.1092625278784796, 0.4811951572873210, -0.8697775121287253), // face 19
];

/** @brief icosahedron face ijk axes as azimuth in radians from face center to
 * vertex 0/1/2 respectively
 */
pub(crate) const faceAxesAzRadsCII: [[f64; 3]; constants::NUM_ICOSA_FACES] = [
    [
        5.619958268523939882,
        3.525563166130744542,
        1.431168063737548730,
    ], // face  0
    [
        5.760339081714187279,
        3.665943979320991689,
        1.571548876927796127,
    ], // face  1
    [
        0.780213654393430055,
        4.969003859179821079,
        2.874608756786625655,
    ], // face  2
    [
        0.430469363979999913,
        4.619259568766391033,
        2.524864466373195467,
    ], // face  3
    [
        6.130269123335111400,
        4.035874020941915804,
        1.941478918548720291,
    ], // face  4
    [
        2.692877706530642877,
        0.598482604137447119,
        4.787272808923838195,
    ], // face  5
    [
        2.982963003477243874,
        0.888567901084048369,
        5.077358105870439581,
    ], // face  6
    [
        3.532912002790141181,
        1.438516900396945656,
        5.627307105183336758,
    ], // face  7
    [
        3.494305004259568154,
        1.399909901866372864,
        5.588700106652763840,
    ], // face  8
    [
        3.003214169499538391,
        0.908819067106342928,
        5.097609271892733906,
    ], // face  9
    [
        5.930472956509811562,
        3.836077854116615875,
        1.741682751723420374,
    ], // face 10
    [
        0.138378484090254847,
        4.327168688876645809,
        2.232773586483450311,
    ], // face 11
    [
        0.448714947059150361,
        4.637505151845541521,
        2.543110049452346120,
    ], // face 12
    [
        0.158629650112549365,
        4.347419854898940135,
        2.253024752505744869,
    ], // face 13
    [
        5.891865957979238535,
        3.797470855586042958,
        1.703075753192847583,
    ], // face 14
    [
        2.711123289609793325,
        0.616728187216597771,
        4.805518392002988683,
    ], // face 15
    [
        3.294508837434268316,
        1.200113735041072948,
        5.388903939827463911,
    ], // face 16
    [
        3.804819692245439833,
        1.710424589852244509,
        5.899214794638635174,
    ], // face 17
    [
        3.664438879055192436,
        1.570043776661997111,
        5.758833981448388027,
    ], // face 18
    [
        2.361378999196363184,
        0.266983896803167583,
        4.455774101589558636,
    ], // face 19
];

// indexes for faceNeighbors table
/// IJ quadrant faceNeighbors table direction
pub(crate) const IJ: i32 = 1;
/// KI quadrant faceNeighbors table direction
pub(crate) const KI: i32 = 2;
/// JK quadrant faceNeighbors table direction
pub(crate) const JK: i32 = 3;

/// Invalid face index
const INVALID_FACE: i32 = -1;

/** @brief direction from the origin face to the destination face, relative to
 * the origin face's coordinate system, or -1 if not adjacent.
 */
const adjacentFaceDir: [[i32; constants::NUM_ICOSA_FACES]; constants::NUM_ICOSA_FACES] = [
    [
        0, KI, -1, -1, IJ, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 0
    [
        IJ, 0, KI, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 1
    [
        -1, IJ, 0, KI, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 2
    [
        -1, -1, IJ, 0, KI, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 3
    [
        KI, -1, -1, IJ, 0, -1, -1, -1, -1, JK, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 4
    [
        JK, -1, -1, -1, -1, 0, -1, -1, -1, -1, IJ, -1, -1, -1, KI, -1, -1, -1, -1, -1,
    ], // face 5
    [
        -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1, -1, -1, -1,
    ], // face 6
    [
        -1, -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1, -1, -1,
    ], // face 7
    [
        -1, -1, -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1, -1,
    ], // face 8
    [
        -1, -1, -1, -1, JK, -1, -1, -1, -1, 0, -1, -1, -1, KI, IJ, -1, -1, -1, -1, -1,
    ], // face 9
    [
        -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1, -1, -1, -1,
    ], // face 10
    [
        -1, -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1, -1, -1,
    ], // face 11
    [
        -1, -1, -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1, -1,
    ], // face 12
    [
        -1, -1, -1, -1, -1, -1, -1, -1, IJ, KI, -1, -1, -1, 0, -1, -1, -1, -1, JK, -1,
    ], // face 13
    [
        -1, -1, -1, -1, -1, KI, -1, -1, -1, IJ, -1, -1, -1, -1, 0, -1, -1, -1, -1, JK,
    ], // face 14
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, -1, 0, IJ, -1, -1, KI,
    ], // face 15
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, KI, 0, IJ, -1, -1,
    ], // face 16
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, KI, 0, IJ, -1,
    ], // face 17
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, -1, -1, -1, KI, 0, IJ,
    ], // face 18
    [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, JK, IJ, -1, -1, KI, 0,
    ], // face 19
];

#[derive(Copy, Clone, Debug, PartialEq)]
/// Digit representing overage type
pub(crate) enum Overage {
    /// No overage (on original face)
    NO_OVERAGE, //= 0,
    /// On face edge (only occurs on substrate grids)
    FACE_EDGE, //= 1,
    /// Overage on new face interior
    NEW_FACE, //= 2
}

#[derive(PartialEq, Default, Debug)]
pub struct Vec2d {
    /// x component,
    pub x: f64,
    /// y component
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /**
     * Calculates the magnitude of a 2D cartesian vector.
     * @param v The 2D cartesian vector.
     * @return The magnitude of the vector.
     */
    pub fn _v2dMag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
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
    pub fn _v2dIntersect(p0: &Self, p1: &Self, p2: &Self, p3: &Self) -> Self {
        let s1 = Vec2d::new(p1.x - p0.x, p1.y - p0.y);
        let s2 = Vec2d::new(p3.x - p2.x, p3.y - p2.y);

        let t = (s2.x * (p0.y - p2.y) - s2.y * (p0.x - p2.x)) / (-s2.x * s1.y + s1.x * s2.y);

        Self::new(p0.x + (t * s1.x), p0.y + (t * s1.y))
    }

    /* Whether two 2D vectors are equal. Does not consider possible false
     * negatives due to floating-point errors.
     * @param v1 First vector to compare
     * @param v2 Second vector to compare
     * @return Whether the vectors are equal
    bool _v2dEquals(const Vec2d* v1, const Vec2d* v2) {
        return v1->x == v2->x && v1->y == v2->y;
    }
    */

    /**
     * Determines the center point in spherical coordinates of a cell given by 2D
     * hex coordinates on a particular icosahedral face.
     *
     * @param v The 2D hex coordinates of the cell.
     * @param face The icosahedral face upon which the 2D hex coordinate system is
     *             centered.
     * @param res The H3 resolution of the cell.
     * @param substrate Indicates whether or not this grid is actually a substrate
     *        grid relative to the specified resolution.
     * @param g The spherical coordinates of the cell center point.
     */
    pub(crate) fn _hex2dToGeo(
        &self, /* v */
        face: i32,
        res: Resolution,
        substrate: bool,
    ) -> GeoCoord {
        // calculate (r, theta) in hex2d
        let mut r = self._v2dMag();

        if r < crate::constants::EPSILON {
            return faceCenterGeo[face as usize];
        }

        if r < crate::constants::EPSILON {
            return faceCenterGeo[face as usize];
        }

        let mut theta = f64::atan2(self.y, self.x);

        // scale for current resolution length u
        for _ in 0..res as usize {
            r /= constants::M_SQRT7;
        }

        // scale accordingly if this is a substrate grid
        if substrate {
            r /= 3.0;
            if res.isResClassIII() {
                r /= constants::M_SQRT7;
            }
        }

        r *= constants::RES0_U_GNOMONIC;

        // perform inverse gnomonic scaling of r
        r = r.atan();

        // adjust theta for Class III
        // if a substrate grid, then it's already been adjusted for Class III
        if !substrate && res.isResClassIII() {
            theta = _posAngleRads(theta + constants::M_AP7_ROT_RADS);
        }

        // find theta as an azimuth
        theta = _posAngleRads(faceAxesAzRadsCII[face as usize][0] - theta);

        // now find the point at (r,theta) from the face center
        _geoAzDistanceRads(&faceCenterGeo[face as usize], theta, r)
    }

    /**
     * Determine the containing hex in ijk+ coordinates for a 2D cartesian
     * coordinate vector (from DGGRID).
     *
     * @param v The 2D cartesian coordinate vector.
     * @param h The ijk+ coordinates of the containing hex.
     */
    pub(crate) fn _hex2dToCoordIJK(&self) -> CoordIJK {
        let mut i;
        let mut j;

        // quantize into the ij system and then normalize
        let k = 0;

        let a1: f64 = self.x.abs();
        let a2: f64 = self.y.abs();

        // first do a reverse conversion
        let x2 = a2 / M_SIN60;
        let x1 = a1 + x2 / 2.;

        // check if we have the center of a hex
        let m1 = x1 as i32;
        let m2 = x2 as i32;

        // otherwise round correctly
        let r1 = x1 - m1 as f64;
        let r2 = x2 - m2 as f64;

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

                if (1.0 - r1) <= r2 && r2 < (2.0 * r1) {
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

                if (2.0 * r1 - 1.0) < r2 && r2 < (1.0 - r1) {
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
            if (j % 2) == 0
            // even
            {
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

        let mut h = CoordIJK::new(i, j, k);
        h.normalize();

        h
    }
}

impl From<CoordIJK> for Vec2d {
    ///Find the center point in 2D cartesian coordinates of a hex.
    ///
    ///@param h The ijk coordinates of the hex.
    ///@param v The 2D cartesian coordinates of the hex center point.
    fn from(h: CoordIJK) -> Self {
        let i = (h.i - h.k) as f64;
        let j = (h.j - h.k) as f64;

        let x = i - 0.5 * j;
        let y = j * constants::M_SQRT3_2;
        Vec2d { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2dMag() {
        let v = Vec2d::new(3.0, 4.0);
        let expected = 5.0;
        let mag = v._v2dMag();
        assert!(
            (mag - expected).abs() < f64::EPSILON,
            "magnitude as expected"
        );
    }

    #[test]
    fn test_v2dIntersect() {
        let p0 = Vec2d::new(2.0, 2.0);
        let p1 = Vec2d::new(6.0, 6.0);
        let p2 = Vec2d::new(0.0, 4.0);
        let p3 = Vec2d::new(10.0, 4.0);

        let intersection = Vec2d::_v2dIntersect(&p0, &p1, &p2, &p3);

        let expectedX = 4.0;
        let expectedY = 4.0;

        assert!(
            (intersection.x - expectedX).abs() < f64::EPSILON,
            "X coord as expected"
        );
        assert!(
            (intersection.y - expectedY).abs() < f64::EPSILON,
            "Y coord as expected"
        );
    }

    #[test]
    fn test_v2dEquals() {
        let v1 = Vec2d::new(3.0, 4.0);
        let v2 = Vec2d::new(3.0, 4.0);
        let v3 = Vec2d::new(3.5, 4.0);
        let v4 = Vec2d::new(3.0, 4.5);

        assert_eq!(v1, v2, "true for equal vectors");
        assert_ne!(v1, v3, "false for different x");
        assert_ne!(v1, v4, "false for different y");
    }
}

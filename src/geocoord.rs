use crate::{constants::*, faceijk::FaceIJK, vec2d::Vec2d, H3Index, Resolution};

/// epsilon of ~0.1mm in degrees
const EPSILON_DEG: f64 = 0.000000001;
/// epsilon of ~0.1mm in radians
const EPSILON_RAD: f64 = EPSILON_DEG * M_PI_180;

#[derive(Copy, Clone, Debug)]
/// latitude/longitude in radians
pub struct GeoCoord {
    /// latitude in radians
    pub lat: f64,
    /// longitude in radians
    pub lon: f64,
}

/**
 * Normalizes radians to a value between 0.0 and two PI.
 *
 * @param rads The input radians value.
 * @return The normalized radians value.
 */
pub(crate) fn _posAngleRads(rads: f64) -> f64 {
    let tmp = if rads < 0.0 { rads + M_2PI } else { rads };
    if rads >= M_2PI {
        tmp - M_2PI
    } else {
        tmp
    }
}

/**
 * Convert from decimal degrees to radians.
 *
 * @param degrees The decimal degrees.
 * @return The corresponding radians.
 */
pub fn degsToRads(degrees: f64) -> f64 {
    degrees * M_PI_180
}

/**
 * Convert from radians to decimal degrees.
 *
 * @param radians The radians.
 * @return The corresponding decimal degrees.
 */
pub fn radsToDegs(radians: f64) -> f64 {
    radians * M_180_PI
}

impl PartialEq for GeoCoord {
    ///Determines if the components of two spherical coordinates are within our
    ///standard epsilon distance of each other.
    ///
    ///@param p1 The first spherical coordinates.
    ///@param p2 The second spherical coordinates.
    ///@return Whether or not the two coordinates are within the epsilon distance of each other.
    fn eq(&self, other: &Self) -> bool {
        Self::geoAlmostEqualThreshold(self, other, EPSILON_RAD)
    }
}

impl GeoCoord {
    pub(crate) const fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    /**
     * Set the components of spherical coordinates in decimal degrees.
     *
     * @param p The spherical coordinates.
     * @param latDegs The desired latitude in decimal degrees.
     * @param lonDegs The desired longitude in decimal degrees.
     */
    fn setGeoDegs(&mut self, latDegs: f64, lonDegs: f64) {
        self._setGeoRads(degsToRads(latDegs), degsToRads(lonDegs));
    }

    /**
     * Set the components of spherical coordinates in radians.
     *
     * @param p The spherical coordinates.
     * @param latRads The desired latitude in decimal radians.
     * @param lonRads The desired longitude in decimal radians.
     */
    fn _setGeoRads(&mut self, latRads: f64, lonRads: f64) {
        self.lat = latRads;
        self.lon = lonRads;
    }

    /**
     * constrainLat makes sure latitudes are in the proper bounds
     *
     * @param lat The original lat value
     * @return The corrected lat value
     */
    fn constrainLat(mut lat: f64) -> f64 {
        while lat > M_PI_2 {
            lat = lat - M_PI;
        }
        lat
    }

    /**
     * constrainLng makes sure longitudes are in the proper bounds
     *
     * @param lng The origin lng value
     * @return The corrected lng value
     */
    pub(crate) fn constrainLng(mut lng: f64) -> f64 {
        while lng > M_PI {
            lng = lng - (2. * M_PI);
        }
        while lng < -M_PI {
            lng = lng + (2. * M_PI);
        }
        lng
    }

    /**
     * The great circle distance in radians between two spherical coordinates.
     *
     * This function uses the Haversine formula.
     * For math details, see:
     *     https://en.wikipedia.org/wiki/Haversine_formula
     *     https://www.movable-type.co.uk/scripts/latlong.html
     *
     * @param  a  the first lat/lng pair (in radians)
     * @param  b  the second lat/lng pair (in radians)
     *
     * @return    the great circle distance in radians between a and b
     */
    pub fn pointDistRads(a: &Self, b: &Self) -> f64 {
        let sinLat = ((b.lat - a.lat) / 2.0).sin();
        let sinLng = ((b.lon - a.lon) / 2.0).sin();

        let a = sinLat * sinLat + a.lat.cos() * b.lat.cos() * sinLng * sinLng;

        2. * f64::atan2(a.sqrt(), (1. - a).sqrt())
    }

    /**
     * The great circle distance in kilometers between two spherical coordinates.
     */
    pub fn pointDistKm(a: &Self, b: &Self) -> f64 {
        Self::pointDistRads(a, b) * EARTH_RADIUS_KM
    }

    /**
     * The great circle distance in meters between two spherical coordinates.
     */
    pub fn pointDistM(a: &Self, b: &Self) -> f64 {
        Self::pointDistKm(a, b) * 1000.
    }

    /**
     * Determines the azimuth to p2 from p1 in radians.
     *
     * @param p1 The first spherical coordinates.
     * @param p2 The second spherical coordinates.
     * @return The azimuth in radians from p1 to p2.
     */
    fn _geoAzimuthRads(p1: &Self, p2: &Self) -> f64 {
        f64::atan2(
            p2.lat.cos() * (p2.lon - p1.lon).sin(),
            p1.lat.cos() * p2.lat.sin() - p1.lat.sin() * p2.lat.cos() * (p2.lon - p1.lon).cos(),
        )
    }

    /**
     * Surface area in radians^2 of spherical triangle on unit sphere.
     *
     * For the math, see:
     * https://en.wikipedia.org/wiki/Spherical_trigonometry#Area_and_spherical_excess
     *
     * @param   a  length of triangle side A in radians
     * @param   b  length of triangle side B in radians
     * @param   c  length of triangle side C in radians
     *
     * @return     area in radians^2 of triangle on unit sphere
     */
    fn triangleEdgeLengthsToArea(a: f64, b: f64, c: f64) -> f64 {
        let s = (a + b + c) / 2.0;

        let a = (s - a) / 2.0;
        let b = (s - b) / 2.0;
        let c = (s - c) / 2.0;
        let s = s / 2.0;

        4. * f64::atan((s.tan() * a.tan() * b.tan() * c.tan()).sqrt())
    }

    /**
     * Compute area in radians^2 of a spherical triangle, given its vertices.
     *
     * @param   a  vertex lat/lng in radians
     * @param   b  vertex lat/lng in radians
     * @param   c  vertex lat/lng in radians
     *
     * @return     area of triangle on unit sphere, in radians^2
     */
    pub(crate) fn triangleArea(a: &Self, b: &Self, c: &Self) -> f64 {
        Self::triangleEdgeLengthsToArea(
            Self::pointDistRads(a, b),
            Self::pointDistRads(b, c),
            Self::pointDistRads(c, a),
        )
    }

    //Determines if the components of two spherical coordinates are within some
    //threshold distance of each other.
    //
    //@param p1 The first spherical coordinates.
    //@param p2 The second spherical coordinates.
    //@param threshold The threshold distance.
    //@return Whether or not the two coordinates are within the threshold distance
    //        of each other.
    fn geoAlmostEqualThreshold(p1: &Self, p2: &Self, threshold: f64) -> bool {
        (p1.lat - p2.lat).abs() < threshold && (p1.lon - p2.lon).abs() < threshold
    }

    /**
     * Encodes a coordinate on the sphere to the FaceIJK address of the containing
     * cell at the specified resolution.
     *
     * @param g The spherical coordinates to encode.
     * @param res The desired H3 resolution for the encoding.
     * @param h The FaceIJK address of the containing cell at resolution res.
     */
    pub(crate) fn _geoToFaceIjk(&self, res: Resolution) -> FaceIJK {
        /*
        // first convert to hex2d
        let v : Vec2d = self._geoToHex2d(res, &h->face, &v);

        // then convert to ijk+
        _hex2dToCoordIJK(&v, &h->coord);
        */
        todo!()
    }

    /**
     * Encodes a coordinate on the sphere to the corresponding icosahedral face and
     * containing 2D hex coordinates relative to that face center.
     *
     * @param g The spherical coordinates to encode.
     * @param res The desired H3 resolution for the encoding.
     * @param face The icosahedral face containing the spherical coordinates.
     * @param v The 2D hex coordinates of the cell containing the point.
     */
    pub(crate) fn _geoToHex2d(&self, res: Resolution) {
        //pub(crate) fn _geoToHex2d(&self, res:Resolution, int* face, Vec2d* v) {
        /*
        Vec3d v3d;
        _geoToVec3d(g, &v3d);

        // determine the icosahedron face
        *face = 0;
        double sqd = _pointSquareDist(&faceCenterPoint[0], &v3d);
        for (int f = 1; f < NUM_ICOSA_FACES; f++) {
            double sqdT = _pointSquareDist(&faceCenterPoint[f], &v3d);
            if (sqdT < sqd) {
                *face = f;
                sqd = sqdT;
            }
        }

        // cos(r) = 1 - 2 * sin^2(r/2) = 1 - 2 * (sqd / 4) = 1 - sqd/2
        double r = acos(1 - sqd / 2);

        if (r < EPSILON) {
            v->x = v->y = 0.0L;
            return;
        }

        // now have face and r, now find CCW theta from CII i-axis
        double theta =
            _posAngleRads(faceAxesAzRadsCII[*face][0] -
                          _posAngleRads(_geoAzimuthRads(&faceCenterGeo[*face], g)));

        // adjust theta for Class III (odd resolutions)
        if (isResClassIII(res)) theta = _posAngleRads(theta - M_AP7_ROT_RADS);

        // perform gnomonic scaling of r
        r = tan(r);

        // scale for current resolution length u
        r /= RES0_U_GNOMONIC;
        for (int i = 0; i < res; i++) r *= M_SQRT7;

        // we now have (r, theta) in hex2d with theta ccw from x-axes

        // convert to local x,y
        v->x = r * cos(theta);
        v->y = r * sin(theta);
        */
        todo!()
    }

    /**
     * Encodes a coordinate on the sphere to the H3 index of the containing cell at
     * the specified resolution.
     *
     * Returns 0 on invalid input.
     *
     * @param g The spherical coordinates to encode.
     * @param res The desired H3 resolution for the encoding.
     * @return The encoded H3Index (or H3_NULL on failure).
     */
    pub fn geoToH3(&self, res: Resolution) -> H3Index {
        //if (res < 0 || res > MAX_H3_RES) {
        //    return H3_NULL;
        //}

        if !self.lat.is_finite() || !self.lon.is_finite() {
            return H3Index::H3_NULL;
        }

        let fijk = self._geoToFaceIjk(res);
        fijk._faceIjkToH3(res)
    }
}

/**
 * Computes the point on the sphere a specified azimuth and distance from
 * another point.
 *
 * @param p1 The first spherical coordinates.
 * @param az The desired azimuth from p1.
 * @param distance The desired distance from p1, must be non-negative.
 * @param p2 The spherical coordinates at the desired azimuth and distance from
 * p1.
 */
pub fn _geoAzDistanceRads(p1: &GeoCoord, az: f64, distance: f64) -> GeoCoord {
    if distance < EPSILON {
        return *p1;
    }

    let az = _posAngleRads(az);

    let mut lat;
    let lon;
    // check for due north/south azimuth
    if az < EPSILON || (az - M_PI).abs() < EPSILON {
        if az < EPSILON {
            // due north
            lat = p1.lat + distance;
        } else {
            // due south
            lat = p1.lat - distance;
        }

        if lat - M_PI_2.abs() < EPSILON {
            // north pole
            lat = M_PI_2;
            lon = 0.0;
        } else if lat + M_PI_2.abs() < EPSILON {
            // south pole
            lat = -M_PI_2;
            lon = 0.0;
        } else {
            lon = GeoCoord::constrainLng(p1.lon);
        }
    } else {
        // not due north or south
        //double sinlat, sinlon, coslon;
        let mut sinlat = p1.lat.sin() * distance.cos() + p1.lat.cos() * distance.sin() * az.cos();

        if sinlat > 1.0 {
            sinlat = 1.0;
        } else if sinlat < -1.0 {
            sinlat = -1.0;
        }

        lat = sinlat.asin();
        if lat - M_PI_2.abs() < EPSILON {
            // north pole
            lat = M_PI_2;
            lon = 0.0;
        } else if lat + M_PI_2.abs() < EPSILON {
            // south pole
            lat = -M_PI_2;
            lon = 0.0;
        } else {
            let mut sinlon = az.sin() * distance.sin() / lat.cos();
            let mut coslon = (distance.cos() - p1.lat.sin() * lat.sin()) / p1.lat.cos() / lat.cos();

            if sinlon > 1.0 {
                sinlon = 1.0;
            } else if sinlon < -1.0 {
                sinlon = -1.0;
            }

            if coslon > 1.0 {
                coslon = 1.0;
            } else if coslon < -1.0 {
                coslon = -1.0;
            }

            lon = GeoCoord::constrainLng(p1.lon + f64::atan2(sinlon, coslon));
        }
    }

    GeoCoord { lat, lon }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {}
}

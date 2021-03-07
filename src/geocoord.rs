use crate::constants::*;

/// epsilon of ~0.1mm in degrees
const EPSILON_DEG : f64 = 0.000000001;


/// epsilon of ~0.1mm in radians
const EPSILON_RAD : f64 = EPSILON_DEG * M_PI_180;





 /// Normalizes radians to a value between 0.0 and two PI.
fn _posAngleRads(rads: f64) -> f64 {
    let tmp = if rads < 0.0 { rads + M_2PI } else { rads};
    if rads >= M_2PI {
        tmp - M_2PI
    } else {
        tmp
    }
}

pub struct GeoCoord {
    lat : f64,  // latitude in radians
    lon : f64,  // longitude in radians
} 


impl GeoCoord {
    pub const fn new(lat: f64, lon: f64) {
        GeoCoord{lat,lon}
    }

    /// Determines if the components of two spherical coordinates are within some threshold distance of each other.
    pub fn geoAlmostEqualThreshold(&self, other: &Self, threshold: f64) -> bool {
        (self.lat - other.lat) < threshold && (self.lon - other.lon) < threshold
    }


    /// Determines if the components of two spherical coordinates are within our standard epsilon distance of each other.
    pub fn geoAlmostEqual(&self, other: &Self) -> bool {
        self.geoAlmostEqualThreshold(other, EPSILON_RAD)
    }

    /// Set the components of spherical coordinates in decimal degrees.
    pub fn setGeoDegs(&mut self, latDegs:f64, lonDegs:f64) {
        self._setGeoRads( degsToRads(latDegs), degsToRads(lonDegs));
    }


    /// Set the components of spherical coordinates in radians.
    fn _setGeoRads(&mut self, latRads:f64, lonRads:f64) {
        self.lat = latRads;
        self.lon = lonRads;
    }


    /// The great circle distance in radians between two spherical coordinates.
    ///
    /// This function uses the Haversine formula.
    /// For math details, see:
    ///     https://en.wikipedia.org/wiki/Haversine_formula
    ///     https://www.movable-type.co.uk/scripts/latlong.html
    fn pointDistRads(&self, other:&Self)->f64 {
        let sinLat = ((other.lat - self.lat) / 2.0).sin();
        let sinLng = ((other.lon - self.lon) / 2.0).sin();

        let A = sinLat * sinLat + self.lat.cos() * other.lat.cos() * sinLng * sinLng;

        2.0 * f64::atan2(A.sqrt(), (1.0 - A).sqrt())
    }

    /// The great circle distance in kilometers between two spherical coordinates.
    fn pointDistKm(&self, other:&Self) -> f64{
        self.pointDistRads(other) * EARTH_RADIUS_KM
    }

    /// The great circle distance in meters between two spherical coordinates.
    fn pointDistM(&self, other:&Self) -> f64 {
        self.pointDistKm(other) * 1000.
    }

    /// Determines the azimuth to p2 from p1 in radians.
    fn _geoAzimuthRads(&self, other:&Self) -> f64 {
        f64::atan2(other.lat.cos() * (other.lon - self.lon).sin(),
        self.lat.cos() * other.lat.sin() -
        self.lat.sin() * other.lat.cos() * (other.lon - self.lon).cos())
    }

    /// Computes the point on the sphere a specified azimuth and distance from another point.
    fn _geoAzDistanceRads(p1: &GeoCoord, az:f64, distance:f64, p2:&GeoCoord) {
        if distance < EPSILON {
            *p2 = *p1;
            return;
        }


        az = _posAngleRads(az);

        // check for due north/south azimuth
        if az < EPSILON || (az - M_PI).abs() < EPSILON {
            if az < EPSILON {// due north
                p2.lat = p1.lat + distance;
            }
            else {// due south
                p2.lat = p1.lat - distance;
            }

            if (p2.lat - M_PI_2).abs() < EPSILON  // north pole
            {
                p2.lat = M_PI_2;
                p2.lon = 0.0;
            } else if (p2.lat + M_PI_2).abs() < EPSILON  // south pole
            {
                p2.lat = -M_PI_2;
                p2.lon = 0.0;
            } else{
                p2.lon = constrainLng(p1.lon);
            }
        } else  // not due north or south
        {
            //double sinlat, sinlon, coslon;
            let mut sinlat = p1.lat.sin() * distance.cos() + p1.lat.cos() * distance.sin() * az.cos();

            if sinlat > 1.0 {
                sinlat = 1.0;
            } else if sinlat < -1.0 {
                sinlat = -1.0;
            }

            p2.lat = sinlat.asin();
            if (p2.lat - M_PI_2).abs() < EPSILON  // north pole
            {
                p2.lat = M_PI_2;
                p2.lon = 0.0;
            } else if (p2.lat + M_PI_2).abs() < EPSILON  // south pole
            {
                p2.lat = -M_PI_2;
                p2.lon = 0.0;
            } else {
                let mut sinlon = az.sin() * distance.sin() / p2.lat.cos();
                let mut coslon = (distance.cos() - p1.lat.sin()* p2.lat.sin()) / p1.lat.cos() / p2.lat.cos();

                if sinlon > 1.0 {sinlon = 1.0;
                } else
                    if sinlon < -1.0 {sinlon = -1.0;
                    }


                if coslon > 1.0 { coslon = 1.0;
                } else if coslon < -1.0 { coslon = -1.0;
                }

                p2.lon = constrainLng(p1.lon + f64::atan2(sinlon, coslon));
            }
        }
    }





    /// The following functions provide meta information about the H3 hexagons at
    /// each zoom level. Since there are only 16 total levels, these are current
    /// handled with hardwired static values, but it may be worthwhile to put these
    /// static values into another file that can be autogenerated by source code in
    /// the future.
    fn hexAreaKm2(res:i32 )->f64 {
        const AREAS : [f64; 16] = [
            4250546.848, 607220.9782, 86745.85403, 12392.26486,
            1770.323552, 252.9033645, 36.1290521,  5.1612932,
            0.7373276,   0.1053325,   0.0150475,   0.0021496,
            0.0003071,   0.0000439,   0.0000063,   0.0000009];
        AREAS[res]
    }

    fn hexAreaM2(res:i32) -> f64{
        const AREAS : [f64; 16] = [
            4.25055E+12, 6.07221E+11, 86745854035, 12392264862,
            1770323552,  252903364.5, 36129052.1,  5161293.2,
            737327.6,    105332.5,    15047.5,     2149.6,
            307.1,       43.9,        6.3,         0.9];
        areas[res]
    }

    fn edgeLengthKm(res:i32)->f32 {
        const LENS : [f64;16] = [
            1107.712591, 418.6760055, 158.2446558, 59.81085794,
            22.6063794,  8.544408276, 3.229482772, 1.220629759,
            0.461354684, 0.174375668, 0.065907807, 0.024910561,
            0.009415526, 0.003559893, 0.001348575, 0.000509713];
        lens[res]
    }

    fn edgeLengthM(res: i32) -> f64 {
        const LENS : [f64;16] = [
            1107712.591, 418676.0055, 158244.6558, 59810.85794,
            22606.3794,  8544.408276, 3229.482772, 1220.629759,
            461.3546837, 174.3756681, 65.90780749, 24.9105614,
            9.415526211, 3.559893033, 1.348574562, 0.509713273];
        lens[res]
    }

    fn numHexagons(res: i32) -> i64 {
        2 + 120 * 7.pow(res) 
    }

    /// Surface area in radians^2 of spherical triangle on unit sphere.
    ///
    /// For the math, see:
    /// https://en.wikipedia.org/wiki/Spherical_trigonometry#Area_and_spherical_excess
    ///
    /// @param   a  length of triangle side A in radians
    /// @param   b  length of triangle side B in radians
    /// @param   c  length of triangle side C in radians
    ///
    /// @return     area in radians^2 of triangle on unit sphere
    fn triangleEdgeLengthsToArea(a:f64, b:f64, c:f64) -> f64 {
        let mut s = (a + b + c) / 2.;

        let a = (s - a) / 2.;
        let b = (s - b) / 2.;
        let c = (s - c) / 2.;
        s = s / 2.;

        4.0 * atan((s.tan() * a.tan() * b.tan() * c.tan()).sqrt())
    }

    /// Compute area in radians^2 of a spherical triangle, given its vertices.
    fn triangleArea(a: &GeoCoord, b: &GeoCoord, c: &GeoCoord) -> f64 {
        triangleEdgeLengthsToArea(
            a.pointDistRads(b),
            b.pointDistRads(c),
            c.pointDistRads(a))
    }




    /// Calculate the 3D coordinate on unit sphere from the latitude and longitude.
    pub fn _geoToVec3d(&self) -> Vec3d {
        let r = self.lat.cos();

        let z = self.lat.sin();
        let x = self.lon.cos() * r;
        let y = self.lon.sin() * r;

        Vec3d { x, y, z}
    }


    /// Return an estimated number of hexagons that trace the cartesian-projected line
    ///
    /// @param self the origin coordinates
    /// @param destination the destination coordinates
    /// @param res the resolution of the H3 hexagons to trace the line
    /// @return the estimated number of hexagons required to trace the line
    fn lineHexEstimate(&self, destination: &Self, res:i32) -> i32 {
        // Get the area of the pentagon as the maximally-distorted area possible
        let pentagons = getPentagonIndexes(res);
        let pentagonRadiusKm = pentagons[0]._hexRadiusKm();

        let dist = self.pointDistKm(destination);

        let estimate = (dist / (2. * pentagonRadiusKm)).ceil() as i32;
        if estimate == 0 {
            1
        } else {
            estimate
        }
    }




    /// Encodes a coordinate on the sphere to the FaceIJK address of the containing cell at the specified resolution.
    ///
    ///@param g The spherical coordinates to encode.
    ///@param res The desired H3 resolution for the encoding.
    ///@param h The FaceIJK address of the containing cell at resolution res.
    fn _geoToFaceIjk(&self, res:i32) -> FaceIJK{
        // first convert to hex2d
        let v : Vec2d = self._geoToHex2d(res, self.face);

        // then convert to ijk+
        v._hex2dToCoordIJK()
    }

    /// Encodes a coordinate on the sphere to the corresponding icosahedral face and containing 2D hex coordinates relative to that face center.
    ///
    ///@param g The spherical coordinates to encode.
    ///@param res The desired H3 resolution for the encoding.
    ///@param face The icosahedral face containing the spherical coordinates.
    ///@param v The 2D hex coordinates of the cell containing the point.
    fn _geoToHex2d(&self /* g */, res:i32) -> (i32, Vec2d) {
        let v3d = g._geoToVec3d();

        // determine the icosahedron face
        let mut face = 0;
        let mut sqd = faceCenterPoint[0]._pointSquareDist(&v3d);
        for f in (1..NUM_ICOSA_FACES) {
            let sqdT = faceCenterPoint[f]._pointSquareDist(&v3d);
            if sqdT < sqd {
                face = f;
                sqd = sqdT;
            }
        }

        // cos(r) = 1 - 2 * sin^2(r/2) = 1 - 2 * (sqd / 4) = 1 - sqd/2
        let r = (1. - sqd / 2.).acos();

        if r < EPSILON {
            return (face, Vec2d::default());
        }

        // now have face and r, now find CCW theta from CII i-axis
        let mut theta =
            _posAngleRads(faceAxesAzRadsCII[face][0] -
                          _posAngleRads(faceCenterGeo[face]._geoAzimuthRads(&g)));

        // adjust theta for Class III (odd resolutions)
        if (isResClassIII(res)) {
            theta = _posAngleRads(theta - M_AP7_ROT_RADS);
        }

        // perform gnomonic scaling of r
        let mut r = r.tan();

        // scale for current resolution length u
        r /= RES0_U_GNOMONIC;
        for _ in 0..res {
            r *= M_SQRT7;
        }

        // we now have (r, theta) in hex2d with theta ccw from x-axes

        // convert to local x,y
        let v = Vec2d::new( r * theta.cos(), r * theta.sin());

        (face, v)
    }
}




/// Convert from decimal degrees to radians.
fn degsToRads(degrees: f64) -> f64 {degrees * M_PI_180 }

/// Convert from radians to decimal degrees.
fn radsToDegs(radians: f64) -> f64 { radians * M_180_PI }

/// constrainLat makes sure latitudes are in the proper bounds
fn constrainLat(mut lat:f64) -> f64{
    while lat > M_PI_2 {
        lat = lat - M_PI;
    }
    lat
}

/// constrainLng makes sure longitudes are in the proper bounds
fn constrainLng(lng:f64)->f64 {
    while lng > M_PI {
        lng = lng - (2. * M_PI);
    }
    while lng < -M_PI {
        lng = lng + (2. * M_PI);
    }
    lng
}











/*
 * The following functions provide meta information about the H3 hexagons at
 * each zoom level. Since there are only 16 total levels, these are current
 * handled with hardwired static values, but it may be worthwhile to put these
 * static values into another file that can be autogenerated by source code in
 * the future.
 */

fn hexAreaKm2(res:i32)->f64 {
    const AREAS : [f64;16] = [
        4250546.848, 607220.9782, 86745.85403, 12392.26486,
        1770.323552, 252.9033645, 36.1290521,  5.1612932,
        0.7373276,   0.1053325,   0.0150475,   0.0021496,
        0.0003071,   0.0000439,   0.0000063,   0.0000009];
    areas[res]
}

fn hexAreaM2(res:i32) -> f64 {
    const AREAS : [f64;16] = [
        4.25055E+12, 6.07221E+11, 86745854035, 12392264862,
        1770323552,  252903364.5, 36129052.1,  5161293.2,
        737327.6,    105332.5,    15047.5,     2149.6,
        307.1,       43.9,        6.3,         0.9];
    areas[res]
}

fn edgeLengthKm(res:i32)->f64 {
    const LENS : [f64;16] = [
        1107.712591, 418.6760055, 158.2446558, 59.81085794,
        22.6063794,  8.544408276, 3.229482772, 1.220629759,
        0.461354684, 0.174375668, 0.065907807, 0.024910561,
        0.009415526, 0.003559893, 0.001348575, 0.000509713];
    lens[res]
}

fn edgeLengthM(res:i32)->f64 {
    const LENS : [f64;16] = [
        1107712.591, 418676.0055, 158244.6558, 59810.85794,
        22606.3794,  8544.408276, 3229.482772, 1220.629759,
        461.3546837, 174.3756681, 65.90780749, 24.9105614,
        9.415526211, 3.559893033, 1.348574562, 0.509713273];
    lens[res]
}

fn numHexagons(res:i32) {
    2 + 120 * 7.pow(res)
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
fn triangleArea(a:&Self, b:&Self, c:&Self) -> f64 {
    triangleEdgeLengthsToArea(
        a.pointDistRads(b),
        b.pointDistRads(c),
        c.pointDistRads(a)
        )
}


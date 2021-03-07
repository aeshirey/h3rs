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
        static const double areas[] = {
            4250546.848, 607220.9782, 86745.85403, 12392.26486,
            1770.323552, 252.9033645, 36.1290521,  5.1612932,
            0.7373276,   0.1053325,   0.0150475,   0.0021496,
            0.0003071,   0.0000439,   0.0000063,   0.0000009};
        areas[res]
    }

    fn hexAreaM2(res:i32) -> f64{
        static const double areas[] = {
            4.25055E+12, 6.07221E+11, 86745854035, 12392264862,
            1770323552,  252903364.5, 36129052.1,  5161293.2,
            737327.6,    105332.5,    15047.5,     2149.6,
            307.1,       43.9,        6.3,         0.9};
        areas[res]
    }

    fn edgeLengthKm(res:i32)->f32 {
        static const double lens[] = {
            1107.712591, 418.6760055, 158.2446558, 59.81085794,
            22.6063794,  8.544408276, 3.229482772, 1.220629759,
            0.461354684, 0.174375668, 0.065907807, 0.024910561,
            0.009415526, 0.003559893, 0.001348575, 0.000509713};
        lens[res]
    }

    fn edgeLengthM(res: i32) -> f64 {
        static const double lens[] = {
            1107712.591, 418676.0055, 158244.6558, 59810.85794,
            22606.3794,  8544.408276, 3229.482772, 1220.629759,
            461.3546837, 174.3756681, 65.90780749, 24.9105614,
            9.415526211, 3.559893033, 1.348574562, 0.509713273};
        lens[res]
    }

    fn numHexagons(res: i32) { 2 + 120 * _ipow(7, res) -> i64 }

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

    /**
     * Area of H3 cell in radians^2.
     *
     * The area is calculated by breaking the cell into spherical triangles and
     * summing up their areas. Note that some H3 cells (hexagons and pentagons)
     * are irregular, and have more than 6 or 5 sides.
     *
     * todo: optimize the computation by re-using the edges shared between triangles
     *
     * @param   cell  H3 cell
     *
     * @return        cell area in radians^2
     */
    fn cellAreaRads2(cell: H3Index) -> f64{
        let c : GeoCoord = h3ToGeo(cell, &c);
        let gb : GeoBoundary = h3ToGeoBoundary(cell);

        let mut area = 0.0;
        for (int i = 0; i < gb.numVerts; i++) {
            int j = (i + 1) % gb.numVerts;
            area += triangleArea(&gb.verts[i], &gb.verts[j], &c);
        }

        area
    }

    /// Area of H3 cell in kilometers^2.
    fn cellAreaKm2(h: H3Index) -> f64{
        cellAreaRads2(h) * EARTH_RADIUS_KM * EARTH_RADIUS_KM
    }

    /// Area of H3 cell in meters^2.
    fn cellAreaM2(h:H3Index)->f64 {
        cellAreaKm2(h) * 1000.0 * 1000.0
    }

    /// Length of a unidirectional edge in radians.
    fn exactEdgeLengthRads(edge:H3Index) -> f64 {
        let gb: GeoBoundary = getH3UnidirectionalEdgeBoundary(edge);
        //let gb: GeoBoundary = getH3UnidirectionalEdgeBoundary(edge, &gb);

        let mut length = 0.0;
        for (int i = 0; i < gb.numVerts - 1; i++) {
            length += pointDistRads(&gb.verts[i], &gb.verts[i + 1]);
        }

        length
    }

    /// Length of a unidirectional edge in kilometers.
    fn exactEdgeLengthKm(edge:H3Index) -> f64 {
        exactEdgeLengthRads(edge) * EARTH_RADIUS_KM
    }

    /// Length of a unidirectional edge in meters.
    fn exactEdgeLengthM(edge: H3Index) -> f64 {
        exactEdgeLengthKm(edge) * 1000.0
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
        H3Index pentagons[12] = {0};
        getPentagonIndexes(res, pentagons);
        double pentagonRadiusKm = _hexRadiusKm(pentagons[0]);

        double dist = pointDistKm(self, destination);
        int estimate = (int)ceil(dist / (2 * pentagonRadiusKm));
        if (estimate == 0) estimate = 1;
        return estimate;
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

double H3_EXPORT(hexAreaKm2)(int res) {
    static const double areas[] = {
        4250546.848, 607220.9782, 86745.85403, 12392.26486,
        1770.323552, 252.9033645, 36.1290521,  5.1612932,
        0.7373276,   0.1053325,   0.0150475,   0.0021496,
        0.0003071,   0.0000439,   0.0000063,   0.0000009};
    return areas[res];
}

double H3_EXPORT(hexAreaM2)(int res) {
    static const double areas[] = {
        4.25055E+12, 6.07221E+11, 86745854035, 12392264862,
        1770323552,  252903364.5, 36129052.1,  5161293.2,
        737327.6,    105332.5,    15047.5,     2149.6,
        307.1,       43.9,        6.3,         0.9};
    return areas[res];
}

double H3_EXPORT(edgeLengthKm)(int res) {
    static const double lens[] = {
        1107.712591, 418.6760055, 158.2446558, 59.81085794,
        22.6063794,  8.544408276, 3.229482772, 1.220629759,
        0.461354684, 0.174375668, 0.065907807, 0.024910561,
        0.009415526, 0.003559893, 0.001348575, 0.000509713};
    return lens[res];
}

double H3_EXPORT(edgeLengthM)(int res) {
    static const double lens[] = {
        1107712.591, 418676.0055, 158244.6558, 59810.85794,
        22606.3794,  8544.408276, 3229.482772, 1220.629759,
        461.3546837, 174.3756681, 65.90780749, 24.9105614,
        9.415526211, 3.559893033, 1.348574562, 0.509713273};
    return lens[res];
}

int64_t H3_EXPORT(numHexagons)(int res) { return 2 + 120 * _ipow(7, res); }


/**
 * Compute area in radians^2 of a spherical triangle, given its vertices.
 *
 * @param   a  vertex lat/lng in radians
 * @param   b  vertex lat/lng in radians
 * @param   c  vertex lat/lng in radians
 *
 * @return     area of triangle on unit sphere, in radians^2
 */
double triangleArea(const GeoCoord *a, const GeoCoord *b, const GeoCoord *c) {
    return triangleEdgeLengthsToArea(H3_EXPORT(pointDistRads)(a, b),
    H3_EXPORT(pointDistRads)(b, c),
    H3_EXPORT(pointDistRads)(c, a));
}

/**
 * Area of H3 cell in radians^2.
 *
 * The area is calculated by breaking the cell into spherical triangles and
 * summing up their areas. Note that some H3 cells (hexagons and pentagons)
 * are irregular, and have more than 6 or 5 sides.
 *
 * todo: optimize the computation by re-using the edges shared between triangles
 *
 * @param   cell  H3 cell
 *
 * @return        cell area in radians^2
 */
double H3_EXPORT(cellAreaRads2)(H3Index cell) {
    GeoCoord c;
    GeoBoundary gb;
    H3_EXPORT(h3ToGeo)(cell, &c);
    H3_EXPORT(h3ToGeoBoundary)(cell, &gb);

    double area = 0.0;
    for (int i = 0; i < gb.numVerts; i++) {
        int j = (i + 1) % gb.numVerts;
        area += triangleArea(&gb.verts[i], &gb.verts[j], &c);
    }

    return area;
}

/**
 * Area of H3 cell in kilometers^2.
 */
double H3_EXPORT(cellAreaKm2)(H3Index h) {
    return H3_EXPORT(cellAreaRads2)(h) * EARTH_RADIUS_KM * EARTH_RADIUS_KM;
}

/**
 * Area of H3 cell in meters^2.
 */
double H3_EXPORT(cellAreaM2)(H3Index h) {
    return H3_EXPORT(cellAreaKm2)(h) * 1000 * 1000;
}

/**
 * Length of a unidirectional edge in radians.
 *
 * @param   edge  H3 unidirectional edge
 *
 * @return        length in radians
 */
double H3_EXPORT(exactEdgeLengthRads)(H3Index edge) {
    GeoBoundary gb;

    H3_EXPORT(getH3UnidirectionalEdgeBoundary)(edge, &gb);

    double length = 0.0;
    for (int i = 0; i < gb.numVerts - 1; i++) {
        length += H3_EXPORT(pointDistRads)(&gb.verts[i], &gb.verts[i + 1]);
    }

    return length;
}

/**
 * Length of a unidirectional edge in kilometers.
 */
double H3_EXPORT(exactEdgeLengthKm)(H3Index edge) {
    return H3_EXPORT(exactEdgeLengthRads)(edge) * EARTH_RADIUS_KM;
}

/**
 * Length of a unidirectional edge in meters.
 */
double H3_EXPORT(exactEdgeLengthM)(H3Index edge) {
    return H3_EXPORT(exactEdgeLengthKm)(edge) * 1000;
}

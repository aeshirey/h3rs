#[derive(PartialEq, Eq)]
/// Geographic bounding box with coordinates defined in radians
pub struct BBox {
    north:f64,  ///< north latitude
    south:f64,  ///< south latitude
    east:f64,   ///< east longitude
    west:f64,   ///< west longitude
} 

impl BBox {
    /// Whether the given bounding box crosses the antimeridian
    pub fn bboxIsTransmeridian(&self) {
        self.east < self.west
    }


/// Get the center of a bounding box
pub fn bboxCenter(const BBox* bbox, GeoCoord* center) {
        center->lat = (bbox->north + bbox->south) / 2.0;
            // If the bbox crosses the antimeridian, shift east 360 degrees
            double east = bboxIsTransmeridian(bbox) ? bbox->east + M_2PI : bbox->east;
            center->lon = constrainLng((east + bbox->west) / 2.0);
            }
            
}

/// Whether the bounding box contains a given point
fn bboxContains(self , const GeoCoord* point) -> bool {
    if point.lat >= self.south && point.lat <= self.north && self.bboxIsTransmeridian() {
        // transmeridian case
        (point.lon >= self.west || point.lon <= self.east)
    } else {
        // standard case
        (point.lon >= self.west && point.lon <= self.east));
    }
}




/// _hexRadiusKm returns the radius of a given hexagon in Km
fn _hexRadiusKm(H3Index h3Index) -> f64 {
    // There is probably a cheaper way to determine the radius of a
    // hexagon, but this way is conceptually simple
    GeoCoord h3Center;
    GeoBoundary h3Boundary;
    h3ToGeo(h3Index, &h3Center);
    h3ToGeoBoundary(h3Index, &h3Boundary);
    return pointDistKm(&h3Center, h3Boundary.verts);
}


/// returns an estimated number of hexagons that fit within the cartesian-projected bounding box
fn bboxHexEstimate(const BBox* bbox, int res) -> i32 {
    // Get the area of the pentagon as the maximally-distorted area possible
    H3Index pentagons[12] = {0};
    getPentagonIndexes(res, pentagons);
    double pentagonRadiusKm = _hexRadiusKm(pentagons[0]);
    // Area of a regular hexagon is 3/2*sqrt(3) * r * r
    // The pentagon has the most distortion (smallest edges) and shares its
    // edges with hexagons, so the most-distorted hexagons have this area,
    // shrunk by 20% off chance that the bounding box perfectly bounds a
    // pentagon.
    double pentagonAreaKm2 =
        0.8 * (2.59807621135 * pentagonRadiusKm * pentagonRadiusKm);

    // Then get the area of the bounding box of the geofence in question
    GeoCoord p1, p2;
    p1.lat = bbox->north;
    p1.lon = bbox->east;
    p2.lat = bbox->south;
    p2.lon = bbox->west;
    double d = pointDistKm(&p1, &p2);
    // Derived constant based on: https://math.stackexchange.com/a/1921940
    // Clamped to 3 as higher values tend to rapidly drag the estimate to zero.
    double a = d * d / fmin(3.0, fabs((p1.lon - p2.lon) / (p1.lat - p2.lat)));

    // Divide the two to get an estimate of the number of hexagons needed
    int estimate = (int)ceil(a / pentagonAreaKm2);
    if (estimate == 0) estimate = 1;
    return estimate;
}



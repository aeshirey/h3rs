#[derive(PartialEq, Eq)]
/// Geographic bounding box with coordinates defined in radians
pub struct BBox {
    /// north latitude
    north: f64,

    /// south latitude
    south: f64,

    /// east longitude
    east: f64,

    /// west longitude
    west: f64,
}

impl BBox {
    /// Whether the given bounding box crosses the antimeridian
    pub fn bboxIsTransmeridian(&self) {
        self.east < self.west
    }

    /// Get the center of a bounding box
    pub fn bboxCenter(&self) -> GeoCoord {
        let lat = (self.north + self.south) / 2.0;
        // If the bbox crosses the antimeridian, shift east 360 degrees
        let east: f64 = if self.bboxIsTransmeridian() {
            self.east + M_2PI
        } else {
            self.east
        };
        let lon = GeoCoord::constrainLng((east + self.west) / 2.0);

        GeoCoord { lat, lon }
    }

    /// Whether the bounding box contains a given point
    fn bboxContains(&self, point: &GeoCoord) -> bool {
        if point.lat >= self.south && point.lat <= self.north && self.bboxIsTransmeridian() {
            // transmeridian case
            (point.lon >= self.west || point.lon <= self.east)
        } else {
            // standard case
            (point.lon >= self.west && point.lon <= self.east)
        }
    }

    /// returns an estimated number of hexagons that fit within the cartesian-projected bounding box
    fn bboxHexEstimate(&self /*bbox*/, res: i32) -> i32 {
        // Get the area of the pentagon as the maximally-distorted area possible
        let pentagons = getPentagonIndexes(res);
        let pentagonRadiusKm: f64 = pentagons[0]._hexRadiusKm();

        // Area of a regular hexagon is 3/2*sqrt(3) * r * r
        // The pentagon has the most distortion (smallest edges) and shares its
        // edges with hexagons, so the most-distorted hexagons have this area,
        // shrunk by 20% off chance that the bounding box perfectly bounds a
        // pentagon.
        let pentagonAreaKm2: f64 = 0.8 * (2.59807621135 * pentagonRadiusKm * pentagonRadiusKm);

        // Then get the area of the bounding box of the geofence in question
        let p1 = GeoCoord::new(self.north, self.east);
        let p2 = GeoCoord::new(self.south, self.west);
        let d = p1.pointDistKm(&p2);

        // Derived constant based on: https://math.stackexchange.com/a/1921940
        // Clamped to 3 as higher values tend to rapidly drag the estimate to zero.
        //double a = d * d / fmin(3.0, fabs((p1.lon - p2.lon) / (p1.lat - p2.lat)));
        let fabs = ((p1.lon - p2.lon) / (p1.lat - p2.lat)).abs();
        let a = d * d / fabs.min(3.0);

        // Divide the two to get an estimate of the number of hexagons needed
        let estimate = ceil(a / pentagonAreaKm2).ceil() as i32;
        if (estimate == 0) {
            1
        } else {
            estimate
        }
    }
}

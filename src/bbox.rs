use crate::{
    constants::{M_2PI, M_PI},
    resolution::Resolution,
    GeoCoord,
};

#[derive(Copy, Clone, PartialEq, Debug)]
/// Geographic bounding box with coordinates defined in radians
pub struct BBox {
    /// north latitude
    pub north: f64,

    /// south latitude
    pub south: f64,

    /// east longitude
    pub east: f64,

    /// west longitude
    pub west: f64,
}

impl BBox {
    pub const fn new(north: f64, south: f64, east: f64, west: f64) -> Self {
        Self {
            north,
            south,
            east,
            west,
        }
    }

    /// Whether the given bounding box crosses the antimeridian
    pub fn bboxIsTransmeridian(&self) -> bool {
        self.east < self.west
    }

    /// Get the center of a bounding box
    pub(crate) fn bboxCenter(&self) -> GeoCoord {
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
    pub(crate) fn bboxContains(&self, point: &GeoCoord) -> bool {
        if point.lat >= self.south && point.lat <= self.north && self.bboxIsTransmeridian() {
            // transmeridian case
            point.lon >= self.west || point.lon <= self.east
        } else {
            // standard case
            point.lon >= self.west && point.lon <= self.east
        }
    }

    /// returns an estimated number of hexagons that fit within the cartesian-projected bounding box
    fn bboxHexEstimate(&self /*bbox*/, res: Resolution) -> i32 {
        todo!()
        /*

           int bboxHexEstimate(const BBox* bbox, int res) {
        // Get the area of the pentagon as the maximally-distorted area possible
        H3Index pentagons[12] = {0};
        H3_EXPORT(getPentagonIndexes)(res, pentagons);
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
        double d = H3_EXPORT(pointDistKm)(&p1, &p2);
        // Derived constant based on: https://math.stackexchange.com/a/1921940
        // Clamped to 3 as higher values tend to rapidly drag the estimate to zero.
        double a = d * d / fmin(3.0, fabs((p1.lon - p2.lon) / (p1.lat - p2.lat)));

        // Divide the two to get an estimate of the number of hexagons needed
        int estimate = (int)ceil(a / pentagonAreaKm2);
        if (estimate == 0) estimate = 1;
        return estimate;
        }
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    void assertBBox(const Geofence* geofence, const BBox* expected, const GeoCoord* inside, const GeoCoord* outside) {
        BBox result;

        bboxFromGeofence(geofence, &result);

        t_assert(bboxEquals(&result, expected), "Got expected bbox");
        t_assert(bboxContains(&result, inside), "Contains expected inside point");
        t_assert(!bboxContains(&result, outside),
                 "Does not contain expected outside point");
    }
    */

    /*
    TEST(posLatPosLon) {
        GeoCoord verts[] = {{0.8, 0.3}, {0.7, 0.6}, {1.1, 0.7}, {1.0, 0.2}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {1.1, 0.7, 0.7, 0.2};
        const GeoCoord inside = {0.9, 0.4};
        const GeoCoord outside = {0.0, 0.0};
        assertBBox(&geofence, &expected, &inside, &outside);
    }

    TEST(negLatPosLon) {
        GeoCoord verts[] = {{-0.3, 0.6}, {-0.4, 0.9}, {-0.2, 0.8}, {-0.1, 0.6}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {-0.1, -0.4, 0.9, 0.6};
        const GeoCoord inside = {-0.3, 0.8};
        const GeoCoord outside = {0.0, 0.0};
        assertBBox(&geofence, &expected, &inside, &outside);
    }

    TEST(posLatNegLon) {
        GeoCoord verts[] = {{0.7, -1.4}, {0.8, -0.9}, {1.0, -0.8}, {1.1, -1.3}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {1.1, 0.7, -0.8, -1.4};
        const GeoCoord inside = {0.9, -1.0};
        const GeoCoord outside = {0.0, 0.0};
        assertBBox(&geofence, &expected, &inside, &outside);
    }

    TEST(negLatNegLon) {
        GeoCoord verts[] = {
            {-0.4, -1.4}, {-0.3, -1.1}, {-0.1, -1.2}, {-0.2, -1.4}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {-0.1, -0.4, -1.1, -1.4};
        const GeoCoord inside = {-0.3, -1.2};
        const GeoCoord outside = {0.0, 0.0};
        assertBBox(&geofence, &expected, &inside, &outside);
    }

    TEST(aroundZeroZero) {
        GeoCoord verts[] = {{0.4, -0.4}, {0.4, 0.4}, {-0.4, 0.4}, {-0.4, -0.4}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {0.4, -0.4, 0.4, -0.4};
        const GeoCoord inside = {-0.1, -0.1};
        const GeoCoord outside = {1.0, -1.0};
        assertBBox(&geofence, &expected, &inside, &outside);
    }
    */

    #[test]
    fn test_transmeridian() {
        let verts: [GeoCoord; 4] = [
            GeoCoord::new(0.4, M_PI - 0.1),
            GeoCoord::new(0.4, -M_PI + 0.1),
            GeoCoord::new(-0.4, -M_PI + 0.1),
            GeoCoord::new(-0.4, M_PI - 0.),
        ];
        //const Geofence geofence = {.numVerts = 4, .verts = verts};
        let expected = BBox::new(0.4, -0.4, -M_PI + 0.1, M_PI - 0.1);

        let insideOnMeridian = GeoCoord::new(-0.1, M_PI);
        let outside = GeoCoord::new(1.0, M_PI - 0.5);
        //assertBBox(&geofence, &expected, &insideOnMeridian, &outside);

        let westInside = GeoCoord::new(0.1, M_PI - 0.05);
        assert!(
            expected.bboxContains(&westInside),
            "Contains expected west inside point"
        );
        let eastInside = GeoCoord::new(0.1, -M_PI + 0.05);
        assert!(
            expected.bboxContains(&eastInside),
            "Contains expected east outside point"
        );

        let westOutside = GeoCoord::new(0.1, M_PI - 0.5);
        assert!(
            !expected.bboxContains(&westOutside),
            "Does not contain expected west outside point"
        );
        let eastOutside = GeoCoord::new(0.1, -M_PI + 0.5);
        assert!(
            !expected.bboxContains(&eastOutside),
            "Does not contain expected east outside point"
        );
    }

    /*
    TEST(edgeOnNorthPole) {
        GeoCoord verts[] = {{M_PI_2 - 0.1, 0.1},
                            {M_PI_2 - 0.1, 0.8},
                            {M_PI_2, 0.8},
                            {M_PI_2, 0.1}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {M_PI_2, M_PI_2 - 0.1, 0.8, 0.1};
        const GeoCoord inside = {M_PI_2 - 0.01, 0.4};
        const GeoCoord outside = {M_PI_2, 0.9};
        assertBBox(&geofence, &expected, &inside, &outside);
    }

    TEST(edgeOnSouthPole) {
        GeoCoord verts[] = {{-M_PI_2 + 0.1, 0.1},
                            {-M_PI_2 + 0.1, 0.8},
                            {-M_PI_2, 0.8},
                            {-M_PI_2, 0.1}};
        const Geofence geofence = {.numVerts = 4, .verts = verts};
        const BBox expected = {-M_PI_2 + 0.1, -M_PI_2, 0.8, 0.1};
        const GeoCoord inside = {-M_PI_2 + 0.01, 0.4};
        const GeoCoord outside = {-M_PI_2, 0.9};
        assertBBox(&geofence, &expected, &inside, &outside);
    }
    */

    #[test]
    fn test_containsEdges() {
        let bbox = BBox::new(0.1, -0.1, 0.2, -0.2);
        let points = vec![
            GeoCoord::new(0.1, 0.2),
            GeoCoord::new(0.1, 0.0),
            GeoCoord::new(0.1, -0.2),
            GeoCoord::new(0.0, 0.2),
            GeoCoord::new(-0.1, 0.2),
            GeoCoord::new(-0.1, 0.0),
            GeoCoord::new(-0.1, -0.2),
            GeoCoord::new(0.0, -0.2),
        ];

        for p in points {
            assert!(bbox.bboxContains(&p), "Contains edge point");
        }
    }

    #[test]
    fn test_containsEdgesTransmeridian() {
        let bbox = BBox::new(0.1, -0.1, -M_PI + 0.2, M_PI - 0.2);
        let points = vec![
            GeoCoord::new(0.1, -M_PI + 0.2),
            GeoCoord::new(0.1, M_PI),
            GeoCoord::new(0.1, M_PI - 0.2),
            GeoCoord::new(0.0, -M_PI + 0.2),
            GeoCoord::new(-0.1, -M_PI + 0.2),
            GeoCoord::new(-0.1, M_PI),
            GeoCoord::new(-0.1, M_PI - 0.2),
            GeoCoord::new(0.0, M_PI - 0.2),
        ];

        for p in points {
            assert!(bbox.bboxContains(&p), "Contains transmeridian edge point");
        }
    }

    #[test]
    fn test_bboxCenterBasicQuandrants() {
        let bbox1 = BBox::new(1.0, 0.8, 1.0, 0.8);
        let expected1 = GeoCoord::new(0.9, 0.9);
        let center = bbox1.bboxCenter();
        assert_eq!(center, expected1, "pos/pos as expected");

        let bbox2 = BBox::new(-0.8, -1.0, 1.0, 0.8);
        let expected2 = GeoCoord::new(-0.9, 0.9);
        let center = bbox2.bboxCenter();
        assert_eq!(center, expected2, "neg/pos as expected");

        let bbox3 = BBox::new(1.0, 0.8, -0.8, -1.0);
        let expected3 = GeoCoord::new(0.9, -0.9);
        let center = bbox3.bboxCenter();
        assert_eq!(center, expected3, "pos/neg as expected");

        let bbox4 = BBox::new(-0.8, -1.0, -0.8, -1.0);
        let expected4 = GeoCoord::new(-0.9, -0.9);
        let center = bbox4.bboxCenter();
        assert_eq!(center, expected4, "neg/neg as expected");

        let bbox5 = BBox::new(0.8, -0.8, 1.0, -1.0);
        let expected5 = GeoCoord::new(0.0, 0.0);
        let center = bbox5.bboxCenter();
        assert_eq!(center, expected5, "around origin as expected");
    }

    #[test]
    fn test_bboxCenterTransmeridian() {
        let bbox1 = BBox::new(1.0, 0.8, -M_PI + 0.3, M_PI - 0.1);
        let expected1 = GeoCoord::new(0.9, -M_PI + 0.1);
        let center = bbox1.bboxCenter();
        assert_eq!(center, expected1, "skew east as expected");

        let bbox2 = BBox::new(1.0, 0.8, -M_PI + 0.1, M_PI - 0.3);
        let expected2 = GeoCoord::new(0.9, M_PI - 0.1);
        let center = bbox2.bboxCenter();
        assert_eq!(center, expected2, "skew west as expected");

        let bbox3 = BBox::new(1.0, 0.8, -M_PI + 0.1, M_PI - 0.1);
        let expected3 = GeoCoord::new(0.9, M_PI);
        let center = bbox3.bboxCenter();
        assert_eq!(center, expected3, "on antimeridian as expected");
    }

    #[test]
    fn test_bboxIsTransmeridian() {
        let bboxNormal = BBox::new(1.0, 0.8, 1.0, 0.8);
        assert!(
            !bboxNormal.bboxIsTransmeridian(),
            "Normal bbox not transmeridian"
        );

        let bboxTransmeridian = BBox::new(1.0, 0.8, -M_PI + 0.3, M_PI - 0.1);
        assert!(
            bboxTransmeridian.bboxIsTransmeridian(),
            "Transmeridian bbox is transmeridian"
        );
    }

    #[test]
    fn test_bboxEquals() {
        let bbox = BBox::new(1.0, 0.0, 1.0, 0.0);
        let mut north = bbox;
        north.north += 0.1;
        let mut south = bbox;
        south.south += 0.1;
        let mut east = bbox;
        east.east += 0.1;
        let mut west = bbox;
        west.west += 0.1;

        assert_eq!(bbox, bbox, "Equals self");
        assert_ne!(bbox, north, "Not equals different north");
        assert_ne!(bbox, south, "Not equals different south");
        assert_ne!(bbox, east, "Not equals different east");
        assert_ne!(bbox, west, "Not equals different west");
    }
}

use crate::resolution::Resolution;

#[derive(Copy, Clone, PartialEq)]
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
    /// Whether the given bounding box crosses the antimeridian
    pub fn bboxIsTransmeridian(&self) -> bool {
        self.east < self.west
    }

    /*
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
            point.lon >= self.west || point.lon <= self.east
        } else {
            // standard case
            point.lon >= self.west && point.lon <= self.east
        }
    }
    */

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
    SUITE(BBox) {
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

        TEST(transmeridian) {
            GeoCoord verts[] = {{0.4, M_PI - 0.1},
                                {0.4, -M_PI + 0.1},
                                {-0.4, -M_PI + 0.1},
                                {-0.4, M_PI - 0.1}};
            const Geofence geofence = {.numVerts = 4, .verts = verts};
            const BBox expected = {0.4, -0.4, -M_PI + 0.1, M_PI - 0.1};
            const GeoCoord insideOnMeridian = {-0.1, M_PI};
            const GeoCoord outside = {1.0, M_PI - 0.5};
            assertBBox(&geofence, &expected, &insideOnMeridian, &outside);

            const GeoCoord westInside = {0.1, M_PI - 0.05};
            t_assert(bboxContains(&expected, &westInside),
                     "Contains expected west inside point");
            const GeoCoord eastInside = {0.1, -M_PI + 0.05};
            t_assert(bboxContains(&expected, &eastInside),
                     "Contains expected east outside point");

            const GeoCoord westOutside = {0.1, M_PI - 0.5};
            t_assert(!bboxContains(&expected, &westOutside),
                     "Does not contain expected west outside point");
            const GeoCoord eastOutside = {0.1, -M_PI + 0.5};
            t_assert(!bboxContains(&expected, &eastOutside),
                     "Does not contain expected east outside point");
        }

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

        TEST(containsEdges) {
            const BBox bbox = {0.1, -0.1, 0.2, -0.2};
            GeoCoord points[] = {
                {0.1, 0.2},  {0.1, 0.0},  {0.1, -0.2},  {0.0, 0.2},
                {-0.1, 0.2}, {-0.1, 0.0}, {-0.1, -0.2}, {0.0, -0.2},
            };
            const int numPoints = 8;

            for (int i = 0; i < numPoints; i++) {
                t_assert(bboxContains(&bbox, &points[i]), "Contains edge point");
            }
        }

        TEST(containsEdgesTransmeridian) {
            const BBox bbox = {0.1, -0.1, -M_PI + 0.2, M_PI - 0.2};
            GeoCoord points[] = {
                {0.1, -M_PI + 0.2}, {0.1, M_PI},         {0.1, M_PI - 0.2},
                {0.0, -M_PI + 0.2}, {-0.1, -M_PI + 0.2}, {-0.1, M_PI},
                {-0.1, M_PI - 0.2}, {0.0, M_PI - 0.2},
            };
            const int numPoints = 8;

            for (int i = 0; i < numPoints; i++) {
                t_assert(bboxContains(&bbox, &points[i]),
                         "Contains transmeridian edge point");
            }
        }

        TEST(bboxCenterBasicQuandrants) {
            GeoCoord center;

            BBox bbox1 = {1.0, 0.8, 1.0, 0.8};
            GeoCoord expected1 = {0.9, 0.9};
            bboxCenter(&bbox1, &center);
            t_assert(geoAlmostEqual(&center, &expected1), "pos/pos as expected");

            BBox bbox2 = {-0.8, -1.0, 1.0, 0.8};
            GeoCoord expected2 = {-0.9, 0.9};
            bboxCenter(&bbox2, &center);
            t_assert(geoAlmostEqual(&center, &expected2), "neg/pos as expected");

            BBox bbox3 = {1.0, 0.8, -0.8, -1.0};
            GeoCoord expected3 = {0.9, -0.9};
            bboxCenter(&bbox3, &center);
            t_assert(geoAlmostEqual(&center, &expected3), "pos/neg as expected");

            BBox bbox4 = {-0.8, -1.0, -0.8, -1.0};
            GeoCoord expected4 = {-0.9, -0.9};
            bboxCenter(&bbox4, &center);
            t_assert(geoAlmostEqual(&center, &expected4), "neg/neg as expected");

            BBox bbox5 = {0.8, -0.8, 1.0, -1.0};
            GeoCoord expected5 = {0.0, 0.0};
            bboxCenter(&bbox5, &center);
            t_assert(geoAlmostEqual(&center, &expected5),
                     "around origin as expected");
        }

        TEST(bboxCenterTransmeridian) {
            GeoCoord center;

            BBox bbox1 = {1.0, 0.8, -M_PI + 0.3, M_PI - 0.1};
            GeoCoord expected1 = {0.9, -M_PI + 0.1};
            bboxCenter(&bbox1, &center);

            t_assert(geoAlmostEqual(&center, &expected1), "skew east as expected");

            BBox bbox2 = {1.0, 0.8, -M_PI + 0.1, M_PI - 0.3};
            GeoCoord expected2 = {0.9, M_PI - 0.1};
            bboxCenter(&bbox2, &center);
            t_assert(geoAlmostEqual(&center, &expected2), "skew west as expected");

            BBox bbox3 = {1.0, 0.8, -M_PI + 0.1, M_PI - 0.1};
            GeoCoord expected3 = {0.9, M_PI};
            bboxCenter(&bbox3, &center);
            t_assert(geoAlmostEqual(&center, &expected3),
                     "on antimeridian as expected");
        }

        TEST(bboxIsTransmeridian) {
            BBox bboxNormal = {1.0, 0.8, 1.0, 0.8};
            t_assert(!bboxIsTransmeridian(&bboxNormal),
                     "Normal bbox not transmeridian");

            BBox bboxTransmeridian = {1.0, 0.8, -M_PI + 0.3, M_PI - 0.1};
            t_assert(bboxIsTransmeridian(&bboxTransmeridian),
                     "Transmeridian bbox is transmeridian");
        }

        TEST(bboxEquals) {
            BBox bbox = {1.0, 0.0, 1.0, 0.0};
            BBox north = bbox;
            north.north += 0.1;
            BBox south = bbox;
            south.south += 0.1;
            BBox east = bbox;
            east.east += 0.1;
            BBox west = bbox;
            west.west += 0.1;

            t_assert(bboxEquals(&bbox, &bbox), "Equals self");
            t_assert(!bboxEquals(&bbox, &north), "Not equals different north");
            t_assert(!bboxEquals(&bbox, &south), "Not equals different south");
            t_assert(!bboxEquals(&bbox, &east), "Not equals different east");
            t_assert(!bboxEquals(&bbox, &west), "Not equals different west");
        }
        */
}
use crate::{
    constants::{EARTH_RADIUS_KM, NUM_HEX_VERTS, NUM_PENT_VERTS},
    faceijk::FaceIJK,
    geopolygon::GeoBoundary,
    GeoCoord, H3Index, Resolution,
};

impl H3Index {
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
    pub fn cellAreaRads2(&self) -> f64 {
        let c: GeoCoord = self.h3ToGeo();
        let gb: GeoBoundary = self.h3ToGeoBoundary();

        let mut area = 0.0;
        for i in 0..gb.numVerts {
            let j = (i + 1) % gb.numVerts;
            area += GeoCoord::triangleArea(&gb.verts[i], &gb.verts[j], &c);
        }

        area
    }

    /// Area of H3 cell in kilometers^2.
    pub fn cellAreaKm2(&self) -> f64 {
        self.cellAreaRads2() * EARTH_RADIUS_KM * EARTH_RADIUS_KM
    }

    /// Area of H3 cell in meters^2.
    pub fn cellAreaM2(&self) -> f64 {
        self.cellAreaKm2() * 1000. * 1000.
    }

    /**
     * Length of a unidirectional edge in radians.
     *
     * @param   edge  H3 unidirectional edge
     *
     * @return        length in radians
     */
    pub fn exactEdgeLengthRads(&self) -> f64 {
        let gb: GeoBoundary = self.getH3UnidirectionalEdgeBoundary();

        let mut length = 0.0;
        for i in 0..gb.numVerts - 1 {
            //length += H3_EXPORT(pointDistRads)(&gb.verts[i], &gb.verts[i + 1]);
            todo!()
        }

        length
    }

    /*
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

    */

    /**
     * Convert an H3Index to a FaceIJK address.
     * @param h The H3Index.
     * @param fijk The corresponding FaceIJK address.
     */
    pub(crate) fn _h3ToFaceIjk(&self /* h */) -> FaceIJK {
        let baseCell = self.get_base_cell();
        todo!()
        /*
        if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {  // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            // TODO: Indicate an error to the caller
            // To prevent reading uninitialized memory, we zero the output.
            fijk->face = 0;
            fijk->coord.i = fijk->coord.j = fijk->coord.k = 0;
            return;
        }
        // adjust for the pentagonal missing sequence; all of sub-sequence 5 needs
        // to be adjusted (and some of sub-sequence 4 below)
        if (_isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) == 5)
            h = _h3Rotate60cw(h);

        // start with the "home" face and ijk+ coordinates for the base cell of c
        *fijk = baseCellData[baseCell].homeFijk;
        if (!_h3ToFaceIjkWithInitializedFijk(h, fijk))
            return;  // no overage is possible; h lies on this face

        // if we're here we have the potential for an "overage"; i.e., it is
        // possible that c lies on an adjacent face

        CoordIJK origIJK = fijk->coord;

        // if we're in Class III, drop into the next finer Class II grid
        int res = H3_GET_RESOLUTION(h);
        if (isResClassIII(res)) {
            // Class III
            _downAp7r(&fijk->coord);
            res++;
        }

        // adjust for overage if needed
        // a pentagon base cell with a leading 4 digit requires special handling
        int pentLeading4 =
            (_isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) == 4);
        if (_adjustOverageClassII(fijk, res, pentLeading4, 0) != NO_OVERAGE) {
            // if the base cell is a pentagon we have the potential for secondary
            // overages
            if (_isBaseCellPentagon(baseCell)) {
                while (_adjustOverageClassII(fijk, res, 0, 0) != NO_OVERAGE)
                    continue;
            }

            if (res != H3_GET_RESOLUTION(h)) _upAp7r(&fijk->coord);
        } else if (res != H3_GET_RESOLUTION(h)) {
            fijk->coord = origIJK;
        }
        */
    }

    /**
     * Determines the cell boundary in spherical coordinates for an H3 index.
     *
     * @param h3 The H3 index.
     * @param gb The boundary of the H3 cell in spherical coordinates.
     */
    pub fn h3ToGeoBoundary(&self) -> GeoBoundary {
        let fijk: FaceIJK = self._h3ToFaceIjk();
        let res = self.get_resolution();
        if self.is_pentagon() {
            fijk._faceIjkPentToGeoBoundary(res, 0, NUM_PENT_VERTS as i32)
        } else {
            fijk._faceIjkToGeoBoundary(res, 0, NUM_HEX_VERTS)
        }
    }

    /**
     * Returns the max number of possible icosahedron faces an H3 index
     * may intersect.
     *
     * @return int count of faces
     */
    pub fn maxFaceCount(&self) -> i32 {
        // a pentagon always intersects 5 faces, a hexagon never intersects more than 2 (but may only intersect 1)
        if self.is_pentagon() {
            5
        } else {
            3
        }
    }
}

use crate::{
    basecell::BaseCell,
    basecellrotation::faceIjkBaseCells,
    constants::{NUM_HEX_VERTS, NUM_ICOSA_FACES, NUM_PENT_VERTS},
    coordijk::CoordIJK,
    geopolygon::GeoBoundary,
    h3index::H3Mode,
    vec2d::Overage,
    Direction, GeoCoord, H3Index, Resolution,
};

#[derive(Copy, Clone, Debug, Default)]
/// Face number and ijk coordinates on that face-centered coordinate system
pub(crate) struct FaceIJK {
    /// face number
    pub(crate) face: i32,
    /// ijk coordinates on that face
    pub(crate) coord: CoordIJK,
}

impl FaceIJK {
    const MAX_FACE_COORD: i32 = 2;

    pub(crate) const fn new(face: i32, coord: (i32, i32, i32)) -> Self {
        let coord = CoordIJK::new(coord.0, coord.1, coord.2);

        Self { face, coord }
    }

    /**
     * Generates the cell boundary in spherical coordinates for a cell given by a
     * FaceIJK address at a specified resolution.
     *
     * @param h The FaceIJK address of the cell.
     * @param res The H3 resolution of the cell.
     * @param start The first topological vertex to return.
     * @param length The number of topological vertexes to return.
     * @param g The spherical coordinates of the cell boundary.
     */
    pub(crate) fn _faceIjkToGeoBoundary(
        &self,
        res: Resolution,
        start: i32,
        length: i32,
    ) -> GeoBoundary {
        let mut adjRes = res;
        let mut centerIJK = *self;
        let fijkVerts = centerIJK._faceIjkToVerts(&mut adjRes);
        //[NUM_HEX_VERTS];

        todo!()
        /*

            // If we're returning the entire loop, we need one more iteration in case
            // of a distortion vertex on the last edge
            int additionalIteration = length == NUM_HEX_VERTS ? 1 : 0;

            // convert each vertex to lat/lon
            // adjust the face of each vertex as appropriate and introduce
            // edge-crossing vertices as needed
            g->numVerts = 0;
            int lastFace = -1;
            Overage lastOverage = NO_OVERAGE;
            for (int vert = start; vert < start + length + additionalIteration; vert++) {
            int v = vert % NUM_HEX_VERTS;

            FaceIJK fijk = fijkVerts[v];

            const int pentLeading4 = 0;
            Overage overage = _adjustOverageClassII(&fijk, adjRes, pentLeading4, 1);

            /*
            Check for edge-crossing. Each face of the underlying icosahedron is a
            different projection plane. So if an edge of the hexagon crosses an
            icosahedron edge, an additional vertex must be introduced at that
            intersection point. Then each half of the cell edge can be projected
            to geographic coordinates using the appropriate icosahedron face
            projection. Note that Class II cell edges have vertices on the face
            edge, with no edge line intersections.
            */
            if (isResClassIII(res) && vert > start && fijk.face != lastFace &&
                lastOverage != FACE_EDGE) {
                // find hex2d of the two vertexes on original face
                int lastV = (v + 5) % NUM_HEX_VERTS;
                Vec2d orig2d0;
                _ijkToHex2d(&fijkVerts[lastV].coord, &orig2d0);

                Vec2d orig2d1;
                _ijkToHex2d(&fijkVerts[v].coord, &orig2d1);

                // find the appropriate icosa face edge vertexes
                int maxDim = maxDimByCIIres[adjRes];
                Vec2d v0 = {3.0 * maxDim, 0.0};
                Vec2d v1 = {-1.5 * maxDim, 3.0 * M_SQRT3_2 * maxDim};
                Vec2d v2 = {-1.5 * maxDim, -3.0 * M_SQRT3_2 * maxDim};

                int face2 = ((lastFace == centerIJK.face) ? fijk.face : lastFace);
                Vec2d* edge0;
                Vec2d* edge1;
                switch (adjacentFaceDir[centerIJK.face][face2]) {
                    case IJ:
                        edge0 = &v0;
                        edge1 = &v1;
                        break;
                    case JK:
                        edge0 = &v1;
                        edge1 = &v2;
                        break;
                        // case KI:
                    default:
                        assert(adjacentFaceDir[centerIJK.face][face2] == KI);
                        edge0 = &v2;
                        edge1 = &v0;
                        break;
                }

                // find the intersection and add the lat/lon point to the result
                Vec2d inter;
                _v2dIntersect(&orig2d0, &orig2d1, edge0, edge1, &inter);
                /*
                   If a point of intersection occurs at a hexagon vertex, then each
                   adjacent hexagon edge will lie completely on a single icosahedron
                   face, and no additional vertex is required.
                   */
                bool isIntersectionAtVertex =
                    _v2dEquals(&orig2d0, &inter) || _v2dEquals(&orig2d1, &inter);
                if (!isIntersectionAtVertex) {
                    _hex2dToGeo(&inter, centerIJK.face, adjRes, 1,
                                &g->verts[g->numVerts]);
                    g->numVerts++;
                }
            }

            // convert vertex to lat/lon and add to the result
            // vert == start + NUM_HEX_VERTS is only used to test for possible
            // intersection on last edge
            if (vert < start + NUM_HEX_VERTS) {
                Vec2d vec;
                _ijkToHex2d(&fijk.coord, &vec);
                _hex2dToGeo(&vec, fijk.face, adjRes, 1, &g->verts[g->numVerts]);
                g->numVerts++;
            }

            lastFace = fijk.face;
            lastOverage = overage;
        }
        */
    }

    /**
     * Convert an FaceIJK address to the corresponding H3Index.
     * @param fijk The FaceIJK address.
     * @param res The cell resolution.
     * @return The encoded H3Index (or H3_NULL on failure).
     */
    pub(crate) fn _faceIjkToH3(&self, res: Resolution) -> H3Index {
        // initialize the index
        let mut h = H3Index::H3_INIT;
        h.set_mode(H3Mode::H3_HEXAGON_MODE);
        h.set_resolution(res);

        // check for res 0/base cell
        if res == Resolution::R0 {
            if self.coord.i > Self::MAX_FACE_COORD
                || self.coord.j > Self::MAX_FACE_COORD
                || self.coord.k > Self::MAX_FACE_COORD
            {
                // out of range input
                return H3Index::H3_NULL;
            }

            h.set_base_cell(self._faceIjkToBaseCell());
            return h;
        }

        // we need to find the correct base cell FaceIJK for this H3 index;
        // start with the passed in face and resolution res ijk coordinates
        // in that face's coordinate system
        let mut fijkBC = *self;

        // build the H3Index from finest res up
        // adjust r for the fact that the res 0 base cell offsets the indexing digits
        for r in (0..res.into()).rev() {
            let lastIJK = fijkBC.coord;
            let mut lastCenter;

            let rr: Resolution = (r + 1).into();

            if rr.isResClassIII() {
                // rotate ccw
                fijkBC.coord._upAp7();
                lastCenter = fijkBC.coord;
                lastCenter._downAp7();
            } else {
                // rotate cw
                fijkBC.coord._upAp7r();
                lastCenter = fijkBC.coord;
                lastCenter._downAp7r();
            }

            let mut diff = lastIJK - lastCenter;
            diff.normalize();

            h.set_index_digit(rr, diff._unitIjkToDigit().into());
        }

        // fijkBC should now hold the IJK of the base cell in the
        // coordinate system of the current face

        if fijkBC.coord.i > Self::MAX_FACE_COORD
            || fijkBC.coord.j > Self::MAX_FACE_COORD
            || fijkBC.coord.k > Self::MAX_FACE_COORD
        {
            // out of range input
            return H3Index::H3_NULL;
        }

        // lookup the correct base cell
        let baseCell = fijkBC._faceIjkToBaseCell();
        h.set_base_cell(baseCell);

        // rotate if necessary to get canonical base cell orientation for this base cell
        let numRots = fijkBC._faceIjkToBaseCellCCWrot60();

        if baseCell._isBaseCellPentagon() {
            // force rotation out of missing k-axes sub-sequence
            if h._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                // check for a cw/ccw offset face; default is ccw
                if baseCell._baseCellIsCwOffset(&fijkBC) {
                    h = h._h3Rotate60cw();
                } else {
                    h = h._h3Rotate60ccw();
                }
            }

            for _ in 0..numRots {
                h = h._h3RotatePent60ccw();
            }
        } else {
            for _ in 0..numRots {
                h = h._h3Rotate60ccw();
            }
        }

        h
    }

    /** Find base cell given FaceIJK.
     *
     * Given the face number and a resolution 0 ijk+ coordinate in that face's
     * face-centered ijk coordinate system, return the base cell located at that
     * coordinate.
     *
     * Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
     */
    pub(crate) fn _faceIjkToBaseCell(&self) -> BaseCell {
        faceIjkBaseCells[self.face as usize][self.coord.i as usize][self.coord.j as usize]
            [self.coord.k as usize]
            .baseCell
            .into()
    }

    /// Find base cell given FaceIJK.
    ///
    /// Given the face number and a resolution 0 ijk+ coordinate in that face's
    /// face-centered ijk coordinate system, return the number of 60' ccw rotations
    /// to rotate into the coordinate system of the base cell at that coordinates.
    ///
    /// Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
    pub(crate) fn _faceIjkToBaseCellCCWrot60(&self) -> i32 {
        faceIjkBaseCells[self.face as usize][self.coord.i as usize][self.coord.j as usize]
            [self.coord.k as usize]
            .ccwRot60
    }

    /**
     * Determines the center point in spherical coordinates of a cell given by
     * a FaceIJK address at a specified resolution.
     *
     * @param h The FaceIJK address of the cell.
     * @param res The H3 resolution of the cell.
     * @param g The spherical coordinates of the cell center point.
     */
    pub(crate) fn _faceIjkToGeo(&self, res: Resolution) -> GeoCoord {
        let v = self.coord._ijkToHex2d();
        v._hex2dToGeo(self.face, res, false)
    }

    /**
     * Generates the cell boundary in spherical coordinates for a pentagonal cell
     * given by a FaceIJK address at a specified resolution.
     *
     * @param h The FaceIJK address of the pentagonal cell.
     * @param res The H3 resolution of the cell.
     * @param start The first topological vertex to return.
     * @param length The number of topological vertexes to return.
     * @param g The spherical coordinates of the cell boundary.
     */
    pub(crate) fn _faceIjkPentToGeoBoundary(
        &self, /* h */
        res: Resolution,
        start: i32,
        length: i32,
    ) -> GeoBoundary {
        todo!()
        /*
            int adjRes = res;
            FaceIJK centerIJK = *h;
            FaceIJK fijkVerts[NUM_PENT_VERTS];
            _faceIjkPentToVerts(&centerIJK, &adjRes, fijkVerts);

            // If we're returning the entire loop, we need one more iteration in case
            // of a distortion vertex on the last edge
            int additionalIteration = length == NUM_PENT_VERTS ? 1 : 0;

            // convert each vertex to lat/lon
            // adjust the face of each vertex as appropriate and introduce
            // edge-crossing vertices as needed
            g->numVerts = 0;
            FaceIJK lastFijk;
            for (int vert = start; vert < start + length + additionalIteration;
                 vert++) {
                int v = vert % NUM_PENT_VERTS;

                FaceIJK fijk = fijkVerts[v];

                _adjustPentVertOverage(&fijk, adjRes);

                // all Class III pentagon edges cross icosa edges
                // note that Class II pentagons have vertices on the edge,
                // not edge intersections
                if (isResClassIII(res) && vert > start) {
                    // find hex2d of the two vertexes on the last face

                    FaceIJK tmpFijk = fijk;

                    Vec2d orig2d0;
                    _ijkToHex2d(&lastFijk.coord, &orig2d0);

                    int currentToLastDir = adjacentFaceDir[tmpFijk.face][lastFijk.face];

                    const FaceOrientIJK* fijkOrient =
                        &faceNeighbors[tmpFijk.face][currentToLastDir];

                    tmpFijk.face = fijkOrient->face;
                    CoordIJK* ijk = &tmpFijk.coord;

                    // rotate and translate for adjacent face
                    for (int i = 0; i < fijkOrient->ccwRot60; i++) _ijkRotate60ccw(ijk);

                    CoordIJK transVec = fijkOrient->translate;
                    _ijkScale(&transVec, unitScaleByCIIres[adjRes] * 3);
                    _ijkAdd(ijk, &transVec, ijk);
                    _ijkNormalize(ijk);

                    Vec2d orig2d1;
                    _ijkToHex2d(ijk, &orig2d1);

                    // find the appropriate icosa face edge vertexes
                    int maxDim = maxDimByCIIres[adjRes];
                    Vec2d v0 = {3.0 * maxDim, 0.0};
                    Vec2d v1 = {-1.5 * maxDim, 3.0 * M_SQRT3_2 * maxDim};
                    Vec2d v2 = {-1.5 * maxDim, -3.0 * M_SQRT3_2 * maxDim};

                    Vec2d* edge0;
                    Vec2d* edge1;
                    switch (adjacentFaceDir[tmpFijk.face][fijk.face]) {
                        case IJ:
                            edge0 = &v0;
                            edge1 = &v1;
                            break;
                        case JK:
                            edge0 = &v1;
                            edge1 = &v2;
                            break;
                        case KI:
                        default:
                            assert(adjacentFaceDir[tmpFijk.face][fijk.face] == KI);
                            edge0 = &v2;
                            edge1 = &v0;
                            break;
                    }

                    // find the intersection and add the lat/lon point to the result
                    Vec2d inter;
                    _v2dIntersect(&orig2d0, &orig2d1, edge0, edge1, &inter);
                    _hex2dToGeo(&inter, tmpFijk.face, adjRes, 1,
                                &g->verts[g->numVerts]);
                    g->numVerts++;
                }

                // convert vertex to lat/lon and add to the result
                // vert == start + NUM_PENT_VERTS is only used to test for possible
                // intersection on last edge
                if (vert < start + NUM_PENT_VERTS) {
                    Vec2d vec;
                    _ijkToHex2d(&fijk.coord, &vec);
                    _hex2dToGeo(&vec, fijk.face, adjRes, 1, &g->verts[g->numVerts]);
                    g->numVerts++;
                }

                lastFijk = fijk;
            }
        */
    }

    /**
     * Get the vertices of a cell as substrate FaceIJK addresses
     *
     * @param fijk The FaceIJK address of the cell.
     * @param res The H3 resolution of the cell. This may be adjusted if
     *            necessary for the substrate grid resolution.
     * @param fijkVerts Output array for the vertices
     */
    fn _faceIjkToVerts(&mut self, res: &mut Resolution) -> [FaceIJK; NUM_HEX_VERTS as usize] {
        // the vertexes of an origin-centered cell in a Class II resolution on a
        // substrate grid with aperture sequence 33r. The aperture 3 gets us the
        // vertices, and the 3r gets us back to Class II.
        // vertices listed ccw from the i-axes
        const vertsCII: [CoordIJK; NUM_HEX_VERTS as usize] = [
            CoordIJK::new(2, 1, 0), // 0
            CoordIJK::new(1, 2, 0), // 1
            CoordIJK::new(0, 2, 1), // 2
            CoordIJK::new(0, 1, 2), // 3
            CoordIJK::new(1, 0, 2), // 4
            CoordIJK::new(2, 0, 1), // 5
        ];

        // the vertexes of an origin-centered cell in a Class III resolution on a
        // substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
        // vertices, and the 3r7r gets us to Class II.
        // vertices listed ccw from the i-axes
        const vertsCIII: [CoordIJK; NUM_HEX_VERTS as usize] = [
            CoordIJK::new(5, 4, 0), // 0
            CoordIJK::new(1, 5, 0), // 1
            CoordIJK::new(0, 5, 4), // 2
            CoordIJK::new(0, 1, 5), // 3
            CoordIJK::new(4, 0, 5), // 4
            CoordIJK::new(5, 0, 1), // 5
        ];

        // get the correct set of substrate vertices for this resolution
        let verts = if res.isResClassIII() {
            vertsCIII
        } else {
            vertsCII
        };

        // adjust the center point to be in an aperture 33r substrate grid
        // these should be composed for speed
        self.coord._downAp3();
        self.coord._downAp3r();

        // if res is Class III we need to add a cw aperture 7 to get to icosahedral Class II
        if res.isResClassIII() {
            self.coord._downAp7r();
            *res = *res + 1;
        }

        // The center point is now in the same substrate grid as the origin
        // cell vertices. Add the center point substate coordinates
        // to each vertex to translate the vertices to that cell.
        let mut fijkVerts = [FaceIJK::default(); NUM_HEX_VERTS as usize];
        for v in 0..NUM_HEX_VERTS as usize {
            let face = self.face;
            let coord = self.coord + verts[v];

            fijkVerts[v] = FaceIJK { face, coord };
            fijkVerts[v].coord.normalize();
        }

        fijkVerts
    }

    /**
     * Get the vertices of a pentagon cell as substrate FaceIJK addresses
     *
     * @param fijk The FaceIJK address of the cell.
     * @param res The H3 resolution of the cell. This may be adjusted if necessary for the substrate grid resolution.
     * @param fijkVerts Output array for the vertices
     */
    pub(crate) fn _faceIjkPentToVerts(
        fijk: &mut Self,
        res: &mut Resolution,
    ) -> [FaceIJK; NUM_PENT_VERTS] {
        // the vertexes of an origin-centered pentagon in a Class II resolution on a
        // substrate grid with aperture sequence 33r. The aperture 3 gets us the
        // vertices, and the 3r gets us back to Class II.
        // vertices listed ccw from the i-axes
        const vertsCII: [CoordIJK; NUM_PENT_VERTS] = [
            CoordIJK::new(2, 1, 0), // 0
            CoordIJK::new(1, 2, 0), // 1
            CoordIJK::new(0, 2, 1), // 2
            CoordIJK::new(0, 1, 2), // 3
            CoordIJK::new(1, 0, 2), // 4
        ];

        // the vertexes of an origin-centered pentagon in a Class III resolution on
        // a substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
        // vertices, and the 3r7r gets us to Class II. vertices listed ccw from the
        // i-axes
        const vertsCIII: [CoordIJK; NUM_PENT_VERTS] = [
            CoordIJK::new(5, 4, 0), // 0
            CoordIJK::new(1, 5, 0), // 1
            CoordIJK::new(0, 5, 4), // 2
            CoordIJK::new(0, 1, 5), // 3
            CoordIJK::new(4, 0, 5), // 4
        ];

        // get the correct set of substrate vertices for this resolution
        let verts = if res.isResClassIII() {
            vertsCIII
        } else {
            vertsCII
        };

        // adjust the center point to be in an aperture 33r substrate grid
        // these should be composed for speed
        fijk.coord._downAp3();
        fijk.coord._downAp3r();

        // if res is Class III we need to add a cw aperture 7 to get to icosahedral Class II
        if res.isResClassIII() {
            fijk.coord._downAp7r();
            *res = Resolution::from(*res as usize + 1);
        }

        // The center point is now in the same substrate grid as the origin
        // cell vertices. Add the center point substate coordinates
        // to each vertex to translate the vertices to that cell.
        let mut fijkVerts = [FaceIJK::default(); NUM_PENT_VERTS];
        for v in 0..NUM_PENT_VERTS {
            fijkVerts[v].face = fijk.face;
            fijkVerts[v].coord = fijk.coord + verts[v];
            fijkVerts[v].coord.normalize();
        }

        fijkVerts
    }

    /**
     * Adjusts a FaceIJK address for a pentagon vertex in a substrate grid in
     * place so that the resulting cell address is relative to the correct
     * icosahedral face.
     *
     * @param fijk The FaceIJK address of the cell.
     * @param res The H3 resolution of the cell.
     */
    pub(crate) fn _adjustPentVertOverage(&mut self, res: Resolution) -> Overage {
        let mut pentLeading4 = false;

        loop {
            let overage = self._adjustOverageClassII(res, pentLeading4, true);
            if overage == Overage::NEW_FACE {
                return overage;
            }
        }
    }

    /**
     * Adjusts a FaceIJK address in place so that the resulting cell address is
     * relative to the correct icosahedral face.
     *
     * @param fijk The FaceIJK address of the cell.
     * @param res The H3 resolution of the cell.
     * @param pentLeading4 Whether or not the cell is a pentagon with a leading
     *        digit 4.
     * @param substrate Whether or not the cell is in a substrate grid.
     * @return 0 if on original face (no overage); 1 if on face edge (only occurs
     *         on substrate grids); 2 if overage on new face interior
     */
    fn _adjustOverageClassII(
        &mut self,
        res: Resolution,
        pentLeading4: bool,
        substrate: bool,
    ) -> Overage {
        let mut overage = Overage::NO_OVERAGE;

        let mut ijk = self.coord;

        // get the maximum dimension value; scale if a substrate grid
        let maxDim = if substrate {
            res.maxDimByCIIres() * 3
        } else {
            res.maxDimByCIIres()
        };

        // check for overage
        if substrate && ijk.i + ijk.j + ijk.k == maxDim {
            // on edge
            overage = Overage::FACE_EDGE;
        } else if ijk.i + ijk.j + ijk.k > maxDim
        // overage
        {
            overage = Overage::NEW_FACE;

            let fijkOrient;
            if ijk.k > 0 {
                if ijk.j > 0 {
                    // jk "quadrant"
                    fijkOrient = &faceNeighbors[self.face as usize][crate::JK as usize];
                } else {
                    // ik "quadrant"
                    fijkOrient = &faceNeighbors[self.face as usize][crate::KI as usize];

                    // adjust for the pentagonal missing sequence
                    if pentLeading4 {
                        // translate origin to center of pentagon
                        let origin = CoordIJK::new(maxDim, 0, 0);
                        let mut tmp = ijk - origin;
                        // rotate to adjust for the missing sequence
                        tmp._ijkRotate60cw();
                        // translate the origin back to the center of the triangle
                        ijk = tmp + origin;
                    }
                }
            } else {
                // ij "quadrant"
                fijkOrient = &faceNeighbors[self.face as usize][crate::IJ as usize];
            }

            self.face = fijkOrient.face;

            // rotate and translate for adjacent face
            for _ in 0..fijkOrient.ccwRot60 {
                ijk._ijkRotate60ccw();
            }

            let mut unitScale = res.unitScaleByCIIres();
            if substrate {
                unitScale *= 3;
            }

            let transVec = fijkOrient.translate * unitScale;
            ijk += transVec;
            ijk.normalize();

            // overage points on pentagon boundaries can end up on edges
            if substrate && ijk.i + ijk.j + ijk.k == maxDim {
                // on edge
                overage = Overage::FACE_EDGE;
            }
        }

        overage
    }
}

/// Information to transform into an adjacent face IJK system
pub(crate) struct FaceOrientIJK {
    /// face number
    face: i32,

    /// res 0 translation relative to primary face
    translate: CoordIJK,

    /// number of 60 degree ccw rotations relative to primary face
    ccwRot60: i32,
}

impl FaceOrientIJK {
    const fn new(face: i32, translate: (i32, i32, i32), ccwRot60: i32) -> Self {
        let translate = CoordIJK::new(translate.0, translate.1, translate.2);

        Self {
            face,
            translate,
            ccwRot60,
        }
    }
}

/// Definition of which faces neighbor each other.
const faceNeighbors: [[FaceOrientIJK; 4]; NUM_ICOSA_FACES] = [
    [
        // face 0
        FaceOrientIJK::new(0, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(4, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(1, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(5, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 1
        FaceOrientIJK::new(1, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(0, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(2, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(6, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 2
        FaceOrientIJK::new(2, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(1, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(3, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(7, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 3
        FaceOrientIJK::new(3, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(2, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(4, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(8, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 4
        FaceOrientIJK::new(4, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(3, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(0, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(9, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 5
        FaceOrientIJK::new(5, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(10, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(14, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(0, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 6
        FaceOrientIJK::new(6, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(11, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(10, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(1, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 7
        FaceOrientIJK::new(7, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(12, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(11, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(2, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 8
        FaceOrientIJK::new(8, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(13, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(12, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(3, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 9
        FaceOrientIJK::new(9, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(14, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(13, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(4, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 10
        FaceOrientIJK::new(10, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(5, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(6, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(15, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 11
        FaceOrientIJK::new(11, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(6, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(7, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(16, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 12
        FaceOrientIJK::new(12, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(7, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(8, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(17, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 13
        FaceOrientIJK::new(13, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(8, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(9, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(18, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 14
        FaceOrientIJK::new(14, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(9, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(5, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(19, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 15
        FaceOrientIJK::new(15, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(16, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(19, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(10, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 16
        FaceOrientIJK::new(16, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(17, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(15, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(11, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 17
        FaceOrientIJK::new(17, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(18, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(16, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(12, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 18
        FaceOrientIJK::new(18, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(19, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(17, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(13, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 19
        FaceOrientIJK::new(19, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(15, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(18, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(14, (0, 2, 2), 3), // jk quadrant
    ],
];

#[cfg(test)]
mod tests {
    #[test]
    fn faceIjkToH3ExtremeCoordinates() {
        /*
            FaceIJK fijk0I = {0, {3, 0, 0}};
            t_assert(_faceIjkToH3(&fijk0I, 0) == 0, "i out of bounds at res 0");
            FaceIJK fijk0J = {1, {0, 4, 0}};
            t_assert(_faceIjkToH3(&fijk0J, 0) == 0, "j out of bounds at res 0");
            FaceIJK fijk0K = {2, {2, 0, 5}};
            t_assert(_faceIjkToH3(&fijk0K, 0) == 0, "k out of bounds at res 0");

            FaceIJK fijk1I = {3, {6, 0, 0}};
            t_assert(_faceIjkToH3(&fijk1I, 1) == 0, "i out of bounds at res 1");
            FaceIJK fijk1J = {4, {0, 7, 1}};
            t_assert(_faceIjkToH3(&fijk1J, 1) == 0, "j out of bounds at res 1");
            FaceIJK fijk1K = {5, {2, 0, 8}};
            t_assert(_faceIjkToH3(&fijk1K, 1) == 0, "k out of bounds at res 1");

            FaceIJK fijk2I = {6, {18, 0, 0}};
            t_assert(_faceIjkToH3(&fijk2I, 2) == 0, "i out of bounds at res 2");
            FaceIJK fijk2J = {7, {0, 19, 1}};
            t_assert(_faceIjkToH3(&fijk2J, 2) == 0, "j out of bounds at res 2");
            FaceIJK fijk2K = {8, {2, 0, 20}};
            t_assert(_faceIjkToH3(&fijk2K, 2) == 0, "k out of bounds at res 2");
        */
    }
}

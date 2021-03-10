use crate::{CoordIJK, Vec3d};

///Information to transform into an adjacent face IJK system
pub struct FaceOrientIJK {
    /// face number
    face: i32,
    /// res 0 translation relative to primary face
    translate: CoordIJK,
    /// number of 60 degree ccw rotations relative to primary face
    ccwRot60: i32,
}

impl FaceOrientIJK {
    pub const fn new(face: i32, translate_ijk: [i32; 3], ccwRot60: i32) -> Self {
        let translate = CoordIJK::new(translate_ijk[0], translate_ijk[1], translate_ijk[2]);
        Self {
            face,
            translate,
            ccwRot60,
        }
    }
}

#[derive(Copy, Clone, Default)]
/// Face number and ijk coordinates on that face-centered coordinate system
pub struct FaceIJK {
    /// face number
    face: i32,
    /// ijk coordinates on that face
    coord: CoordIJK,
}

impl FaceIJK {
    pub const fn new(face: i32, coord: [i32; 3]) -> Self {
        FaceIjK {
            face,
            coord: CoordIJK::new(coord[0], coord[1], coord[2]),
        }
    }

    /// Find base cell given FaceIJK.
    ///
    ///Given the face number and a resolution 0 ijk+ coordinate in that face's
    ///face-centered ijk coordinate system, return the base cell located at that
    ///coordinate.
    ///
    ///Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
    fn _faceIjkToBaseCell(&self) -> i32 {
        faceIjkBaseCells[self.face][self.coord.i][self.coord.j][self.coord.k].baseCell
    }

    /// Find base cell given FaceIJK.
    ///
    ///Given the face number and a resolution 0 ijk+ coordinate in that face's
    ///face-centered ijk coordinate system, return the number of 60' ccw rotations
    ///to rotate into the coordinate system of the base cell at that coordinates.
    ///
    ///Valid ijk+ lookup coordinates are from (0, 0, 0) to (2, 2, 2).
    fn _faceIjkToBaseCellCCWrot60(&self) -> i32 {
        faceIjkBaseCells[self.face][self.coord.i][self.coord.j][self.coord.k].ccwRot60
    }

    /// Find the FaceIJK given a base cell.
    fn _baseCellToFaceIjk(baseCell: i32) -> Self {
        baseCellData[baseCell].homeFijk
    }

    /// Determines the center point in spherical coordinates of a cell given by a FaceIJK address at a specified resolution.
    ///
    ///@param h The FaceIJK address of the cell.
    ///@param res The H3 resolution of the cell.
    ///@param g The spherical coordinates of the cell center point.
    pub fn _faceIjkToGeo(&self, res: i32) -> GeoCoord {
        let v: Vec2d = self.coord._ijkToHex2d();
        v._hex2dToGeo(self.face, res, false)
    }

    /// Generates the cell boundary in spherical coordinates for a pentagonal cell given by a FaceIJK address at a specified resolution.
    ///
    ///@param h The FaceIJK address of the pentagonal cell.
    ///@param res The H3 resolution of the cell.
    ///@param start The first topological vertex to return.
    ///@param length The number of topological vertexes to return.
    ///@param g The spherical coordinates of the cell boundary.
    fn _faceIjkPentToGeoBoundary(
        &self, /* h */
        res: i32,
        start: i32,
        length: i32,
    ) -> GeoBoundary /* g */ {
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

    /// Get the vertices of a pentagon cell as substrate FaceIJK addresses
    ///
    ///@param fijk The FaceIJK address of the cell.
    ///@param res The H3 resolution of the cell. This may be adjusted if
    ///           necessary for the substrate grid resolution.
    ///@param fijkVerts Output array for the vertices
    fn _faceIjkPentToVerts(&mut self /*fijk*/, res: &mut i32) -> FaceIJK /*fijkVerts*/ {
        todo!()
        /*
        // the vertexes of an origin-centered pentagon in a Class II resolution on a
        // substrate grid with aperture sequence 33r. The aperture 3 gets us the
        // vertices, and the 3r gets us back to Class II.
        // vertices listed ccw from the i-axes
        CoordIJK vertsCII[NUM_PENT_VERTS] = {
        {2, 1, 0},  // 0
        {1, 2, 0},  // 1
        {0, 2, 1},  // 2
        {0, 1, 2},  // 3
        {1, 0, 2},  // 4
        };

        // the vertexes of an origin-centered pentagon in a Class III resolution on
        // a substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
        // vertices, and the 3r7r gets us to Class II. vertices listed ccw from the
        // i-axes
        CoordIJK vertsCIII[NUM_PENT_VERTS] = {
        {5, 4, 0},  // 0
        {1, 5, 0},  // 1
        {0, 5, 4},  // 2
        {0, 1, 5},  // 3
        {4, 0, 5},  // 4
        };

        // get the correct set of substrate vertices for this resolution
        CoordIJK* verts;
        if (isResClassIII(*res))
        verts = vertsCIII;
        else
        verts = vertsCII;

        // adjust the center point to be in an aperture 33r substrate grid
        // these should be composed for speed
        _downAp3(&fijk->coord);
        _downAp3r(&fijk->coord);

        // if res is Class III we need to add a cw aperture 7 to get to
        // icosahedral Class II
        if (isResClassIII(*res)) {
        _downAp7r(&fijk->coord);
         *res += 1;
         }

        // The center point is now in the same substrate grid as the origin
        // cell vertices. Add the center point substate coordinates
        // to each vertex to translate the vertices to that cell.
        for (int v = 0; v < NUM_PENT_VERTS; v++) {
        fijkVerts[v].face = fijk->face;
        _ijkAdd(&fijk->coord, &verts[v], &fijkVerts[v].coord);
        _ijkNormalize(&fijkVerts[v].coord);
        }
        */
    }

    ///
    ///Generates the cell boundary in spherical coordinates for a cell given by a
    ///FaceIJK address at a specified resolution.
    ///
    ///@param h The FaceIJK address of the cell.
    ///@param res The H3 resolution of the cell.
    ///@param start The first topological vertex to return.
    ///@param length The number of topological vertexes to return.
    ///@param g The spherical coordinates of the cell boundary.
    fn _faceIjkToGeoBoundary(&self /* h */, res: i32, start: i32, length: i32) -> GeoBoundary /* g */
    {
        todo!()
        /*
                   int adjRes = res;
                   FaceIJK centerIJK = *h;
                   FaceIJK fijkVerts[NUM_HEX_VERTS];
                   _faceIjkToVerts(&centerIJK, &adjRes, fijkVerts);

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

    /// Adjusts a FaceIJK address in place so that the resulting cell address is relative to the correct icosahedral face.
    ///
    ///@param fijk The FaceIJK address of the cell.
    ///@param res The H3 resolution of the cell.
    ///@param pentLeading4 Whether or not the cell is a pentagon with a leading
    ///       digit 4.
    ///@param substrate Whether or not the cell is in a substrate grid.
    ///@return 0 if on original face (no overage); 1 if on face edge (only occurs
    ///        on substrate grids); 2 if overage on new face interior
    fn _adjustOverageClassII(
        &mut self, /*fijk*/
        res: i32,
        pentLeading4: bool,
        substrate: bool,
    ) -> Overage {
        let mut overage = Overage::NO_OVERAGE;
        let mut ijk: CoordIJK = self.coord;

        // get the maximum dimension value; scale if a substrate grid
        let mut maxDim = maxDimByCIIres[res as usize];
        if substrate {
            maxDim *= 3;
        }

        // check for overage
        if substrate && ijk.i + ijk.j + ijk.k == maxDim {
            // on edge
            overage = FACE_EDGE;
        } else if ijk.i + ijk.j + ijk.k > maxDim {
            // overage
            overage = NEW_FACE;

            //const FaceOrientIJK* fijkOrient;
            let fijkOrient = if ijk.k > 0 {
                if ijk.j > 0 {
                    // jk "quadrant"
                    &faceNeighbors[self.face as usize][JK as usize]
                } else {
                    // adjust for the pentagonal missing sequence
                    if pentLeading4 {
                        // translate origin to center of pentagon
                        let origin = CoordIJK::new(maxDim, 0, 0);
                        let mut tmp: CoordIJK = ijk - origin;
                        // rotate to adjust for the missing sequence
                        tmp._ijkRotate60cw();
                        // translate the origin back to the center of the triangle
                        ijk = tmp + origin;
                    }
                    // ik "quadrant"
                    faceNeighbors[self.face as usize][KI as usize]
                }
            } else {
                // ij "quadrant"
                faceNeighbors[self.face as usize][IJ as usize]
            };

            self.face = fijkOrient.face;

            // rotate and translate for adjacent face
            for _ in 0..fijkOrient.ccwRot60 {
                ijk._ijkRotate60ccw();
            }

            let mut transVec = fijkOrient.translate;
            let unitScale = unitScaleByCIIres[res as usize];
            if substrate {
                unitScale *= 3;
            }

            transVec *= unitScale;
            ijk += transVec;
            ijk._ijkNormalize();

            // overage points on pentagon boundaries can end up on edges
            if substrate && ijk.i + ijk.j + ijk.k == maxDim {
                // on edge
                overage = Overage::FACE_EDGE;
            }
        }

        overage
    }

    ///Adjusts a FaceIJK address for a pentagon vertex in a substrate grid in place so that the resulting cell address is relative to the correct icosahedral face.
    ///
    ///@param fijk The FaceIJK address of the cell.
    ///@param res The H3 resolution of the cell.
    fn _adjustPentVertOverage(&mut self, res: i32) -> Overage {
        let pentLeading4 = false;
        let mut overage = Overage::NEW_FACE;
        while overage == Overage::NEW_FACE {
            overage = self._adjustOverageClassII(res, pentLeading4, 1);
        }

        overage
    }

    /**
     * Convert an FaceIJK address to the corresponding H3Index.
     * @param fijk The FaceIJK address.
     * @param res The cell resolution.
     * @return The encoded H3Index (or H3_NULL on failure).
     */
    fn _faceIjkToH3(&self /*fijk*/, res: i32) -> H3Index {
        // initialize the index
        let mut h = H3Index::H3_INIT();
        h.H3_SET_MODE(H3_HEXAGON_MODE);
        h.H3_SET_RESOLUTION(res);

        // check for res 0/base cell
        if res == 0 {
            if self.coord.i > MAX_FACE_COORD
                || self.coord.j > MAX_FACE_COORD
                || self.coord.k > MAX_FACE_COORD
            {
                // out of range input
                return H3_NULL;
            }

            h.H3_SET_BASE_CELL(self._faceIjkToBaseCell());
            return h;
        }

        todo!()
        /*
        // we need to find the correct base cell FaceIJK for this H3 index;
        // start with the passed in face and resolution res ijk coordinates
        // in that face's coordinate system
        FaceIJK fijkBC = *fijk;

        // build the H3Index from finest res up
        // adjust r for the fact that the res 0 base cell offsets the indexing
        // digits
        CoordIJK* ijk = &fijkBC.coord;
        for (int r = res - 1; r >= 0; r--) {
        CoordIJK lastIJK = *ijk;
        CoordIJK lastCenter;
        if (isResClassIII(r + 1)) {
        // rotate ccw
        _upAp7(ijk);
        lastCenter = *ijk;
        _downAp7(&lastCenter);
        } else {
        // rotate cw
        _upAp7r(ijk);
        lastCenter = *ijk;
        _downAp7r(&lastCenter);
        }

        let mut diff :  CoordIJK = lastIJK - lastCenter;
        diff._ijkNormalize();

        h.H3_SET_INDEX_DIGIT(r + 1, diff._unitIjkToDigit());
        }

        // fijkBC should now hold the IJK of the base cell in the
        // coordinate system of the current face

        if (fijkBC.coord.i > MAX_FACE_COORD
        || fijkBC.coord.j > MAX_FACE_COORD
        || fijkBC.coord.k > MAX_FACE_COORD) {
        // out of range input
        return H3_NULL;
        }

        // lookup the correct base cell
        let baseCell = fijkBC._faceIjkToBaseCell();
        h.H3_SET_BASE_CELL(baseCell);

        // rotate if necessary to get canonical base cell orientation
        // for this base cell
        let numRots = fijkBC._faceIjkToBaseCellCCWrot60();
        if (_isBaseCellPentagon(baseCell)) {
        // force rotation out of missing k-axes sub-sequence
        if (_h3LeadingNonZeroDigit(h) == K_AXES_DIGIT) {
        // check for a cw/ccw offset face; default is ccw
        if (_baseCellIsCwOffset(baseCell, fijkBC.face)) {
        h._h3Rotate60cw();
        } else {
        h._h3Rotate60ccw();
        }
        }

        for i in 0..numRots {
        h._h3RotatePent60ccw(h);
        }
        } else {
        for i in 0..numRots {
        h._h3Rotate60ccw();
        }
        }

        h
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
    //fn _faceIjkToVerts(FaceIJK* fijk, int* res, FaceIJK* fijkVerts)
    fn _faceIjkToVerts(&self, res: &mut i32) -> Vec<FaceIJK> {
        todo!("This function still in-progress");
        // the vertexes of an origin-centered cell in a Class II resolution on a
        // substrate grid with aperture sequence 33r. The aperture 3 gets us the
        // vertices, and the 3r gets us back to Class II.
        // vertices listed ccw from the i-axes
        let vertsCII: [CoordIJK; NUM_HEX_VERTS] = [
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
        let vertsCIII: [CoordIJK; NUM_HEX_VERTS] = [
            CoordIJK::new(5, 4, 0), // 0
            CoordIJK::new(1, 5, 0), // 1
            CoordIJK::new(0, 5, 4), // 2
            CoordIJK::new(0, 1, 5), // 3
            CoordIJK::new(4, 0, 5), // 4
            CoordIJK::new(5, 0, 1), // 5
        ];

        // get the correct set of substrate vertices for this resolution
        let verts = if isResClassIII(res) {
            vertsCIII
        } else {
            vertsCII
        };

        // adjust the center point to be in an aperture 33r substrate grid these should be composed for speed
        _downAp3(self.coord);
        _downAp3r(self.coord);

        // if res is Class III we need to add a cw aperture 7 to get to icosahedral Class II
        if isResClassIII(*res) {
            _downAp7r(&self.coord);
            *res += 1;
        }

        // The center point is now in the same substrate grid as the origin
        // cell vertices. Add the center point substate coordinates
        // to each vertex to translate the vertices to that cell.
        for v in 0..NUM_HEX_VERTS {
            fijkVerts[v].face = self.face;
            //_ijkAdd(&fijk->coord, &verts[v], &fijkVerts[v].coord);
            fijkVerts[v].coord = self.coord + verts[v];
            fijkVerts[v].coord._ijkNormalize();
        }

        todo!()
    }
}

// indexes for faceNeighbors table
/// IJ quadrant faceNeighbors table direction
const IJ: i32 = 1;
/// KI quadrant faceNeighbors table direction
const KI: i32 = 2;
/// JK quadrant faceNeighbors table direction
const JK: i32 = 3;

/// Invalid face index */
const INVALID_FACE: i32 = -1;

/// icosahedron face centers in lat/lon radians
const faceCenterGeo: [GeoCoord; NUM_ICOSA_FACES] = [
    GeoCoord::new(0.803582649718989942, 1.248397419617396099), // face  0
    GeoCoord::new(1.307747883455638156, 2.536945009877921159), // face  1
    GeoCoord::new(1.054751253523952054, -1.347517358900396623), // face  2
    GeoCoord::new(0.600191595538186799, -0.450603909469755746), // face  3
    GeoCoord::new(0.491715428198773866, 0.401988202911306943), // face  4
    GeoCoord::new(0.172745327415618701, 1.678146885280433686), // face  5
    GeoCoord::new(0.605929321571350690, 2.953923329812411617), // face  6
    GeoCoord::new(0.427370518328979641, -1.888876200336285401), // face  7
    GeoCoord::new(-0.079066118549212831, -0.733429513380867741), // face  8
    GeoCoord::new(-0.230961644455383637, 0.506495587332349035), // face  9
    GeoCoord::new(0.079066118549212831, 2.408163140208925497), // face 10
    GeoCoord::new(0.230961644455383637, -2.635097066257444203), // face 11
    GeoCoord::new(-0.172745327415618701, -1.463445768309359553), // face 12
    GeoCoord::new(-0.605929321571350690, -0.187669323777381622), // face 13
    GeoCoord::new(-0.427370518328979641, 1.252716453253507838), // face 14
    GeoCoord::new(-0.600191595538186799, 2.690988744120037492), // face 15
    GeoCoord::new(-0.491715428198773866, -2.739604450678486295), // face 16
    GeoCoord::new(-0.803582649718989942, -1.893195233972397139), // face 17
    GeoCoord::new(-1.307747883455638156, -0.604647643711872080), // face 18
    GeoCoord::new(-1.054751253523952054, 1.794075294689396615), // face 19
];

/// icosahedron face centers in x/y/z on the unit sphere
const faceCenterPoint: [Vec3d; NUM_ICOSA_FACES] = [
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

/// icosahedron face ijk axes as azimuth in radians from face center to * vertex 0/1/2 respectively
const faceAxesAzRadsCII: [[f64; 3]; NUM_ICOSA_FACES] = [
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

/// overage distance table
const maxDimByCIIres: [i32; 17] = [
    2,        // res  0
    -1,       // res  1
    14,       // res  2
    -1,       // res  3
    98,       // res  4
    -1,       // res  5
    686,      // res  6
    -1,       // res  7
    4802,     // res  8
    -1,       // res  9
    33614,    // res 10
    -1,       // res 11
    235298,   // res 12
    -1,       // res 13
    1647086,  // res 14
    -1,       // res 15
    11529602, // res 16
];

/// unit scale distance table
const unitScaleByCIIres: [i32; 17] = [
    1,       // res  0
    -1,      // res  1
    7,       // res  2
    -1,      // res  3
    49,      // res  4
    -1,      // res  5
    343,     // res  6
    -1,      // res  7
    2401,    // res  8
    -1,      // res  9
    16807,   // res 10
    -1,      // res 11
    117649,  // res 12
    -1,      // res 13
    823543,  // res 14
    -1,      // res 15
    5764801, // res 16
];

/// Definition of which faces neighbor each other.
const faceNeighbors: [[FaceOrientIJK; 4]; NUM_ICOSA_FACES] = [
    [
        // face 0
        FaceOrientIJK::new(0, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(4, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(1, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(5, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 1
        FaceOrientIJK::new(1, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(0, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(2, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(6, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 2
        FaceOrientIJK::new(2, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(1, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(3, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(7, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 3
        FaceOrientIJK::new(3, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(2, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(4, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(8, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 4
        FaceOrientIJK::new(4, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(3, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(0, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(9, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 5
        FaceOrientIJK::new(5, [0, 0, 0], 0),  // central face
        FaceOrientIJK::new(10, [2, 2, 0], 3), // ij quadrant
        FaceOrientIJK::new(14, [2, 0, 2], 3), // ki quadrant
        FaceOrientIJK::new(0, [0, 2, 2], 3),  // jk quadrant
    ],
    [
        // face 6
        FaceOrientIJK::new(6, [0, 0, 0], 0),  // central face
        FaceOrientIJK::new(11, [2, 2, 0], 3), // ij quadrant
        FaceOrientIJK::new(10, [2, 0, 2], 3), // ki quadrant
        FaceOrientIJK::new(1, [0, 2, 2], 3),  // jk quadrant
    ],
    [
        // face 7
        FaceOrientIJK::new(7, [0, 0, 0], 0),  // central face
        FaceOrientIJK::new(12, [2, 2, 0], 3), // ij quadrant
        FaceOrientIJK::new(11, [2, 0, 2], 3), // ki quadrant
        FaceOrientIJK::new(2, [0, 2, 2], 3),  // jk quadrant
    ],
    [
        // face 8
        FaceOrientIJK::new(8, [0, 0, 0], 0),  // central face
        FaceOrientIJK::new(13, [2, 2, 0], 3), // ij quadrant
        FaceOrientIJK::new(12, [2, 0, 2], 3), // ki quadrant
        FaceOrientIJK::new(3, [0, 2, 2], 3),  // jk quadrant
    ],
    [
        // face 9
        FaceOrientIJK::new(9, [0, 0, 0], 0),  // central face
        FaceOrientIJK::new(14, [2, 2, 0], 3), // ij quadrant
        FaceOrientIJK::new(13, [2, 0, 2], 3), // ki quadrant
        FaceOrientIJK::new(4, [0, 2, 2], 3),  // jk quadrant
    ],
    [
        // face 10
        FaceOrientIJK::new(10, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(5, [2, 2, 0], 3),  // ij quadrant
        FaceOrientIJK::new(6, [2, 0, 2], 3),  // ki quadrant
        FaceOrientIJK::new(15, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 11
        FaceOrientIJK::new(11, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(6, [2, 2, 0], 3),  // ij quadrant
        FaceOrientIJK::new(7, [2, 0, 2], 3),  // ki quadrant
        FaceOrientIJK::new(16, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 12
        FaceOrientIJK::new(12, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(7, [2, 2, 0], 3),  // ij quadrant
        FaceOrientIJK::new(8, [2, 0, 2], 3),  // ki quadrant
        FaceOrientIJK::new(17, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 13
        FaceOrientIJK::new(13, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(8, [2, 2, 0], 3),  // ij quadrant
        FaceOrientIJK::new(9, [2, 0, 2], 3),  // ki quadrant
        FaceOrientIJK::new(18, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 14
        FaceOrientIJK::new(14, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(9, [2, 2, 0], 3),  // ij quadrant
        FaceOrientIJK::new(5, [2, 0, 2], 3),  // ki quadrant
        FaceOrientIJK::new(19, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 15
        FaceOrientIJK::new(15, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(16, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(19, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(10, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 16
        FaceOrientIJK::new(16, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(17, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(15, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(11, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 17
        FaceOrientIJK::new(17, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(18, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(16, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(12, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 18
        FaceOrientIJK::new(18, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(19, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(17, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(13, [0, 2, 2], 3), // jk quadrant
    ],
    [
        // face 19
        FaceOrientIJK::new(19, [0, 0, 0], 0), // central face
        FaceOrientIJK::new(15, [2, 0, 2], 1), // ij quadrant
        FaceOrientIJK::new(18, [2, 2, 0], 5), // ki quadrant
        FaceOrientIJK::new(14, [0, 2, 2], 3), // jk quadrant
    ],
];

/// direction from the origin face to the destination face, relative to the origin face's coordinate system, or -1 if not adjacent.
const adjacentFaceDir: [[i32; NUM_ICOSA_FACES]; NUM_ICOSA_FACES] = [
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

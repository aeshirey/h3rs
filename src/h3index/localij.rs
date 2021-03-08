/* localij */
/**
 * Origin leading digit -> index leading digit -> rotations 60 cw
 * Either being 1 (K axis) is invalid.
 * No good default at 0.
 */
const PENTAGON_ROTATIONS: [[i32; 7]; 7] = [
    [0, -1, 0, 0, 0, 0, 0],       // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, -1, 0, 0, 0, 1, 0],       // 2
    [0, -1, 0, 0, 1, 1, 0],       // 3
    [0, -1, 0, 5, 0, 0, 0],       // 4
    [0, -1, 5, 5, 0, 0, 0],       // 5
    [0, -1, 0, 0, 0, 0, 0],       // 6
];

/**
 * Reverse base cell direction -> leading index digit -> rotations 60 ccw.
 * For reversing the rotation introduced in PENTAGON_ROTATIONS when
 * the origin is on a pentagon (regardless of the base cell of the index.)
 */
const PENTAGON_ROTATIONS_REVERSE: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],        // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, 1, 0, 0, 0, 0, 0],        // 2
    [0, 1, 0, 0, 0, 1, 0],        // 3
    [0, 5, 0, 0, 0, 0, 0],        // 4
    [0, 5, 0, 5, 0, 0, 0],        // 5
    [0, 0, 0, 0, 0, 0, 0],        // 6
];
/**
 * Reverse base cell direction -> leading index digit -> rotations 60 ccw.
 * For reversing the rotation introduced in PENTAGON_ROTATIONS when the index is
 * on a pentagon and the origin is not.
 */
const PENTAGON_ROTATIONS_REVERSE_NONPOLAR: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],        // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, 1, 0, 0, 0, 0, 0],        // 2
    [0, 1, 0, 0, 0, 1, 0],        // 3
    [0, 5, 0, 0, 0, 0, 0],        // 4
    [0, 1, 0, 5, 1, 1, 0],        // 5
    [0, 0, 0, 0, 0, 0, 0],        // 6
];
/**
 * Reverse base cell direction -> leading index digit -> rotations 60 ccw.
 * For reversing the rotation introduced in PENTAGON_ROTATIONS when the index is
 * on a polar pentagon and the origin is not.
 */
const PENTAGON_ROTATIONS_REVERSE_POLAR: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],        // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, 1, 1, 1, 1, 1, 1],        // 2
    [0, 1, 0, 0, 0, 1, 0],        // 3
    [0, 1, 0, 0, 1, 1, 1],        // 4
    [0, 1, 0, 5, 1, 1, 0],        // 5
    [0, 1, 1, 0, 1, 1, 1],        // 6
];

/**
 * Prohibited directions when unfolding a pentagon.
 *
 * Indexes by two directions, both relative to the pentagon base cell. The first
 * is the direction of the origin index and the second is the direction of the
 * index to unfold. Direction refers to the direction from base cell to base
 * cell if the indexes are on different base cells, or the leading digit if
 * within the pentagon base cell.
 *
 * This previously included a Class II/Class III check but these were removed
 * due to failure cases. It's possible this could be restricted to a narrower
 * set of a failure cases. Currently, the logic is any unfolding across more
 * than one icosahedron face is not permitted.
 */
const FAILED_DIRECTIONS: [[bool; 7]; 7] = [
    [false, false, false, false, false, false, false], // 0
    [false, false, false, false, false, false, false], // 1
    [false, false, false, false, true, true, false],   // 2
    [false, false, false, false, true, false, true],   // 3
    [false, false, true, true, false, false, false],   // 4
    [false, false, true, false, false, false, true],   // 5
    [false, false, false, true, false, true, false],   // 6
];

impl H3Index {
    /**
     * Produces the grid distance between the two indexes.
     *
     * This function may fail to find the distance between two indexes, for
     * example if they are very far apart. It may also fail when finding
     * distances for indexes on opposite sides of a pentagon.
     *
     * @param origin Index to find the distance from.
     * @param index Index to find the distance to.
     * @return The distance, or a negative number if the library could not
     * compute the distance.
     */
    fn h3Distance(&self, h3: &Self) -> i32 {
        todo!();
        //CoordIJK originIjk, h3Ijk;
        if (h3ToLocalIjk(self, self, &originIjk)) {
            // Currently there are no tests that would cause getting the coordinates
            // for an index the same as the origin to fail.
            return -1; // LCOV_EXCL_LINE
        }
        if (h3ToLocalIjk(self, h3, &h3Ijk)) {
            return -1;
        }

        originIjk.ijkDistance(&h3Ijk)
    }

    /**
     * Number of indexes in a line from the start index to the end index,
     * to be used for allocating memory. Returns a negative number if the
     * line cannot be computed.
     *
     * @param start Start index of the line
     * @param end End index of the line
     * @return Size of the line, or a negative number if the line cannot
     * be computed.
     */
    fn h3LineSize(&self, end: &Self) -> i32 {
        let distance = self.h3Distance(end);
        if distance >= 0 {
            distance + 1
        } else {
            // line can't be computed
            distance
        }
    }

    /**
     * Produces ijk+ coordinates for an index anchored by an origin.
     *
     * The coordinate space used by this function may have deleted
     * regions or warping due to pentagonal distortion.
     *
     * Coordinates are only comparable if they come from the same
     * origin index.
     *
     * Failure may occur if the index is too far away from the origin
     * or if the index is on the other side of a pentagon.
     *
     * @param origin An anchoring index for the ijk+ coordinate system.
     * @param index Index to find the coordinates of
     * @param out ijk+ coordinates of the index will be placed here on success
     * @return 0 on success, or another value on failure.
     */
    fn h3ToLocalIjk(&self, h3: &H3Index) -> (i32, CoordIJK) {
        let res = self.H3_GET_RESOLUTION();

        if (res != h3.H3_GET_RESOLUTION()) {
            return (1, CoordIJK::default());
        }

        let originBaseCell = &self.H3_GET_BASE_CELL();
        let baseCell = h3.H3_GET_BASE_CELL();

        if (originBaseCell < 0 ||  // LCOV_EXCL_BR_LINE
            originBaseCell >= NUM_BASE_CELLS)
        {
            // Base cells less than zero can not be represented in an index
            return (1, CoordIJK::default());
        }
        if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return (1, CoordIJK::default());
        }

        // Direction from origin base cell to index base cell
        let mut dir = Direction::CENTER_DIGIT;
        let mut revDir = Direction::CENTER_DIGIT;
        if (originBaseCell != baseCell) {
            dir = _getBaseCellDirection(originBaseCell, baseCell);
            if (dir == INVALID_DIGIT) {
                // Base cells are not neighbors, can't unfold.
                return (2, CoordIJK::default());
            }
            revDir = _getBaseCellDirection(baseCell, originBaseCell);
            assert!(revDir != INVALID_DIGIT);
        }

        let originOnPent = originBaseCell._isBaseCellPentagon();
        let indexOnPent = baseCell._isBaseCellPentagon();

        if (dir != Direction::CENTER_DIGIT) {
            // Rotate index into the orientation of the origin base cell.
            // cw because we are undoing the rotation into that base cell.
            let baseCellRotations = baseCellNeighbor60CCWRots[originBaseCell][dir];
            if (indexOnPent) {
                for i in 0..baseCellRotations {
                    h3 = _h3RotatePent60cw(h3);

                    revDir = _rotate60cw(revDir);
                    if (revDir == K_AXES_DIGIT) {
                        revDir = _rotate60cw(revDir);
                    }
                }
            } else {
                for i in 0..baseCellRotations {
                    h3._h3Rotate60cw();
                    revDir._rotate60cw();
                }
            }
        }
        // Face is unused. This produces coordinates in base cell coordinate space.
        let indexFijk: FaceIJK = h3._h3ToFaceIjkWithInitializedFijk(&indexFijk);

        if (dir != CENTER_DIGIT) {
            assert!(baseCell != originBaseCell);
            assert!(!(originOnPent && indexOnPent));

            let mut pentagonRotations = 0;
            let mut directionRotations = 0;

            if (originOnPent) {
                let originLeadingDigit = self._h3LeadingNonZeroDigit();

                if (FAILED_DIRECTIONS[originLeadingDigit][dir]) {
                    // TODO: We may be unfolding the pentagon incorrectly in this
                    // case; return an error code until this is guaranteed to be
                    // correct.
                    return (3, CoordIJK::default());
                }

                directionRotations = PENTAGON_ROTATIONS[originLeadingDigit][dir];
                pentagonRotations = directionRotations;
            } else if (indexOnPent) {
                let indexLeadingDigit = _h3LeadingNonZeroDigit(h3);

                if (FAILED_DIRECTIONS[indexLeadingDigit][revDir]) {
                    // TODO: We may be unfolding the pentagon incorrectly in this
                    // case; return an error code until this is guaranteed to be
                    // correct.
                    return (4, CoordIJK::default());
                }

                pentagonRotations = PENTAGON_ROTATIONS[revDir][indexLeadingDigit];
            }

            assert!(pentagonRotations >= 0);
            assert!(directionRotations >= 0);

            for i in 0..pentagonRotations {
                indexFijk.coord._ijkRotate60cw();
            }

            //CoordIJK offset = {0};
            todo!();
            _neighbor(&offset, dir);
            // Scale offset based on resolution
            for r in (r..res).rev() {
                //for (int r = res - 1; r >= 0; r--) {
                if (isResClassIII(r + 1)) {
                    // rotate ccw
                    _downAp7(&offset);
                } else {
                    // rotate cw
                    _downAp7r(&offset);
                }
            }

            for _ in 0..directionRotations {
                _ijkRotate60cw(&offset);
            }

            // Perform necessary translation
            indexFijk.coord += offset;
            indexFijk.coord._ijkNormalize();
        } else if (originOnPent && indexOnPent) {
            // If the origin and index are on pentagon, and we checked that the base
            // cells are the same or neighboring, then they must be the same base
            // cell.
            assert!(baseCell == originBaseCell);

            let originLeadingDigit = self._h3LeadingNonZeroDigit();
            let indexLeadingDigit = h3._h3LeadingNonZeroDigit();

            if (FAILED_DIRECTIONS[originLeadingDigit][indexLeadingDigit]) {
                // TODO: We may be unfolding the pentagon incorrectly in this case;
                // return an error code until this is guaranteed to be correct.
                return (5, CoordIJK::default());
            }

            let withinPentagonRotations = PENTAGON_ROTATIONS[originLeadingDigit][indexLeadingDigit];

            for _ in 0..withinPentagonRotations {
                indexFijk.coord._ijkRotate60cw();
            }
        }

        return (0, indexFijk.coord);
    }

    /**
     * Produces an index for ijk+ coordinates anchored by an origin.
     *
     * The coordinate space used by this function may have deleted
     * regions or warping due to pentagonal distortion.
     *
     * Failure may occur if the coordinates are too far away from the origin
     * or if the index is on the other side of a pentagon.
     *
     * @param origin An anchoring index for the ijk+ coordinate system.
     * @param ijk IJK+ Coordinates to find the index of
     * @param out The index will be placed here on success
     * @return 0 on success, or another value on failure.
     */
    fn localIjkToH3(origin: Self, ijk: &CoordIJK) -> (i32, H3Index) {
        let res = origin.H3_GET_RESOLUTION();
        let originBaseCell = origin.H3_GET_BASE_CELL();
        if (originBaseCell < 0 ||  // LCOV_EXCL_BR_LINE
                originBaseCell >= NUM_BASE_CELLS)
        {
            // Base cells less than zero can not be represented in an index
            return (1, H3Index::default());
        }

        let originOnPent = originBaseCell._isBaseCellPentagon();

        // This logic is very similar to faceIjkToH3
        // initialize the index
        let mut out = H3Index::H3_INIT();
        out.H3_SET_MODE(H3_HEXAGON_MODE);
        out.H3_SET_RESOLUTION(res);

        // check for res 0/base cell
        if (res == 0) {
            if (ijk.i > 1 || ijk.j > 1 || ijk.k > 1) {
                // out of range input
                return (1, out);
            }

            let dir: Direction = ijk._unitIjkToDigit();
            let newBaseCell: i32 = originBaseCell._getBaseCellNeighbor(dir);
            if (newBaseCell == INVALID_BASE_CELL) {
                // Moving in an invalid direction off a pentagon.
                return (1, out);
            }
            out.H3_SET_BASE_CELL(newBaseCell);
            return (0, out);
        }

        // we need to find the correct base cell offset (if any) for this H3 index;
        // start with the passed in base cell and resolution res ijk coordinates
        // in that base cell's coordinate system
        let mut ijkCopy: CoordIJK = ijk.clone();

        // build the H3Index from finest res up
        // adjust r for the fact that the res 0 base cell offsets the indexing
        // digits
        for r in (0..res).rev() {
            let lastIJK: CoordIJK = ijkCopy.clone();
            let lastCenter;
            if (isResClassIII(r + 1)) {
                // rotate ccw
                _upAp7(&ijkCopy);
                lastCenter = ijkCopy;
                _downAp7(&lastCenter);
            } else {
                // rotate cw
                _upAp7r(&ijkCopy);
                lastCenter = ijkCopy;
                _downAp7r(&lastCenter);
            }

            let mut diff: CoordIJK = lastIJK - lastCenter;
            diff._ijkNormalize();

            out.H3_SET_INDEX_DIGIT(r + 1, diff._unitIjkToDigit());
        }

        // ijkCopy should now hold the IJK of the base cell in the
        // coordinate system of the current base cell

        if (ijkCopy.i > 1 || ijkCopy.j > 1 || ijkCopy.k > 1) {
            // out of range input
            return (2, out);
        }

        // lookup the correct base cell
        let dir: Direction = ijkCopy._unitIjkToDigit();
        let baseCell: i32 = originBaseCell._getBaseCellNeighbor(dir);
        // If baseCell is invalid, it must be because the origin base cell is a
        // pentagon, and because pentagon base cells do not border each other,
        // baseCell must not be a pentagon.
        let indexOnPent = if baseCell == INVALID_BASE_CELL {
            false
        } else {
            _isBaseCellPentagon(baseCell)
        };

        if (dir != CENTER_DIGIT) {
            // If the index is in a warped direction, we need to unwarp the base
            // cell direction. There may be further need to rotate the index digits.
            let mut pentagonRotations = 0;
            if (originOnPent) {
                let originLeadingDigit = origin._h3LeadingNonZeroDigit();
                pentagonRotations = PENTAGON_ROTATIONS_REVERSE[originLeadingDigit][dir];
                for _ in 0..pentagonRotations {
                    dir = _rotate60ccw(dir);
                }
                // The pentagon rotations are being chosen so that dir is not the
                // deleted direction. If it still happens, it means we're moving
                // into a deleted subsequence, so there is no index here.
                if (dir == K_AXES_DIGIT) {
                    return (3, out);
                }
                baseCell = _getBaseCellNeighbor(originBaseCell, dir);

                // indexOnPent does not need to be checked again since no pentagon
                // base cells border each other.
                assert!(baseCell != INVALID_BASE_CELL);
                assert!(!_isBaseCellPentagon(baseCell));
            }

            // Now we can determine the relation between the origin and target base
            // cell.
            let baseCellRotations = baseCellNeighbor60CCWRots[originBaseCell][dir];
            assert!(baseCellRotations >= 0);

            // Adjust for pentagon warping within the base cell. The base cell
            // should be in the right location, so now we need to rotate the index
            // back. We might not need to check for errors since we would just be
            // double mapping.
            if (indexOnPent) {
                let revDir: Direction = _getBaseCellDirection(baseCell, originBaseCell);
                assert!(revDir != INVALID_DIGIT);

                // Adjust for the different coordinate space in the two base cells.
                // This is done first because we need to do the pentagon rotations
                // based on the leading digit in the pentagon's coordinate system.
                for _ in 0..baseCellRotations {
                    out = _h3Rotate60ccw(*out);
                }

                let indexLeadingDigit: Direction = out._h3LeadingNonZeroDigit();
                if (baseCell._isBaseCellPolarPentagon()) {
                    pentagonRotations = PENTAGON_ROTATIONS_REVERSE_POLAR[revDir][indexLeadingDigit];
                } else {
                    pentagonRotations =
                        PENTAGON_ROTATIONS_REVERSE_NONPOLAR[revDir][indexLeadingDigit];
                }

                assert!(pentagonRotations >= 0);
                for _ in 0..pentagonRotations {
                    out = _h3RotatePent60ccw(*out);
                }
            } else {
                assert!(pentagonRotations >= 0);
                for _ in 0..pentagonRotations {
                    out = _h3Rotate60ccw(*out);
                }

                // Adjust for the different coordinate space in the two base cells.
                for _ in 0..baseCellRotations {
                    out = _h3Rotate60ccw(*out);
                }
            }
        } else if (originOnPent && indexOnPent) {
            let originLeadingDigit = origin._h3LeadingNonZeroDigit();
            let indexLeadingDigit = out._h3LeadingNonZeroDigit();

            let withinPentagonRotations =
                PENTAGON_ROTATIONS_REVERSE[originLeadingDigit][indexLeadingDigit];
            assert!(withinPentagonRotations >= 0);

            for _ in 0..withinPentagonRotations {
                *out = _h3Rotate60ccw(*out);
            }
        }

        if (indexOnPent) {
            // TODO: There are cases in h3ToLocalIjk which are failed but not
            // accounted for here - instead just fail if the recovered index is
            // invalid.
            if (out._h3LeadingNonZeroDigit() == K_AXES_DIGIT) {
                return (4, out);
            }
        }

        out.H3_SET_BASE_CELL(baseCell);
        return (0, out);
    }

    /**
     * Produces ij coordinates for an index anchored by an origin.
     *
     * The coordinate space used by this function may have deleted
     * regions or warping due to pentagonal distortion.
     *
     * Coordinates are only comparable if they come from the same
     * origin index.
     *
     * Failure may occur if the index is too far away from the origin
     * or if the index is on the other side of a pentagon.
     *
     * This function is experimental, and its output is not guaranteed
     * to be compatible across different versions of H3.
     *
     * @param origin An anchoring index for the ij coordinate system.
     * @param index Index to find the coordinates of
     * @param out ij coordinates of the index will be placed here on success
     * @return 0 on success, or another value on failure.
     */
    fn experimentalH3ToLocalIj(origin: &Self, h3: &Self) -> (i32, CoordIJ) {
        // This function is currently experimental. Once ready to be part of the
        // non-experimental API, this function (with the experimental prefix) will
        // be marked as deprecated and to be removed in the next major version. It
        // will be replaced with a non-prefixed function name.
        todo!()
        /*
        CoordIJK ijk;
        int failed = h3ToLocalIjk(origin, h3, &ijk);
        if (failed) {
        return failed;
        }

        let out = ijk.ijkToIj(out);

        return 0;
        */
    }

    /**
     * Produces an index for ij coordinates anchored by an origin.
     *
     * The coordinate space used by this function may have deleted
     * regions or warping due to pentagonal distortion.
     *
     * Failure may occur if the index is too far away from the origin
     * or if the index is on the other side of a pentagon.
     *
     * This function is experimental, and its output is not guaranteed
     * to be compatible across different versions of H3.
     *
     * @param origin An anchoring index for the ij coordinate system.
     * @param out ij coordinates to index.
     * @param index Index will be placed here on success.
     * @return 0 on success, or another value on failure.
     */
    fn experimentalLocalIjToH3(origin: &H3Index, ij: &CoordIJ) {
        // This function is currently experimental. Once ready to be part of the
        // non-experimental API, this function (with the experimental prefix) will
        // be marked as deprecated and to be removed in the next major version. It
        // will be replaced with a non-prefixed function name.
        todo!()
        /*
        CoordIJK ijk;
        ijToIjk(ij, &ijk);

        return localIjkToH3(origin, &ijk, out);
        */
    }

    /**
     * Given two H3 indexes, return the line of indexes between them (inclusive).
     *
     * This function may fail to find the line between two indexes, for
     * example if they are very far apart. It may also fail when finding
     * distances for indexes on opposite sides of a pentagon.
     *
     * Notes:
     *
     *  - The specific output of this function should not be considered stable
     *    across library versions. The only guarantees the library provides are
     *    that the line length will be `h3Distance(start, end) + 1` and that
     *    every index in the line will be a neighbor of the preceding index.
     *  - Lines are drawn in grid space, and may not correspond exactly to either
     *    Cartesian lines or great arcs.
     *
     * @param start Start index of the line
     * @param end End index of the line
     * @param out Output array, which must be of size h3LineSize(start, end)
     * @return 0 on success, or another value on failure.
     */
    fn h3Line(&self, end: &H3Index) -> (i32, Vec<H3Index>) {
        let distance = start.h3Distance(end);
        // Early exit if we can't calculate the line
        if (distance < 0) {
            return (distance, Vec::new());
        }

        // Get IJK coords for the start and end. We've already confirmed
        // that these can be calculated with the distance check above.

        // Convert H3 addresses to IJK coords
        let mut startIjk = start.h3ToLocalIjk(start);
        let mut endIjk = start.h3ToLocalIjk(end);

        // Convert IJK to cube coordinates suitable for linear interpolation
        startIjk.ijkToCube();
        endIjk.ijkToCube();

        let (iStep, jStep, kStep) = if distance > 0 {
            let d = distance as f64;

            (
                (endIjk.i - startIjk.i) as f64 / d,
                (endIjk.j - startIjk.j) as f64 / d,
                (endIjk.k - startIjk.k) as f64 / d,
            )
        } else {
            (0., 0., 0.)
        };

        let currentIjk = CoordIjK::new(startIjk.i, startIjk.j, startIjk.k);
        let mut out = Vec::new();
        for n in 0..=distance {
            currentIjk.cubeRound(
                startIjk.i as f64 + iStep * n as f64,
                startIjk.j as f64 + jStep * n as f64,
                startIjk.k as f64 + kStep * n as f64,
            );
            // Convert cube -> ijk -> h3 index
            cubeToIjk(&currentIjk);
            out.push(start.localIjkToH3(&currentIjk));
        }

        return (0, out);
    }
}

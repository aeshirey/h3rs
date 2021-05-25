use std::ops::Add;

use crate::{
    basecell::baseCellNeighbor60CCWRots, coordij::CoordIJ, coordijk::CoordIJK, faceijk::FaceIJK,
    BaseCell, Direction, Resolution,
};

use super::H3Index;

/// Origin leading digit -> index leading digit -> rotations 60 cw
/// Either being 1 (K axis) is invalid.
/// No good default at 0.
const PENTAGON_ROTATIONS: [[i32; 7]; 7] = [
    [0, -1, 0, 0, 0, 0, 0],       // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, -1, 0, 0, 0, 1, 0],       // 2
    [0, -1, 0, 0, 1, 1, 0],       // 3
    [0, -1, 0, 5, 0, 0, 0],       // 4
    [0, -1, 5, 5, 0, 0, 0],       // 5
    [0, -1, 0, 0, 0, 0, 0],       // 6
];

/// Reverse base cell direction -> leading index digit -> rotations 60 ccw.
/// For reversing the rotation introduced in PENTAGON_ROTATIONS when
/// the origin is on a pentagon (regardless of the base cell of the index.)
const PENTAGON_ROTATIONS_REVERSE: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],        // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, 1, 0, 0, 0, 0, 0],        // 2
    [0, 1, 0, 0, 0, 1, 0],        // 3
    [0, 5, 0, 0, 0, 0, 0],        // 4
    [0, 5, 0, 5, 0, 0, 0],        // 5
    [0, 0, 0, 0, 0, 0, 0],        // 6
];

/// Reverse base cell direction -> leading index digit -> rotations 60 ccw.
/// For reversing the rotation introduced in PENTAGON_ROTATIONS when the index is
/// on a pentagon and the origin is not.
const PENTAGON_ROTATIONS_REVERSE_NONPOLAR: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],        // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, 1, 0, 0, 0, 0, 0],        // 2
    [0, 1, 0, 0, 0, 1, 0],        // 3
    [0, 5, 0, 0, 0, 0, 0],        // 4
    [0, 1, 0, 5, 1, 1, 0],        // 5
    [0, 0, 0, 0, 0, 0, 0],        // 6
];

/// Reverse base cell direction -> leading index digit -> rotations 60 ccw.
/// For reversing the rotation introduced in PENTAGON_ROTATIONS when the index is
/// on a polar pentagon and the origin is not.
const PENTAGON_ROTATIONS_REVERSE_POLAR: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],        // 0
    [-1, -1, -1, -1, -1, -1, -1], // 1
    [0, 1, 1, 1, 1, 1, 1],        // 2
    [0, 1, 0, 0, 0, 1, 0],        // 3
    [0, 1, 0, 0, 1, 1, 1],        // 4
    [0, 1, 0, 5, 1, 1, 0],        // 5
    [0, 1, 1, 0, 1, 1, 1],        // 6
];

/// Prohibited directions when unfolding a pentagon.
///
/// Indexes by two directions, both relative to the pentagon base cell. The first
/// is the direction of the origin index and the second is the direction of the
/// index to unfold. Direction refers to the direction from base cell to base
/// cell if the indexes are on different base cells, or the leading digit if
/// within the pentagon base cell.
///
/// This previously included a Class II/Class III check but these were removed
/// due to failure cases. It's possible this could be restricted to a narrower
/// set of a failure cases. Currently, the logic is any unfolding across more
/// than one icosahedron face is not permitted.
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
    pub fn h3Distance(&self, h3: &Self) -> Result<i32, ()> {
        // Currently there are no tests that would cause getting the coordinates
        // for an index the same as the origin to fail.
        let originIjk = self.h3ToLocalIjk(self).map_err(|_| ())?;

        let h3Ijk = self.h3ToLocalIjk(h3).map_err(|_| ())?;

        Ok(originIjk.ijkDistance(&h3Ijk))
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
    pub(crate) fn h3ToLocalIjk(&self /*origin*/, h3: &Self) -> Result<CoordIJK, i32> {
        let mut h3 = *h3;

        let res = self.get_resolution();

        if res != h3.get_resolution() {
            return Err(1);
        }

        let originBaseCell = self.get_base_cell();
        let baseCell = h3.get_base_cell();

        if originBaseCell < 0 ||  // LCOV_EXCL_BR_LINE
        originBaseCell >= BaseCell::NUM_BASE_CELLS
        {
            // Base cells less than zero can not be represented in an index
            return Err(1);
        }
        if baseCell < 0 || baseCell >= BaseCell::NUM_BASE_CELLS {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return Err(1);
        }

        // Direction from origin base cell to index base cell
        let mut dir = Direction::CENTER_DIGIT;
        let mut revDir = Direction::CENTER_DIGIT;

        if originBaseCell != baseCell {
            dir = originBaseCell._getBaseCellDirection(baseCell);
            if dir == Direction::INVALID_DIGIT {
                // Base cells are not neighbors, can't unfold.
                return Err(2);
            }
            revDir = baseCell._getBaseCellDirection(originBaseCell);
            assert!(revDir != Direction::INVALID_DIGIT);
        }

        let originOnPent = originBaseCell._isBaseCellPentagon();
        let indexOnPent = baseCell._isBaseCellPentagon();

        if dir != Direction::CENTER_DIGIT {
            // Rotate index into the orientation of the origin base cell.
            // cw because we are undoing the rotation into that base cell.
            let baseCellRotations =
                baseCellNeighbor60CCWRots[originBaseCell.0 as usize][dir as usize];
            if indexOnPent {
                for _ in 0..baseCellRotations.into() {
                    h3 = h3._h3RotatePent60cw();

                    revDir = revDir.rotate60cw();
                    if revDir == Direction::K_AXES_DIGIT {
                        revDir = revDir.rotate60cw();
                    }
                }
            } else {
                for _ in 0..baseCellRotations.0 {
                    h3 = h3._h3Rotate60cw();

                    revDir = revDir.rotate60cw();
                }
            }
        }

        // Face is unused. This produces coordinates in base cell coordinate space.
        let mut indexFijk = FaceIJK::default();
        h3._h3ToFaceIjkWithInitializedFijk(&mut indexFijk);

        if dir != Direction::CENTER_DIGIT {
            assert!(baseCell != originBaseCell);
            assert!(!(originOnPent && indexOnPent));

            let mut pentagonRotations = 0;
            let mut directionRotations = 0;

            if originOnPent {
                let originLeadingDigit = self._h3LeadingNonZeroDigit() as usize;

                if FAILED_DIRECTIONS[originLeadingDigit][dir as usize] {
                    // TODO: We may be unfolding the pentagon incorrectly in this
                    // case; return an error code until this is guaranteed to be
                    // correct.
                    return Err(3);
                }

                directionRotations = PENTAGON_ROTATIONS[originLeadingDigit][dir as usize];
                pentagonRotations = directionRotations;
            } else if indexOnPent {
                let indexLeadingDigit = h3._h3LeadingNonZeroDigit();

                if FAILED_DIRECTIONS[indexLeadingDigit as usize][revDir as usize] {
                    // TODO: We may be unfolding the pentagon incorrectly in this
                    // case; return an error code until this is guaranteed to be
                    // correct.
                    return Err(4);
                }

                pentagonRotations = PENTAGON_ROTATIONS[revDir as usize][indexLeadingDigit as usize];
            }

            assert!(pentagonRotations >= 0);
            assert!(directionRotations >= 0);

            for _ in 0..pentagonRotations {
                indexFijk.coord._ijkRotate60cw();
            }

            let mut offset = CoordIJK::default();
            offset._neighbor(dir);

            // Scale offset based on resolution
            for r in (0..res.into()).rev() {
                let r: Resolution = (r + 1).into();
                if r.isResClassIII() {
                    // rotate ccw
                    offset._downAp7();
                } else {
                    // rotate cw
                    offset._downAp7r();
                }
            }

            for _ in 0..directionRotations {
                offset._ijkRotate60cw();
            }

            // Perform necessary translation
            indexFijk.coord += offset;
            indexFijk.coord.normalize();
        } else if originOnPent && indexOnPent {
            // If the origin and index are on pentagon, and we checked that the base
            // cells are the same or neighboring, then they must be the same base
            // cell.
            assert!(baseCell == originBaseCell);

            let originLeadingDigit = self._h3LeadingNonZeroDigit();
            let indexLeadingDigit = h3._h3LeadingNonZeroDigit();

            if FAILED_DIRECTIONS[originLeadingDigit as usize][indexLeadingDigit as usize] {
                // TODO: We may be unfolding the pentagon incorrectly in this case;
                // return an error code until this is guaranteed to be correct.
                return Err(5);
            }

            let withinPentagonRotations =
                PENTAGON_ROTATIONS[originLeadingDigit as usize][indexLeadingDigit as usize];

            for _ in 0..withinPentagonRotations {
                indexFijk.coord._ijkRotate60cw();
            }
        }

        Ok(indexFijk.coord)
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
    pub fn h3LineSize(start: &Self, end: &Self) -> Result<i32, ()> {
        let distance = start.h3Distance(end)?;
        Ok(distance + 1)
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
    pub fn experimentalLocalIjToH3(&self, ij: &CoordIJ) -> Result<Self, i32> {
        // This function is currently experimental. Once ready to be part of the
        // non-experimental API, this function (with the experimental prefix) will
        // be marked as deprecated and to be removed in the next major version. It
        // will be replaced with a non-prefixed function name.
        let ijk: CoordIJK = ij.into();

        self.localIjkToH3(&ijk)
    }

    /// Produces an index for ijk+ coordinates anchored by an origin.
    ///
    /// The coordinate space used by this function may have deleted
    /// regions or warping due to pentagonal distortion.
    ///
    /// Failure may occur if the coordinates are too far away from the origin
    /// or if the index is on the other side of a pentagon.
    fn localIjkToH3(&self, ijk: &CoordIJK) -> Result<Self, i32> {
        let res = self.get_resolution();
        let originBaseCell = self.get_base_cell();

        if i32::from(originBaseCell) < 0 || usize::from(originBaseCell) >= BaseCell::NUM_BASE_CELLS
        {
            // Base cells less than zero can not be represented in an index
            return Err(1);
        }

        let originOnPent = originBaseCell._isBaseCellPentagon();

        // This logic is very similar to faceIjkToH3
        // initialize the index
        let mut out = H3Index::H3_INIT;
        out.set_mode(super::H3Mode::H3_HEXAGON_MODE);
        out.set_resolution(res);

        // check for res 0/base cell
        if res == Resolution::R0 {
            if ijk.i > 1 || ijk.j > 1 || ijk.k > 1 {
                // out of range input
                return Err(1);
            }

            let dir: Direction = ijk._unitIjkToDigit();
            let new_basecell = originBaseCell._getBaseCellNeighbor(&dir);
            if new_basecell == BaseCell::INVALID {
                // Moving in an invalid direction off a pentagon.
                return Err(1);
            }
            out.set_base_cell(new_basecell);
            return Ok(out);
        }

        // we need to find the correct base cell offset (if any) for this H3 index;
        // start with the passed in base cell and resolution res ijk coordinates
        // in that base cell's coordinate system
        let mut ijkCopy = ijk.clone();

        // build the H3Index from finest res up
        // adjust r for the fact that the res 0 base cell offsets the indexing
        // digits
        for r in (0..res as u64).rev() {
            let r: Resolution = r.into();
            let last_ijk = ijkCopy.clone();
            let last_center: CoordIJK = if (r + 1).isResClassIII() {
                // rotate ccw
                ijkCopy._upAp7();
                let mut lc = ijkCopy.clone();
                lc._downAp7();
                lc
            } else {
                // rotate cw
                ijkCopy._upAp7r();
                let mut lc = ijkCopy.clone();
                lc._downAp7r();
                lc
            };

            let mut diff = last_ijk - last_center;
            diff.normalize();

            let digit: Direction = diff._unitIjkToDigit();

            out.set_index_digit(r.into(), digit.into());
        }

        // ijkCopy should now hold the IJK of the base cell in the
        // coordinate system of the current base cell

        if ijkCopy.i > 1 || ijkCopy.j > 1 || ijkCopy.k > 1 {
            // out of range input
            return Err(2);
        }

        // lookup the correct base cell
        let mut dir = ijkCopy._unitIjkToDigit();
        let mut basecell = originBaseCell._getBaseCellNeighbor(&dir);

        // If baseCell is invalid, it must be because the origin base cell is a
        // pentagon, and because pentagon base cells do not border each other,
        // baseCell must not be a pentagon.
        let indexOnPent = if basecell == BaseCell::INVALID {
            false
        } else {
            basecell._isBaseCellPentagon()
        };

        if dir != Direction::CENTER_DIGIT {
            // If the index is in a warped direction, we need to unwarp the base
            // cell direction. There may be further need to rotate the index digits.
            let mut pentagon_rotations = 0;
            if originOnPent {
                let origin_leading_digit: usize = self._h3LeadingNonZeroDigit().into();
                pentagon_rotations =
                    PENTAGON_ROTATIONS_REVERSE[origin_leading_digit as usize][dir as usize];
                for _ in 0..pentagon_rotations {
                    dir = dir.rotate60ccw();
                }

                // The pentagon rotations are being chosen so that dir is not the
                // deleted direction. If it still happens, it means we're moving
                // into a deleted subsequence, so there is no index here.
                if dir == Direction::K_AXES_DIGIT {
                    return Err(3);
                }
                basecell = originBaseCell._getBaseCellNeighbor(&dir);

                // indexOnPent does not need to be checked again since no pentagon
                // base cells border each other.
                assert!(basecell != BaseCell::INVALID);
                assert!(basecell._isBaseCellPentagon());
            }

            // Now we can determine the relation between the origin and target base cell.
            let base_cell_rotations =
                baseCellNeighbor60CCWRots[usize::from(originBaseCell)][dir as usize];
            //assert!(baseCellRotations.into() >= 0);

            // Adjust for pentagon warping within the base cell. The base cell
            // should be in the right location, so now we need to rotate the index
            // back. We might not need to check for errors since we would just be
            // double mapping.
            if indexOnPent {
                let revDir = basecell._getBaseCellDirection(originBaseCell);
                assert!(revDir != Direction::INVALID_DIGIT);

                // Adjust for the different coordinate space in the two base cells.
                // This is done first because we need to do the pentagon rotations
                // based on the leading digit in the pentagon's coordinate system.
                for _ in 0..base_cell_rotations.into() {
                    out = out._h3Rotate60ccw();
                }

                let indexLeadingDigit = out._h3LeadingNonZeroDigit();
                pentagon_rotations = if basecell._isBaseCellPolarPentagon() {
                    PENTAGON_ROTATIONS_REVERSE_POLAR[revDir as usize][indexLeadingDigit as usize]
                } else {
                    PENTAGON_ROTATIONS_REVERSE_NONPOLAR[revDir as usize][indexLeadingDigit as usize]
                };

                assert!(pentagon_rotations >= 0);
                for _ in 0..pentagon_rotations {
                    out = out._h3RotatePent60ccw();
                }
            } else {
                assert!(pentagon_rotations >= 0);
                for _ in 0..pentagon_rotations {
                    out = out._h3Rotate60ccw();
                }

                // Adjust for the different coordinate space in the two base cells.
                for _ in 0..base_cell_rotations.into() {
                    out = out._h3Rotate60ccw();
                }
            }
        } else if originOnPent && indexOnPent {
            let originLeadingDigit = self._h3LeadingNonZeroDigit();
            let indexLeadingDigit = out._h3LeadingNonZeroDigit();

            let withinPentagonRotations =
                PENTAGON_ROTATIONS_REVERSE[originLeadingDigit as usize][indexLeadingDigit as usize];
            assert!(withinPentagonRotations >= 0);

            for _ in 0..withinPentagonRotations {
                out = out._h3Rotate60ccw();
            }
        }

        if indexOnPent {
            // TODO: There are cases in h3ToLocalIjk which are failed but not
            // accounted for here - instead just fail if the recovered index is
            // invalid.
            if out._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                return Err(4);
            }
        }

        out.set_base_cell(basecell);
        Ok(out)
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

    pub fn h3Line(start: Self, end: Self) -> Result<Vec<H3Index>, ()> {
        // Early exit if we can't calculate the line
        let distance = start.h3Distance(&end)?;

        // Get IJK coords for the start and end. We've already confirmed
        // that these can be calculated with the distance check above.
        let mut startIjk = start.h3ToLocalIjk(&start).unwrap();
        let mut endIjk = start.h3ToLocalIjk(&end).unwrap();

        // Convert IJK to cube coordinates suitable for linear interpolation
        startIjk.ijkToCube();
        endIjk.ijkToCube();

        let iStep = if distance > 0 {
            (endIjk.i - startIjk.i) as f32 / distance as f32
        } else {
            0.0
        };
        let jStep = if distance > 0 {
            (endIjk.j - startIjk.j) as f32 / distance as f32
        } else {
            0.0
        };
        let kStep = if distance > 0 {
            (endIjk.k - startIjk.k) as f32 / distance as f32
        } else {
            0.0
        };

        let mut currentIjk = startIjk;

        let mut result = Vec::with_capacity(distance as usize + 1);

        for n in 0..=distance {
            let mut currentIjk = Self::cubeRound(
                startIjk.i as f32 + iStep * n as f32,
                startIjk.j as f32 + jStep * n as f32,
                startIjk.k as f32 + kStep * n as f32,
            );

            // Convert cube -> ijk -> h3 index
            currentIjk.cubeToIjk();

            result.push(start.localIjkToH3(&currentIjk).unwrap());
        }

        Ok(result)
    }

    fn cubeRound(i: f32, j: f32, k: f32) -> CoordIJK {
        let mut ri = i.round() as i32;
        let mut rj = j.round() as i32;
        let mut rk = k.round() as i32;

        let iDiff = (ri as f32 - i).abs();
        let jDiff = (rj as f32 - j).abs();
        let kDiff = (rk as f32 - k).abs();

        // Round, maintaining valid cube coords
        if iDiff > jDiff && iDiff > kDiff {
            ri = -rj - rk;
        } else if jDiff > kDiff {
            rj = -ri - rk;
        } else {
            rk = -ri - rj;
        }

        CoordIJK::new(ri as i32, rj as i32, rk as i32)
    }

    /*
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
    //*
    fn h3ToLocalIjk__newversion(origin: H3Index, mut h3: H3Index) -> Result<CoordIJK, i32> {
        let res = origin.get_resolution();

        if res != h3.get_resolution() {
            return Err(1);
        }

        let originBaseCell = origin.get_base_cell();
        let baseCell = h3.get_base_cell();

        if originBaseCell >= BaseCell::NUM_BASE_CELLS {
            return Err(1);
        }

        if baseCell >= BaseCell::NUM_BASE_CELLS {
            // Base cells less than zero can not be represented in an index
            return Err(1);
        }

        // Direction from origin base cell to index base cell
        let mut dir = Direction::CENTER_DIGIT;
        let mut revDir = Direction::CENTER_DIGIT;

        if originBaseCell != baseCell {
            dir = originBaseCell._getBaseCellDirection(baseCell);
            if dir == Direction::INVALID_DIGIT {
                // Base cells are not neighbors, can't unfold.
                return Err(2);
            }
            revDir = baseCell._getBaseCellDirection(originBaseCell);
            assert_ne!(revDir, Direction::INVALID_DIGIT);
        }

        let originOnPent = originBaseCell._isBaseCellPentagon();
        let indexOnPent = baseCell._isBaseCellPentagon();

        if dir != Direction::CENTER_DIGIT {
            // Rotate index into the orientation of the origin base cell.
            // cw because we are undoing the rotation into that base cell.
            let baseCellRotations =
                baseCellNeighbor60CCWRots[originBaseCell.0 as usize][dir as usize];
            if indexOnPent {
                for _ in 0..baseCellRotations.0 {
                    h3 = h3._h3RotatePent60cw();

                    revDir = revDir.rotate60cw();
                    if revDir == Direction::K_AXES_DIGIT {
                        revDir = revDir.rotate60cw();
                    }
                }
            } else {
                for _ in 0..baseCellRotations.0 {
                    h3 = h3._h3Rotate60cw();

                    revDir = revDir.rotate60cw();
                }
            }
        }

        // Face is unused. This produces coordinates in base cell coordinate space.
        let mut indexFijk = FaceIJK::default();
        h3._h3ToFaceIjkWithInitializedFijk(&mut indexFijk);

        if dir != Direction::CENTER_DIGIT {
            assert!(baseCell != originBaseCell);
            assert!(!(originOnPent && indexOnPent));

            let mut pentagonRotations = 0;
            let mut directionRotations = 0;

            if originOnPent {
                let originLeadingDigit = origin._h3LeadingNonZeroDigit();

                if FAILED_DIRECTIONS[originLeadingDigit as usize][dir as usize] {
                    // TODO: We may be unfolding the pentagon incorrectly in this
                    // case; return an error code until this is guaranteed to be
                    // correct.
                    return Err(3);
                }

                directionRotations = PENTAGON_ROTATIONS[originLeadingDigit as usize][dir as usize];
                pentagonRotations = directionRotations;
            } else if indexOnPent {
                let indexLeadingDigit = h3._h3LeadingNonZeroDigit();

                if FAILED_DIRECTIONS[indexLeadingDigit as usize][revDir as usize] {
                    // TODO: We may be unfolding the pentagon incorrectly in this
                    // case; return an error code until this is guaranteed to be
                    // correct.
                    return Err(4);
                }

                pentagonRotations = PENTAGON_ROTATIONS[revDir as usize][indexLeadingDigit as usize];
            }

            assert!(pentagonRotations >= 0);
            assert!(directionRotations >= 0);

            for _ in 0..pentagonRotations {
                indexFijk.coord._ijkRotate60cw();
            }

            let mut offset = CoordIJK::default();
            offset._neighbor(dir);

            // Scale offset based on resolution
            for r in (0..res as usize).rev() {
                let r1: Resolution = (r + 1).into();

                if r1.isResClassIII() {
                    // rotate ccw
                    offset._downAp7();
                } else {
                    // rotate cw
                    offset._downAp7r();
                }
            }

            for _ in 0..directionRotations {
                offset._ijkRotate60cw();
            }

            // Perform necessary translation
            indexFijk.coord += offset;
            indexFijk.coord.normalize();
            //_ijkAdd(&indexFijk.coord, &offset, &indexFijk.coord);
            //_ijkNormalize(&indexFijk.coord);
        } else if originOnPent && indexOnPent {
            // If the origin and index are on pentagon, and we checked that the base
            // cells are the same or neighboring, then they must be the same base
            // cell.
            assert_eq!(baseCell, originBaseCell);

            let originLeadingDigit = origin._h3LeadingNonZeroDigit();
            let indexLeadingDigit = h3._h3LeadingNonZeroDigit();

            if FAILED_DIRECTIONS[originLeadingDigit as usize][indexLeadingDigit as usize] {
                // TODO: We may be unfolding the pentagon incorrectly in this case;
                // return an error code until this is guaranteed to be correct.
                return Err(5);
            }

            let withinPentagonRotations =
                PENTAGON_ROTATIONS[originLeadingDigit as usize][indexLeadingDigit as usize];

            for _ in 0..withinPentagonRotations {
                indexFijk.coord._ijkRotate60cw();
            }
        }

        Ok(indexFijk.coord)
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
    pub fn experimentalH3ToLocalIj(origin: H3Index, h3: H3Index) -> Result<CoordIJ, i32> {
        // This function is currently experimental. Once ready to be part of the
        // non-experimental API, this function (with the experimental prefix) will
        // be marked as deprecated and to be removed in the next major version. It
        // will be replaced with a non-prefixed function name.
        origin.h3ToLocalIjk(&h3).map(|ijk| ijk.ijkToIj())

        //let ijk = origin.h3ToLocalIjk(&h3)?;
        //let out = ijk.ijkToIj();
        //Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h3Line_acrossMultipleFaces() {
        let start = H3Index(0x85285aa7fffffff);
        let end = H3Index(0x851d9b1bfffffff);

        let lineSz = H3Index::h3LineSize(&start, &end);
        assert!(
            lineSz.is_err(),
            "Line not computable across multiple icosa faces"
        );
    }

    const MAX_DISTANCES: [i32; 6] = [1, 2, 5, 12, 19, 26];

    // The same traversal constants from algos.c (for hexRange) here reused as local IJ vectors.
    const DIRECTIONS: [CoordIJ; 6] = [
        CoordIJ::new(0, 1),
        CoordIJ::new(-1, 0),
        CoordIJ::new(-1, -1),
        CoordIJ::new(0, -1),
        CoordIJ::new(1, 0),
        CoordIJ::new(1, 1),
    ];
    const NEXT_RING_DIRECTION: CoordIJ = CoordIJ::new(1, 0);

    /// Property-based testing of h3Line output
    fn h3Line_assertions(start: H3Index, end: H3Index) {
        /*
        int sz = H3_EXPORT(h3LineSize)(start, end);
        t_assert(sz > 0, "got valid size");
        H3Index *line = calloc(sz, sizeof(H3Index));

        int err = H3_EXPORT(h3Line)(start, end, line);

        t_assert(err == 0, "no error on line");
        t_assert(line[0] == start, "line starts with start index");
        t_assert(line[sz - 1] == end, "line ends with end index");

        for (int i = 1; i < sz; i++) {
            t_assert(H3_EXPORT(h3IsValid)(line[i]), "index is valid");
            t_assert(H3_EXPORT(h3IndexesAreNeighbors)(line[i], line[i - 1]),
                     "index is a neighbor of the previous index");
            if (i > 1) {
                t_assert(
                    !H3_EXPORT(h3IndexesAreNeighbors)(line[i], line[i - 2]),
                    "index is not a neighbor of the index before the previous");
            }
        }

        free(line);
        */
    }

    /// Tests for invalid h3Line input
    fn h3Line_invalid_assertions(start: H3Index, end: H3Index) {
        let sz = H3Index::h3LineSize(&start, &end);
        assert!(sz.is_err(), "line size marked as invalid");

        let line = H3Index::h3Line(start, end);
        assert!(line.is_err(), "line marked as invalid");
    }

    /// Test for lines from an index to all neighbors within a kRing
    fn h3Line_kRing_assertions(h3: H3Index) {
        /*
        int r = H3_GET_RESOLUTION(h3);
        t_assert(r <= 5, "resolution supported by test function (kRing)");
        int maxK = MAX_DISTANCES[r];

        int sz = H3_EXPORT(maxKringSize)(maxK);

        if (H3_EXPORT(h3IsPentagon)(h3)) {
            return;
        }

        H3Index *neighbors = calloc(sz, sizeof(H3Index));
        H3_EXPORT(kRing)(h3, maxK, neighbors);

        for (int i = 0; i < sz; i++) {
            if (neighbors[i] == 0) {
                continue;
            }
            int distance = H3_EXPORT(h3Distance)(h3, neighbors[i]);
            if (distance >= 0) {
                h3Line_assertions(h3, neighbors[i]);
            } else {
                h3Line_invalid_assertions(h3, neighbors[i]);
            }
        }

        free(neighbors);
        */
    }

    #[test]
    fn h3Line_kRing() {
        //iterateAllIndexesAtRes(0, h3Line_kRing_assertions);
        //iterateAllIndexesAtRes(1, h3Line_kRing_assertions);
        //iterateAllIndexesAtRes(2, h3Line_kRing_assertions);
        // Don't iterate all of res 3, to save time
        //iterateAllIndexesAtResPartial(3, h3Line_kRing_assertions, 6);
        // Further resolutions aren't tested to save time.
    }

    fn setup() -> (H3Index, H3Index, H3Index, H3Index) {
        let bc1 = H3Index::setH3Index(Resolution::R0, BaseCell(15), Direction::CENTER_DIGIT);
        let bc2 = H3Index::setH3Index(Resolution::R0, BaseCell(8), Direction::CENTER_DIGIT);
        let bc3 = H3Index::setH3Index(Resolution::R0, BaseCell(31), Direction::CENTER_DIGIT);
        let pent1 = H3Index::setH3Index(Resolution::R0, BaseCell(4), Direction::CENTER_DIGIT);

        (bc1, bc2, bc3, pent1)
    }

    #[test]
    fn ijkBaseCells1() {
        let (bc1, bc2, bc3, pent1) = setup();

        let ijk = pent1.h3ToLocalIjk(&bc1);
        assert!(ijk.is_ok(), "got ijk for base cells 4 and 15");
        //assert_eq!(ijk.unwrpa(), &UNIT_VECS[2]) == 1, "neighboring base cell at 0,1,0");
    }

    #[test]
    fn ijBaseCells2() {
        let mut ij = CoordIJ::default();
        let origin = H3Index(0x8029fffffffffff);
        let retrieved = origin.experimentalLocalIjToH3(&ij);
        assert!(retrieved.is_ok(), "got origin back");
        assert_eq!(
            retrieved.unwrap().0,
            0x8029fffffffffff,
            "origin matches self"
        );
        ij.i = 1;
        let retrieved = origin.experimentalLocalIjToH3(&ij);
        assert!(retrieved.is_ok(), "got offset index");
        assert_eq!(
            retrieved.unwrap().0,
            0x8051fffffffffff,
            "modified index matches expected"
        );

        ij.i = 2;
        let retrieved = origin.experimentalLocalIjToH3(&ij);
        assert!(retrieved.is_ok(), "out of range base cell (1)");

        ij.i = 0;
        ij.j = 2;
        let retrieved = origin.experimentalLocalIjToH3(&ij);
        assert!(retrieved.is_ok(), "out of range base cell (2)");

        ij.i = -2;
        ij.j = -2;
        let retrieved = origin.experimentalLocalIjToH3(&ij);
        assert!(retrieved.is_ok(), "out of range base cell (3)");
    }

    #[test]
    fn ijOutOfRange() {
        const numCoords: usize = 7;
        const coords: [CoordIJ; numCoords] = [
            CoordIJ::new(0, 0),
            CoordIJ::new(1, 0),
            CoordIJ::new(2, 0),
            CoordIJ::new(3, 0),
            CoordIJ::new(4, 0),
            CoordIJ::new(-4, 0),
            CoordIJ::new(0, 4),
        ];
        const expected: [H3Index; numCoords] = [
            H3Index(0x81283ffffffffff),
            H3Index(0x81293ffffffffff),
            H3Index(0x8150bffffffffff),
            H3Index(0x8151bffffffffff),
            H3Index::H3_NULL,
            H3Index::H3_NULL,
            H3Index::H3_NULL,
        ];

        for (ex, coord) in expected.iter().zip(coords.iter()) {
            let result = expected[0].experimentalLocalIjToH3(coord);
            if *ex == H3Index::H3_NULL {
                assert!(result.is_err(), "coordinates are out of range");
            } else {
                assert!(result.is_ok(), "coordinates in range");
                // >>> bin(581703223744659455) <- left
                // '0b100000010010100111111111111111111111111111111111111111111111'
                // >>> bin(581672437419081727) <- right
                // '0b100000010010100000111111111111111111111111111111111111111111'
                assert_eq!(result.unwrap(), *ex, "result matches expectations");
            }
        }
    }

    #[test]
    fn experimentalH3ToLocalIjFailed() {
        let (bc1, bc2, bc3, pent1) = setup();

        let ij = H3Index::experimentalH3ToLocalIj(bc1, bc1);
        assert!(ij.is_ok(), "found IJ (1)");
        assert_eq!(ij.unwrap().i, 0, "ij.i correct (1)");
        assert_eq!(ij.unwrap().j, 0, "ij.j correct (1)");

        let ij = H3Index::experimentalH3ToLocalIj(bc1, pent1);
        assert!(ij.is_ok(), "found IJ (2)");
        assert_eq!(ij.unwrap().i, 1, "ij.i correct (2)");
        assert_eq!(ij.unwrap().j, 0, "ij.j correct (2)");

        let ij = H3Index::experimentalH3ToLocalIj(bc1, bc2);
        assert!(ij.is_ok(), "found IJ (3)");
        assert!(ij.unwrap().i == 0 && ij.unwrap().j == -1, "ij correct (3)");

        let ij = H3Index::experimentalH3ToLocalIj(bc1, bc3);
        assert!(ij.is_ok(), "found IJ (4)");
        assert!(ij.unwrap().i == -1 && ij.unwrap().j == 0, "ij correct (4)");

        let ij = H3Index::experimentalH3ToLocalIj(pent1, bc3);
        assert!(ij.is_err(), "found IJ (5)");
    }

    #[test]
    fn experimentalH3ToLocalIjInvalid() {
        let (bc1, _, _, _) = setup();
        let mut invalid_index = H3Index(0x7fffffffffffffff);
        invalid_index.set_resolution(bc1.get_resolution());

        let ij = H3Index::experimentalH3ToLocalIj(bc1, invalid_index);
        assert!(ij.is_err(), "invalid index");

        let ij = H3Index::experimentalH3ToLocalIj(H3Index(0x7fffffffffffffff), invalid_index);
        assert!(ij.is_err(), "invalid origin");

        let ij = H3Index::experimentalH3ToLocalIj(
            H3Index(0x7fffffffffffffff),
            H3Index(0x7fffffffffffffff),
        );
        assert!(ij.is_err(), "invalid origin, and index");
    }

    #[test]
    fn experimentalLocalIjToH3Invalid() {
        let ij = CoordIJ::default();
        let index = H3Index::experimentalLocalIjToH3(&H3Index(0x7fffffffffffffff), &ij);

        assert!(index.is_err(), "invalid origin for ijToH3");
    }

    /**
     * Test that coming from the same direction outside the pentagon is handled
     * the same as coming from the same direction inside the pentagon.
     */
    #[test]
    fn onOffPentagonSame() {
        for bc in 0..BaseCell::NUM_BASE_CELLS {
            for res in 1..=Resolution::MAX_H3_RES {
                let res: Resolution = res.into();
                let bc: BaseCell = bc.into();
                // K_AXES_DIGIT is the first internal direction, and it's also
                // invalid for pentagons, so skip to next.
                let mut startDir = Direction::K_AXES_DIGIT;
                if bc._isBaseCellPentagon() {
                    startDir += 1;
                }

                let mut dir = startDir;
                while dir != Direction::INVALID_DIGIT {
                    let internalOrigin = H3Index::setH3Index(res, bc, dir);
                    let externalOrigin = H3Index::setH3Index(
                        res,
                        bc._getBaseCellNeighbor(&dir),
                        Direction::CENTER_DIGIT,
                    );

                    let mut testDir = startDir;
                    while testDir != Direction::INVALID_DIGIT {
                        let testIndex = H3Index::setH3Index(res, bc, testDir);

                        let internalIj =
                            H3Index::experimentalH3ToLocalIj(internalOrigin, testIndex);
                        let externalIj =
                            H3Index::experimentalH3ToLocalIj(externalOrigin, testIndex);
                        assert_eq!(
                            internalIj.is_err(),
                            externalIj.is_err(),
                            "internal/external failed matches when getting IJ"
                        );

                        if internalIj.is_err() {
                            continue;
                        }

                        let internalIndex =
                            H3Index::experimentalLocalIjToH3(&internalOrigin, &internalIj.unwrap());
                        let externalIndex =
                            H3Index::experimentalLocalIjToH3(&externalOrigin, &externalIj.unwrap());

                        assert_eq!(
                            internalIj.is_err(),
                            externalIj.is_err(),
                            "internal/external failed matches when getting index"
                        );

                        if internalIj.is_err() {
                            continue;
                        }

                        assert_eq!(
                            internalIndex.unwrap(),
                            externalIndex.unwrap(),
                            "internal/external index matches"
                        );

                        testDir += 1;
                    }

                    dir += 1;
                }

                todo!()
            }
        }
    }

    /// Test that the local coordinates for an index map to itself.
    fn localIjToH3_identity_assertions(h3: H3Index) {
        let ij = H3Index::experimentalH3ToLocalIj(h3, h3);
        assert!(ij.is_ok(), "able to setup localIjToH3 test");

        let retrieved = h3.experimentalLocalIjToH3(&ij.unwrap());
        assert!(retrieved.is_ok(), "got an index back from localIjTOh3");
        assert_eq!(
            h3,
            retrieved.unwrap(),
            "round trip through local IJ space works"
        );
    }

    /// Test that coordinates for an index match some simple rules about index
    /// digits, when using the index as its own origin. That is, that the IJ
    /// coordinates are in the coordinate space of the origin's base cell.
    fn h3ToLocalIj_coordinates_assertions(h3: H3Index) {
        let r = h3.get_resolution();

        let ij = H3Index::experimentalH3ToLocalIj(h3, h3);
        assert!(ij.is_ok(), "get ij for origin");

        let ijk = ij.unwrap().ijToIjk();

        if r == Resolution::R0 {
            assert_eq!(ijk, CoordIJK::UNIT_VECS[0].0, "res 0 cell at 0,0,0");
        } else if r == Resolution::R1 {
            let index = h3.get_index_digit(r);
            assert_eq!(
                ijk,
                CoordIJK::UNIT_VECS[index as usize].0,
                "res 1 cell at expected coordinates"
            );
        } else if r == Resolution::R2 {
            // the C unit test uses an index digit of 1 here, not 2
            let index = h3.get_index_digit(r);
            let mut expected = CoordIJK::UNIT_VECS[index as usize].0;

            expected._downAp7r();
            expected._neighbor(h3.get_index_digit(r));
            assert_eq!(ijk, expected, "res 2 cell at expected coordinates");
        } else {
            assert!(false, "resolution supported by test function (coordinates)");
        }
    }

    /**
     * Test the the immediate neighbors of an index are at the expected locations in
     * the local IJ coordinate space.
     */
    fn h3ToLocalIj_neighbors_assertions(h3: H3Index) {
        let origin = H3Index::experimentalH3ToLocalIj(h3, h3);
        assert!(origin.is_ok(), "got ij for origin");

        let originIjk = origin.unwrap().ijToIjk();

        let mut d = Direction::K_AXES_DIGIT;
        while d != Direction::INVALID_DIGIT {
            if d == Direction::K_AXES_DIGIT && h3.is_pentagon() {
                continue;
            }

            let mut rotations = 0;
            let offset = h3.h3NeighborRotations(d, &mut rotations);

            let ij = H3Index::experimentalH3ToLocalIj(h3, offset);
            assert!(ij.is_ok(), "got ij for destination");
            let mut ijk = ij.unwrap().ijToIjk();
            let mut invertedIjk = CoordIJK::default();
            invertedIjk._neighbor(d);

            for _ in 0..3 {
                invertedIjk._ijkRotate60ccw();
            }

            ijk = invertedIjk + ijk;
            ijk.normalize();

            assert_eq!(ijk, originIjk, "back to origin");
        }
        d += 1;
    }

    /// Call the callback for every index at the given resolution.
    fn iterateAllIndexesAtRes(res: Resolution, cb: fn(H3Index)) {
        //void (*callback)(H3Index)) {
        iterateAllIndexesAtResPartial(res, cb, BaseCell::NUM_BASE_CELLS);
    }

    /// Call the callback for every index at the given resolution in base
    /// cell 0 up to the given base cell number.
    fn iterateAllIndexesAtResPartial(res: Resolution, cb: fn(H3Index), baseCells: usize) {
        assert!(baseCells <= BaseCell::NUM_BASE_CELLS);
        for i in 0..baseCells {
            iterateBaseCellIndexesAtRes(res, cb, i);
        }
    }

    //#[test]
    //fn h3ToLocalIj_neighbors() {
    //iterateAllIndexesAtRes(0, h3ToLocalIj_neighbors_assertions);
    //iterateAllIndexesAtRes(1, h3ToLocalIj_neighbors_assertions);
    //iterateAllIndexesAtRes(2, h3ToLocalIj_neighbors_assertions);
    //}
}

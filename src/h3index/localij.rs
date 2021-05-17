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
}

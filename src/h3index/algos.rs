use crate::{
    basecell::{baseCellData, baseCellNeighbor60CCWRots, baseCellNeighbors, BaseCell},
    direction::{Direction, Direction::*},
    H3Index, Resolution,
};

/// Return codes from hexRange and related functions.
pub(crate) enum HexRangeCode {
    HEX_RANGE_SUCCESS,       // 0
    HEX_RANGE_PENTAGON,      // 1
    HEX_RANGE_K_SUBSEQUENCE, // 2
    MAX_ONE_RING_SIZE,       // 7
    HEX_HASH_OVERFLOW,       // -1
    POLYFILL_BUFFER,         // 12
}

/**
 * Directions used for traversing a hexagonal ring counterclockwise around
 * {1, 0, 0}
 *
 * <pre>
 *      _
 *    _/ \\_
 *   / \\5/ \\
 *   \\0/ \\4/
 *   / \\_/ \\
 *   \\1/ \\3/
 *     \\2/
 * </pre>
 */
const DIRECTIONS: [Direction; 6] = [
    J_AXES_DIGIT,
    JK_AXES_DIGIT,
    K_AXES_DIGIT,
    IK_AXES_DIGIT,
    I_AXES_DIGIT,
    IJ_AXES_DIGIT,
];

/// Direction used for traversing to the next outward hexagonal ring.
const NEXT_RING_DIRECTION: Direction = Direction::I_AXES_DIGIT;

/**
 * New digit when traversing along class II grids.
 *
 * Current digit -> direction -> new digit.
 */
const NEW_DIGIT_II: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        K_AXES_DIGIT,
        I_AXES_DIGIT,
        JK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        IK_AXES_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        K_AXES_DIGIT,
        I_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
    ],
    [
        JK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
    ],
    [
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        K_AXES_DIGIT,
    ],
    [
        IK_AXES_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        JK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        I_AXES_DIGIT,
    ],
    [
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        J_AXES_DIGIT,
        K_AXES_DIGIT,
        I_AXES_DIGIT,
        JK_AXES_DIGIT,
    ],
];

/**
 * New traversal direction when traversing along class II grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_ADJUSTMENT_II: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        JK_AXES_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        I_AXES_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
    ],
];

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_DIGIT_III: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
    ],
    [
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
    ],
    [
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
    ],
    [
        IK_AXES_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
    ],
    [
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
    ],
];

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_ADJUSTMENT_III: [[Direction; 7]; 7] = [
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        J_AXES_DIGIT,
        J_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        JK_AXES_DIGIT,
        J_AXES_DIGIT,
        JK_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        IK_AXES_DIGIT,
        I_AXES_DIGIT,
    ],
    [
        CENTER_DIGIT,
        K_AXES_DIGIT,
        CENTER_DIGIT,
        CENTER_DIGIT,
        IK_AXES_DIGIT,
        IK_AXES_DIGIT,
        CENTER_DIGIT,
    ],
    [
        CENTER_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
        CENTER_DIGIT,
        I_AXES_DIGIT,
        CENTER_DIGIT,
        IJ_AXES_DIGIT,
    ],
];

/**
 * Maximum number of cells that result from the kRing algorithm with the given
 * k. Formula source and proof: https://oeis.org/A003215
 *
 * @param  k   k value, k >= 0.
 */
pub fn maxKringSize(k: u32) -> u32 {
    3 * k * (k + 1) + 1
}

impl H3Index {
    /**
     * Returns the hexagon index neighboring the origin, in the direction dir.
     *
     * Implementation note: The only reachable case where this returns 0 is if the
     * origin is a pentagon and the translation is in the k direction. Thus,
     * 0 can only be returned if origin is a pentagon.
     *
     * @param origin Origin index
     * @param dir Direction to move in
     * @param rotations Number of ccw rotations to perform to reorient the
     *                  translation vector. Will be modified to the new number of
     *                  rotations to perform (such as when crossing a face edge.)
     * @return H3Index of the specified neighbor or H3_NULL if deleted k-subsequence
     *         distortion is encountered.
     */
    //fn h3NeighborRotations(H3Index origin, Direction dir, int* rotations) -> Self {
    pub(crate) fn h3NeighborRotations(&self, mut dir: Direction, rotations: &mut i32) -> Self {
        let mut out = *self;

        for _ in 0..*rotations {
            dir = dir.rotate60ccw();
        }

        let oldBaseCell = out.get_base_cell();
        if oldBaseCell.0 >= BaseCell::NUM_BASE_CELLS as i32 {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return H3Index::H3_NULL;
        }

        let oldLeadingDigit = out._h3LeadingNonZeroDigit();

        let mut newRotations = 0;

        // Adjust the indexing digits and, if needed, the base cell.
        let mut r = out.get_resolution() as i32 - 1;
        loop {
            if r == -1 {
                out.set_base_cell(baseCellNeighbors[usize::from(oldBaseCell)][dir as usize]);
                newRotations = baseCellNeighbor60CCWRots[usize::from(oldBaseCell)][dir as usize].0;

                if out.get_base_cell() == BaseCell::INVALID {
                    // Adjust for the deleted k vertex at the base cell level.
                    // This edge actually borders a different neighbor.
                    out.set_base_cell(
                        baseCellNeighbors[usize::from(oldBaseCell)]
                            [Direction::IK_AXES_DIGIT as usize],
                    );
                    newRotations = baseCellNeighbor60CCWRots[usize::from(oldBaseCell)]
                        [Direction::IK_AXES_DIGIT as usize]
                        .0;

                    // perform the adjustment for the k-subsequence we're skipping over.
                    out = out._h3Rotate60ccw();
                    *rotations += 1;
                    //*rotations = *rotations + 1;
                }

                break;
            } else {
                let res: Resolution = (r + 1).into();
                let oldDigit = out.get_index_digit(res);

                let nextDir = if res.isResClassIII() {
                    out.set_index_digit(
                        res,
                        NEW_DIGIT_II[usize::from(oldDigit)][usize::from(dir)] as u64,
                    );
                    NEW_ADJUSTMENT_II[usize::from(oldDigit)][usize::from(dir)]
                } else {
                    out.set_index_digit(
                        res,
                        NEW_DIGIT_III[usize::from(oldDigit)][usize::from(dir)] as u64,
                    );
                    NEW_ADJUSTMENT_III[usize::from(oldDigit)][usize::from(dir)]
                };

                if nextDir != CENTER_DIGIT {
                    dir = nextDir;
                    r -= 1;
                } else {
                    // No more adjustment to perform
                    break;
                }
            }
        }

        let newBaseCell = out.get_base_cell();

        if newBaseCell._isBaseCellPentagon() {
            let mut alreadyAdjustedKSubsequence = false;

            // force rotation out of missing k-axes sub-sequence
            if out._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                if oldBaseCell != newBaseCell {
                    // in this case, we traversed into the deleted k subsequence of a pentagon base cell.
                    // We need to rotate out of that case depending on how we got here.
                    // check for a cw/ccw offset face; default is ccw

                    out = if newBaseCell
                        ._baseCellIsCwOffset(&baseCellData[usize::from(oldBaseCell)].homeFijk)
                    {
                        out._h3Rotate60cw()
                    } else {
                        // See cwOffsetPent in testKRing.c for why this is unreachable.
                        out._h3Rotate60ccw() // LCOV_EXCL_LINE
                    };

                    alreadyAdjustedKSubsequence = true;
                } else {
                    // In this case, we traversed into the deleted k subsequence from within the same pentagon base cell.
                    if oldLeadingDigit == CENTER_DIGIT {
                        // Undefined: the k direction is deleted from here
                        return H3Index::H3_NULL;
                    } else if oldLeadingDigit == JK_AXES_DIGIT {
                        // Rotate out of the deleted k subsequence
                        // We also need an additional change to the direction we're moving in
                        out = out._h3Rotate60ccw();
                        *rotations = *rotations + 1;
                    } else if oldLeadingDigit == IK_AXES_DIGIT {
                        // Rotate out of the deleted k subsequence
                        // We also need an additional change to the direction we're moving in
                        out = out._h3Rotate60cw();
                        *rotations = *rotations + 5;
                    } else {
                        // Should never occur
                        return H3Index::H3_NULL; // LCOV_EXCL_LINE
                    }
                }
            }

            for _ in 0..newRotations {
                out = out._h3Rotate60ccw();
            }

            // Account for differing orientation of the base cells (this edge might not follow properties of some other edges.)
            if oldBaseCell != newBaseCell {
                if newBaseCell._isBaseCellPolarPentagon() {
                    // 'polar' base cells behave differently because they have all i neighbors.
                    if oldBaseCell != 118
                        && oldBaseCell != 8
                        && out._h3LeadingNonZeroDigit() != JK_AXES_DIGIT
                    {
                        *rotations = *rotations + 1;
                    }
                } else if out._h3LeadingNonZeroDigit() == IK_AXES_DIGIT
                    && !alreadyAdjustedKSubsequence
                {
                    // account for distortion introduced to the 5 neighbor by the deleted k subsequence.
                    *rotations = *rotations + 1;
                }
            }
        } else {
            for _ in 0..newRotations {
                out = out._h3Rotate60ccw();
            }
        }

        *rotations = (*rotations + newRotations) % 6;

        out
    }
}

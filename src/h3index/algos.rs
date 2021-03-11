use super::H3Index;
use crate::{
    baseCells::{baseCellData, baseCellNeighbor60CCWRots, baseCellNeighbors},
    constants::{INVALID_BASE_CELL, NUM_BASE_CELLS},
    direction::Direction,
};

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
    Direction::J_AXES_DIGIT,
    Direction::JK_AXES_DIGIT,
    Direction::K_AXES_DIGIT,
    Direction::IK_AXES_DIGIT,
    Direction::I_AXES_DIGIT,
    Direction::IJ_AXES_DIGIT,
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
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ],
    [
        Direction::K_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IK_AXES_DIGIT,
    ],
    [
        Direction::JK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
    ],
    [
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::K_AXES_DIGIT,
    ],
    [
        Direction::IK_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
    ],
    [
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
    ],
];

/**
 * New traversal direction when traversing along class II grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_ADJUSTMENT_II: [[Direction; 7]; 7] = [
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::J_AXES_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ],
];

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_DIGIT_III: [[Direction; 7]; 7] = [
    [
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ],
    [
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
    ],
    [
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
    ],
    [
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
    ],
    [
        Direction::IK_AXES_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
    ],
    [
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
    ],
];

/**
 * New traversal direction when traversing along class III grids.
 *
 * Current digit -> direction -> new ap7 move (at coarser level).
 */
const NEW_ADJUSTMENT_III: [[Direction; 7]; 7] = [
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::J_AXES_DIGIT,
        Direction::JK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::I_AXES_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::K_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::IK_AXES_DIGIT,
        Direction::CENTER_DIGIT,
    ],
    [
        Direction::CENTER_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IJ_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::I_AXES_DIGIT,
        Direction::CENTER_DIGIT,
        Direction::IJ_AXES_DIGIT,
    ],
];

impl H3Index {
    /**
     * Get the direction from the origin to a given neighbor. This is effectively
     * the reverse operation for h3NeighborRotations. Returns INVALID_DIGIT if the
     * cells are not neighbors.
     *
     * TODO: This is currently a brute-force algorithm, but as it's O(6) that's
     * probably acceptible.
     */
    pub(crate) fn directionForNeighbor(&self, destination: &Self) -> Direction {
        let isPentagon = self.h3IsPentagon();
        // Checks each neighbor, in order, to determine which direction the
        // destination neighbor is located. Skips CENTER_DIGIT since that
        // would be the origin; skips deleted K direction for pentagons.
        let direction = if isPentagon {
            Direction::J_AXES_DIGIT
        } else {
            Direction::K_AXES_DIGIT
        };

        while direction != Direction::INVALID {
            let mut rotations = 0;
            let neighbor = self.h3NeighborRotations(direction, &mut rotations);
            if neighbor == destination {
                return direction;
            }

            direction += 1;
        }

        Direction::INVALID
    }

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
    pub(crate) fn h3NeighborRotations(&self, dir: Direction, rotations: &mut i32) -> Self {
        let mut out = *self;

        for i in 0..*rotations {
            dir = dir._rotate60ccw();
        }

        let mut newRotations = 0;
        let mut oldBaseCell = out.H3_GET_BASE_CELL();
        if oldBaseCell < 0 || oldBaseCell >= NUM_BASE_CELLS {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return Self::H3_NULL;
        }
        let oldLeadingDigit: Direction = out._h3LeadingNonZeroDigit();

        // Adjust the indexing digits and, if needed, the base cell.
        let r = out.H3_GET_RESOLUTION() - 1;
        loop {
            if r == -1 {
                out.H3_SET_BASE_CELL(baseCellNeighbors[oldBaseCell][dir]);
                newRotations = baseCellNeighbor60CCWRots[oldBaseCell][dir];

                if out.H3_GET_BASE_CELL() == INVALID_BASE_CELL {
                    // Adjust for the deleted k vertex at the base cell level.
                    // This edge actually borders a different neighbor.
                    out.H3_SET_BASE_CELL(
                        baseCellNeighbors[oldBaseCell][Direction::IK_AXES_DIGIT as usize],
                    );
                    newRotations =
                        baseCellNeighbor60CCWRots[oldBaseCell][Direction::IK_AXES_DIGIT as usize];

                    // perform the adjustment for the k-subsequence we're skipping
                    // over.
                    out = out._h3Rotate60ccw();
                    *rotations = *rotations + 1;
                }

                break;
            } else {
                let oldDigit = out.H3_GET_INDEX_DIGIT(r + 1);
                let nextDir: Direction = if H3Index::isResClassIII(r + 1) {
                    out.H3_SET_INDEX_DIGIT(r + 1, NEW_DIGIT_II[oldDigit][dir]);
                    NEW_ADJUSTMENT_II[oldDigit][dir]
                } else {
                    out.H3_SET_INDEX_DIGIT(r + 1, NEW_DIGIT_III[oldDigit][dir]);
                    NEW_ADJUSTMENT_III[oldDigit][dir]
                };

                if nextDir != Direction::CENTER_DIGIT {
                    dir = nextDir;
                    r -= 1;
                } else {
                    // No more adjustment to perform
                    break;
                }
            }
        }

        let newBaseCell = out.H3_GET_BASE_CELL();
        if newBaseCell._isBaseCellPentagon() {
            let mut alreadyAdjustedKSubsequence = 0;

            // force rotation out of missing k-axes sub-sequence
            if out._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                if oldBaseCell != newBaseCell {
                    // in this case, we traversed into the deleted
                    // k subsequence of a pentagon base cell.
                    // We need to rotate out of that case depending
                    // on how we got here.
                    // check for a cw/ccw offset face; default is ccw

                    out = if newBaseCell
                        ._baseCellIsCwOffset(baseCellData[oldBaseCell].homeFijk.face)
                    {
                        out._h3Rotate60cw()
                    } else {
                        // See cwOffsetPent in testKRing.c for why this is
                        // unreachable.
                        out._h3Rotate60ccw() // LCOV_EXCL_LINE
                    };
                    alreadyAdjustedKSubsequence = 1;
                } else {
                    // In this case, we traversed into the deleted
                    // k subsequence from within the same pentagon
                    // base cell.
                    if oldLeadingDigit == Direction::CENTER_DIGIT {
                        // Undefined: the k direction is deleted from here
                        return H3Index::H3_NULL;
                    } else if oldLeadingDigit == Direction::JK_AXES_DIGIT {
                        // Rotate out of the deleted k subsequence
                        // We also need an additional change to the direction we're
                        // moving in
                        out = out._h3Rotate60ccw();
                        *rotations = *rotations + 1;
                    } else if oldLeadingDigit == Direction::IK_AXES_DIGIT {
                        // Rotate out of the deleted k subsequence
                        // We also need an additional change to the direction we're
                        // moving in
                        out = out._h3Rotate60cw();
                        *rotations = *rotations + 5;
                    } else {
                        // Should never occur
                        return H3Index::H3_NULL; // LCOV_EXCL_LINE
                    }
                }
            }

            for i in 0..newRotations {
                out = out._h3RotatePent60ccw();
            }

            // Account for differing orientation of the base cells (this edge
            // might not follow properties of some other edges.)
            if oldBaseCell != newBaseCell {
                if newBaseCell._isBaseCellPolarPentagon() {
                    // 'polar' base cells behave differently because they have all i neighbors.
                    if oldBaseCell != 118
                        && oldBaseCell != 8
                        && out._h3LeadingNonZeroDigit() != Direction::JK_AXES_DIGIT
                    {
                        *rotations = *rotations + 1;
                    }
                } else if out._h3LeadingNonZeroDigit() == Direction::IK_AXES_DIGIT
                    && !alreadyAdjustedKSubsequence
                {
                    // account for distortion introduced to the 5 neighbor by the deleted k subsequence.
                    *rotations = *rotations + 1;
                }
            }
        } else {
            for i in 0..newRotations {
                out = out._h3Rotate60ccw();
            }
        }

        *rotations = (*rotations + newRotations) % 6;

        out
    }

    /**
     * Maximum number of cells that result from the kRing algorithm with the given
     * k. Formula source and proof: https://oeis.org/A003215
     *
     * @param  k   k value, k >= 0.
     */
    pub(crate) fn maxKringSize(k: i32) -> i32 {
        3 * k * (k + 1) + 1
    }

    /**
     * Produce cells within grid distance k of the origin cell.
     *
     * k-ring 0 is defined as the origin cell, k-ring 1 is defined as k-ring 0 and
     * all neighboring cells, and so on.
     *
     * Output is placed in the provided array in no particular order. Elements of
     * the output array may be left zero, as can happen when crossing a pentagon.
     *
     * @param  origin   origin cell
     * @param  k        k >= 0
     * @param  out      zero-filled array which must be of size maxKringSize(k)
     */
    pub(crate) fn kRing(&self, k: i32) -> Self {
        //H3_EXPORT(kRingDistances)(origin, k, out, NULL);
        Self.kRingDistances(k, None);
    }

    /**
     * Produce cells and their distances from the given origin cell, up to
     * distance k.
     *
     * k-ring 0 is defined as the origin cell, k-ring 1 is defined as k-ring 0 and
     * all neighboring cells, and so on.
     *
     * Output is placed in the provided array in no particular order. Elements of
     * the output array may be left zero, as can happen when crossing a pentagon.
     *
     * @param  origin      origin cell
     * @param  k           k >= 0
     * @param  out         zero-filled array which must be of size maxKringSize(k)
     * @param  distances   NULL or a zero-filled array which must be of size
     *                     maxKringSize(k)
     */
    pub(crate) fn kRingDistances(&self, k: i32, distances: Option<Vec<i32>>) -> Self {
        // Optimistically try the faster hexRange algorithm first
        let failed = self.hexRangeDistances(k, out, distances);
        if failed {
            let maxIdx = maxKringSize(k);
            // Fast algo failed, fall back to slower, correct algo
            // and also wipe out array because contents untrustworthy
            //memset(out, 0, maxIdx * sizeof(H3Index));
            //
            if let Some(dists) = distances {
                Self::_kRingInternal(origin, k, out, distances, maxIdx, 0);
            } else {
                distances = H3_MEMORY(calloc)(maxIdx, sizeof(int));
                if !distances {
                    // TODO: Return an error code when this is not void
                    return;
                }
                Self::_kRingInternal(origin, k, out, distances, maxIdx, 0);
            }
        }
    }

    /**
     * Internal helper function called recursively for kRingDistances.
     *
     * Adds the origin cell to the output set (treating it as a hash set)
     * and recurses to its neighbors, if needed.
     *
     * @param  origin      Origin cell
     * @param  k           Maximum distance to move from the origin
     * @param  out         Array treated as a hash set, elements being either
     *                     H3Index or 0.
     * @param  distances   Scratch area, with elements paralleling the out array.
     *                     Elements indicate ijk distance from the origin cell to
     *                     the output cell
     * @param  maxIdx      Size of out and scratch arrays (must be maxKringSize(k))
     * @param  curK        Current distance from the origin
     */
    fn bleh() {}

    /*
    fn _kRingInternal(H3Index origin, int k, H3Index* out, int* distances, int maxIdx, int curK) {
        if (origin == 0) {
            return;
        }

        // Put origin in the output array. out is used as a hash set.
        let mut off = origin % maxIdx;
        while (out[off] != 0 && out[off] != origin) {
            off = (off + 1) % maxIdx;
        }

        // We either got a free slot in the hash set or hit a duplicate
        // We might need to process the duplicate anyways because we got
        // here on a longer path before.
        if (out[off] == origin && distances[off] <= curK) {
            return;
        }

        out[off] = origin;
        distances[off] = curK;

        // Base case: reached an index k away from the origin.
        if (curK >= k) return;

        // Recurse to all neighbors in no particular order.
        for i in 0..6 {
            let mut rotations = 0;
            _kRingInternal(h3NeighborRotations(origin, DIRECTIONS[i], &mut rotations),
            k, out, distances, maxIdx, curK + 1);
        }
    }
    */
}

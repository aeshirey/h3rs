use std::{collections::HashSet, str::FromStr};
mod geocoord;
pub use geocoord::*;

use crate::{
    basecell::BaseCell,
    constants::{NUM_HEX_VERTS, NUM_PENT_VERTS},
    faceijk::FaceIJK,
    geopolygon::GeoBoundary,
    Direction, GeoCoord, Resolution,
};

mod algos;
mod basecell;
mod h3UniEdge;
mod localij;

#[derive(Clone, Copy, PartialEq, Debug)]
/// The H3Index fits within a 64-bit unsigned integer
pub struct H3Index(u64);

impl H3Index {
    /// Invalid index used to indicate an error from geoToH3 and related functions or missing data in arrays of h3 indices. Analogous to NaN in floating point.
    pub(crate) const H3_NULL: H3Index = H3Index(0);

    // define's of constants and macros for bitwise manipulation of H3Index's.

    /// The number of bits in an H3 index.
    const H3_NUM_BITS: u64 = 64;

    /// The bit offset of the max resolution digit in an H3 index.
    const H3_MAX_OFFSET: u64 = 63;

    /// The bit offset of the mode in an H3 index.
    const H3_MODE_OFFSET: u64 = 59;

    /// The bit offset of the base cell in an H3 index.
    const H3_BC_OFFSET: u64 = 45;

    /// The bit offset of the resolution in an H3 index.
    const H3_RES_OFFSET: u64 = 52;

    /// The bit offset of the reserved bits in an H3 index.
    const H3_RESERVED_OFFSET: u64 = 56;

    /// The number of bits in a single H3 resolution digit.
    const H3_PER_DIGIT_OFFSET: u64 = 3;

    /// 1 in the highest bit, 0's everywhere else.
    const H3_HIGH_BIT_MASK: u64 = 1 << Self::H3_MAX_OFFSET;

    /// 0 in the highest bit, 1's everywhere else.
    const H3_HIGH_BIT_MASK_NEGATIVE: u64 = !Self::H3_HIGH_BIT_MASK;

    /// 1's in the 4 mode bits, 0's everywhere else.
    const H3_MODE_MASK: u64 = 15 << Self::H3_MODE_OFFSET;

    /// 0's in the 4 mode bits, 1's everywhere else.
    const H3_MODE_MASK_NEGATIVE: u64 = !Self::H3_MODE_MASK;

    /// 1's in the 7 base cell bits, 0's everywhere else.
    const H3_BC_MASK: u64 = 127 << Self::H3_BC_OFFSET;

    /// 0's in the 7 base cell bits, 1's everywhere else.
    const H3_BC_MASK_NEGATIVE: u64 = !Self::H3_BC_MASK;

    /// 1's in the 4 resolution bits, 0's everywhere else.
    /// Note that in the original H3 library, this used UINT64_C, which is different than uint64_t
    /// in some cases.
    const H3_RES_MASK: u64 = 15 << Self::H3_RES_OFFSET;

    /// 0's in the 4 resolution bits, 1's everywhere else.
    const H3_RES_MASK_NEGATIVE: u64 = !Self::H3_RES_MASK;

    /// 1's in the 3 reserved bits, 0's everywhere else.
    const H3_RESERVED_MASK: u64 = 7 << Self::H3_RESERVED_OFFSET;

    /// 0's in the 3 reserved bits, 1's everywhere else.
    const H3_RESERVED_MASK_NEGATIVE: u64 = !Self::H3_RESERVED_MASK;

    /// 1's in the 3 bits of res 15 digit bits, 0's everywhere else.
    const H3_DIGIT_MASK: u64 = 7;

    /// 0's in the 7 base cell bits, 1's everywhere else.
    const H3_DIGIT_MASK_NEGATIVE: u64 = !Self::H3_DIGIT_MASK;

    /// H3 index with mode 0, res 0, base cell 0, and 7 for all index digits.
    /// Typically used to initialize the creation of an H3 cell index, which
    /// expects all direction digits to be 7 beyond the cell's resolution.
    pub(crate) const H3_INIT: H3Index = H3Index(35184372088831);

    /// Gets the highest bit of the H3 index.
    fn get_high_bit(&self) -> u64 {
        self.0 >> Self::H3_MAX_OFFSET
    }

    /// Gets the integer mode of h3.
    pub(crate) fn get_mode(&self) -> H3Mode {
        let m = (self.0 & Self::H3_MODE_MASK) >> Self::H3_MODE_OFFSET;
        println!("Getting mode for {:?}: {}", self, m);
        m.into()
    }

    /// Sets the integer mode of h3 to v.
    pub(crate) fn set_mode(&mut self, mode: H3Mode) {
        let v = mode as u64;
        self.0 = (self.0 & Self::H3_MODE_MASK_NEGATIVE) | (v << Self::H3_MODE_OFFSET);
    }

    /// Gets the integer base cell of h3.
    pub(crate) fn get_base_cell(&self) -> BaseCell {
        let bc = (self.0 & Self::H3_BC_MASK) >> Self::H3_BC_OFFSET;
        BaseCell::new(bc as i32)
    }

    /// Sets the integer base cell of h3 to bc.
    pub(crate) fn set_base_cell(&mut self, bc: BaseCell) {
        let i: i32 = bc.into();
        self.0 = (self.0 & Self::H3_BC_MASK_NEGATIVE) | ((i as u64) << Self::H3_BC_OFFSET);
    }

    /// Gets the integer resolution of h3.
    pub(crate) fn get_resolution(&self) -> Resolution {
        let r = (self.0 & Self::H3_RES_MASK) >> Self::H3_RES_OFFSET;
        Resolution::from(r)
    }

    /// Sets the integer resolution of h3.
    pub(crate) fn set_resolution(&mut self, res: Resolution) {
        let i: usize = res.into();
        self.0 = (self.0 & Self::H3_RES_MASK_NEGATIVE) | ((i as u64) << Self::H3_RES_OFFSET);
    }

    /// Sets a value in the reserved space. Setting to non-zero may produce invalid indexes.
    pub(crate) fn set_reserved_bits(&mut self, v: u64) {
        self.0 = (self.0 & Self::H3_RESERVED_MASK_NEGATIVE) | (v << Self::H3_RESERVED_OFFSET);
    }

    /// Gets a value in the reserved space. Should always be zero for valid indexes.
    pub(crate) fn get_reserved_bits(&self) -> u64 {
        (self.0 & Self::H3_RESERVED_MASK) >> Self::H3_RESERVED_OFFSET
    }

    /// Sets the highest bit of the h3 to v.
    pub(crate) fn set_high_bit(&mut self, v: u64) {
        self.0 = (self.0 & Self::H3_HIGH_BIT_MASK_NEGATIVE) | (v << Self::H3_MAX_OFFSET);
    }

    /// Gets the resolution res integer digit (0-7) of h3.
    pub(crate) fn get_index_digit(&self, res: Resolution) -> Direction {
        let r = usize::from(res) as u64;
        let d = (self.0 >> ((Resolution::MAX_H3_RES as u64 - r) * Self::H3_PER_DIGIT_OFFSET))
            & Self::H3_DIGIT_MASK;

        (d as usize).into()
    }

    /// Sets the resolution res digit of h3 to the integer digit (0-7)
    pub(crate) fn set_index_digit(&mut self, res: Resolution, digit: u64) {
        let r = usize::from(res) as u64;
        self.0 = (self.0
            & !(Self::H3_DIGIT_MASK
                << ((Resolution::MAX_H3_RES as u64 - r) * Self::H3_PER_DIGIT_OFFSET)))
            | (digit << ((Resolution::MAX_H3_RES as u64 - r) * Self::H3_PER_DIGIT_OFFSET))
    }

    /**
     * Determines the spherical coordinates of the center point of an H3 index.
     *
     * @param h3 The H3 index.
     * @param g The spherical coordinates of the H3 cell center.
     */
    pub fn h3ToGeo(&self) -> GeoCoord {
        let fijk = self._h3ToFaceIjk();
        let res = self.get_resolution();
        fijk._faceIjkToGeo(res)
    }

    /**
     * h3IsPentagon takes an H3Index and determines if it is actually a
     * pentagon.
     * @param h The H3Index to check.
     * @return Returns 1 if it is a pentagon, otherwise 0.
     */
    pub fn is_pentagon(&self) -> bool {
        self.get_base_cell()._isBaseCellPentagon()
            && self._h3LeadingNonZeroDigit() == Direction::CENTER_DIGIT
    }

    /**
     * Returns the highest resolution non-zero digit in an H3Index.
     * @param h The H3Index.
     * @return The highest resolution non-zero digit in the H3Index.
     */
    pub(crate) fn _h3LeadingNonZeroDigit(&self) -> Direction {
        for r in 1..=self.get_resolution() as usize {
            let dig = self.get_index_digit(r.into());
            if dig != Direction::CENTER_DIGIT {
                return dig;
            }
        }

        // if we're here it's all 0's
        Direction::CENTER_DIGIT
    }

    /**
     * h3ToParent produces the parent index for a given H3 index
     *
     * @param h H3Index to find parent of
     * @param parentRes The resolution to switch to (parent, grandparent, etc)
     *
     * @return H3Index of the parent, or H3_NULL if you actually asked for a child
     */
    pub fn h3ToParent(&mut self, parentRes: Resolution) -> Self {
        let childRes = self.get_resolution();
        if parentRes > childRes {
            return Self::H3_NULL;
        } else if parentRes == childRes {
            return *self;
        }

        self.set_resolution(parentRes);
        let mut parentH = *self;
        for i in parentRes as u64 + 1..=childRes as u64 {
            parentH.set_index_digit(i.into(), Self::H3_DIGIT_MASK);
        }

        parentH
    }

    /**
     * maxH3ToChildrenSize returns the maximum number of children possible for a
     * given child level.
     *
     * @param h H3Index to find the number of children of
     * @param childRes The resolution of the child level you're interested in
     *
     * @return int count of maximum number of children (equal for hexagons, less for
     * pentagons
     */
    pub fn maxH3ToChildrenSize(&self, childRes: Resolution) -> u64 {
        let parentRes = self.get_resolution();
        if !parentRes._isValidChildRes(&childRes) {
            0
        } else {
            //_ipow(7, (childRes - parentRes));
            let c = (childRes as u64) as u32;
            let p = (parentRes as u64) as u32;
            7u64.pow(c - p)
        }
    }

    /**
     * h3ToCenterChild produces the center child index for a given H3 index at
     * the specified resolution
     *
     * @param h H3Index to find center child of
     * @param childRes The resolution to switch to
     *
     * @return H3Index of the center child, or H3_NULL if you actually asked for a
     * parent
     */
    pub fn h3ToCenterChild(&mut self, childRes: Resolution) -> Self {
        let parentRes = self.get_resolution();
        if !parentRes._isValidChildRes(&childRes) {
            return Self::H3_NULL;
        } else if childRes == parentRes {
            return *self;
        }

        self.set_resolution(childRes);
        let mut child = *self;
        for i in parentRes as u64 + 1..=childRes as u64 {
            child.set_index_digit(i.into(), 0);
        }

        child
    }

    /**
     * h3IsResClassIII takes a hexagon ID and determines if it is in a
     * Class III resolution (rotated versus the icosahedron and subject
     * to shape distortion adding extra points on icosahedron edges, making
     * them not true hexagons).
     * @param h The H3Index to check.
     * @return Returns 1 if the hexagon is class III, otherwise 0.
     */
    pub fn h3IsResClassIII(&self) -> bool {
        self.get_resolution() as u64 % 2 == 1
    }

    /**
     * Rotate an H3Index 60 degrees counter-clockwise.
     * @param h The H3Index.
     */
    pub(crate) fn _h3Rotate60ccw(&self) -> Self {
        let res = self.get_resolution() as u64;
        let mut h = *self;
        for r in 1..=res {
            let old_digit = self.get_index_digit(r.into());
            let old_digit = old_digit.rotate60ccw() as u64;
            h.set_index_digit(r.into(), old_digit);
        }

        *self
    }

    /**
     * Rotate an H3Index 60 degrees clockwise.
     * @param h The H3Index.
     */
    pub(crate) fn _h3Rotate60cw(&self) -> Self {
        let res = self.get_resolution() as u64;
        let mut h = *self;
        for r in 1..=res {
            let old_digit = self.get_index_digit(r.into());
            let old_digit = old_digit.rotate60cw() as u64;
            h.set_index_digit(r.into(), old_digit);
        }

        *self
    }

    /// Rotate an H3Index 60 degrees counter-clockwise about a pentagonal center.
    pub(crate) fn _h3RotatePent60ccw(&self) -> Self {
        // rotate in place; skips any leading 1 digits (k-axis)
        let mut h = *self;

        let mut foundFirstNonZeroDigit = false;
        for r in 1..=self.get_resolution().into() {
            // rotate this digit
            let r = r.into();
            let digit = h.get_index_digit(r).rotate60ccw();
            h.set_index_digit(r.into(), digit.into());

            // look for the first non-zero digit so we can adjust for deleted k-axes sequence if necessary
            if !foundFirstNonZeroDigit && h.get_index_digit(r) != Direction::CENTER_DIGIT {
                foundFirstNonZeroDigit = true;

                // adjust for deleted k-axes sequence
                if h._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                    h = h._h3Rotate60ccw();
                }
            }
        }

        h
    }

    /// Rotate an H3Index 60 degrees clockwise about a pentagonal center.
    pub(crate) fn _h3RotatePent60cw(&self) -> Self {
        // rotate in place; skips any leading 1 digits (k-axis)
        let mut h = *self;

        let mut foundFirstNonZeroDigit = false;
        for r in 1..=self.get_resolution().into() {
            // rotate this digit
            let r = r.into();
            let digit = h.get_index_digit(r).rotate60cw();
            h.set_index_digit(r.into(), digit.into());

            // look for the first non-zero digit so we can adjust for deleted k-axes sequence if necessary
            if !foundFirstNonZeroDigit && h.get_index_digit(r) != Direction::CENTER_DIGIT {
                foundFirstNonZeroDigit = true;

                // adjust for deleted k-axes sequence
                if h._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                    h = h._h3Rotate60cw();
                }
            }
        }

        h
    }

    /**
     * Convert an H3Index to the FaceIJK address on a specified icosahedral face.
     * @param h The H3Index.
     * @param fijk The FaceIJK address, initialized with the desired face
     *        and normalized base cell coordinates.
     * @return Returns 1 if the possibility of overage exists, otherwise 0.
     */
    pub(crate) fn _h3ToFaceIjkWithInitializedFijk(&self, fijk: &mut FaceIJK) -> bool {
        let res = self.get_resolution();

        // center base cell hierarchy is entirely on this face
        let mut possibleOverage = false;

        if !self.get_base_cell()._isBaseCellPentagon()
            && (res == Resolution::R0
                || (fijk.coord.i == 0 && fijk.coord.j == 0 && fijk.coord.k == 0))
        {
            possibleOverage = false;
        }

        for r in 1..=res.into() {
            let r: Resolution = r.into();
            if r.isResClassIII() {
                // Class III == rotate ccw
                fijk.coord._downAp7();
            } else {
                // Class II == rotate cw
                fijk.coord._downAp7r();
            }

            fijk.coord._neighbor(self.get_index_digit(r));
        }

        possibleOverage
    }

    /// The number of pentagons (same at any resolution)
    pub fn pentagonIndexCount() -> i32 {
        crate::constants::NUM_PENTAGONS
    }

    /**
     * Generates all pentagons at the specified resolution
     *
     * @param res The resolution to produce pentagons at.
     * @param out Output array. Must be of size pentagonIndexCount().
     */
    pub fn getPentagonIndexes(res: Resolution) -> [Self; BaseCell::NUM_BASE_CELLS] {
        let mut result = [H3Index::H3_NULL; BaseCell::NUM_BASE_CELLS];

        for bc in 0..BaseCell::NUM_BASE_CELLS as i32 {
            let basecell = BaseCell::new(bc);
            if basecell._isBaseCellPentagon() {
                let pentagon = Self::setH3Index(res, basecell, Direction::CENTER_DIGIT);
                result[bc as usize] = pentagon;
            }
        }

        /*
        for bc in 0..BaseCell::NUM_BASE_CELLS {
            let basecell = BaseCell::new(bc as i32);
            if basecell._isBaseCellPentagon() {
                let pentagon = Self::setH3Index(res, basecell, Direction::CENTER_DIGIT);
                result[bc] = pentagon;
            }
        }
        */
        result
    }

    /// Returns whether or not an H3 index is a valid cell (hexagon or pentagon).
    pub fn is_valid(&self) -> bool {
        if self.get_high_bit() != 0 {
            return false;
        }

        if self.get_mode() != H3Mode::H3_HEXAGON_MODE {
            return false;
        }

        if self.get_reserved_bits() != 0 {
            return false;
        }

        let baseCell = self.get_base_cell();
        if baseCell.0 < 0 || baseCell.0 as usize >= BaseCell::NUM_BASE_CELLS {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return false;
        }

        let res = self.get_resolution();

        if res as usize >= Resolution::MAX_H3_RES {
            // Resolutions less than zero can not be represented in an index
            return false;
        }

        let mut found_first_non_zero_digit = false;
        for r in 1..=res.into() {
            let digit = self.get_index_digit(r.into());

            if !found_first_non_zero_digit && digit != Direction::CENTER_DIGIT {
                found_first_non_zero_digit = true;
                if baseCell._isBaseCellPentagon() && digit == Direction::K_AXES_DIGIT {
                    return false;
                }
            }

            if digit >= Direction::INVALID_DIGIT {
                return false;
            }
        }

        for r in (res as usize + 1)..=Resolution::MAX_H3_RES {
            let digit = self.get_index_digit(r.into());
            if digit != Direction::INVALID_DIGIT {
                return false;
            }
        }

        true
    }

    /**
     * Initializes an H3 index.
     * @param hp The H3 index to initialize.
     * @param res The H3 resolution to initialize the index to.
     * @param baseCell The H3 base cell to initialize the index to.
     * @param initDigit The H3 digit (0-7) to initialize all of the index digits to.
     */
    pub(crate) fn setH3Index(res: Resolution, base_cell: BaseCell, init_digit: Direction) -> Self {
        let mut h = Self::H3_INIT;
        h.set_mode(H3Mode::H3_HEXAGON_MODE);
        h.set_resolution(res);
        h.set_base_cell(base_cell);

        for r in 1..=res.into() {
            h.set_index_digit(r.into(), init_digit as u64);
        }

        h
    }

    /**
     * compact takes a set of hexagons all at the same resolution and compresses
     * them by pruning full child branches to the parent level. This is also done
     * for all parents recursively to get the minimum number of hex addresses that
     * perfectly cover the defined space.
     * @param h3Set Set of hexagons
     * @param compactedSet The output array of compressed hexagons (preallocated)
     * @param numHexes The size of the input and output arrays (possible that no
     * contiguous regions exist in the set at all and no compression possible)
     * @return an error code on bad input data
     */
    pub fn compact(h3Set: &[H3Index]) -> Result<Vec<H3Index>, i32> {
        if h3Set.is_empty() {
            return Ok(h3Set.iter().cloned().collect());
        }

        let res = h3Set[0].get_resolution();

        if res == Resolution::R0 {
            // No compaction possible, just copy the set to output
            return Ok(h3Set.iter().cloned().collect());
        }

        /*
        H3Index* remainingHexes = H3_MEMORY(malloc)(numHexes * sizeof(H3Index));
        if (!remainingHexes) {
            return COMPACT_ALLOC_FAILED;
        }
        memcpy(remainingHexes, h3Set, numHexes * sizeof(H3Index));
        H3Index* hashSetArray = H3_MEMORY(calloc)(numHexes, sizeof(H3Index));
        if (!hashSetArray) {
            H3_MEMORY(free)(remainingHexes);
            return COMPACT_ALLOC_FAILED;
        }
        H3Index* compactedSetOffset = compactedSet;
        int numRemainingHexes = numHexes;
        while (numRemainingHexes) {
            res = H3_GET_RESOLUTION(remainingHexes[0]);
            int parentRes = res - 1;
            // Put the parents of the hexagons into the temp array
            // via a hashing mechanism, and use the reserved bits
            // to track how many times a parent is duplicated
            for (int i = 0; i < numRemainingHexes; i++) {
                H3Index currIndex = remainingHexes[i];
                if (currIndex != 0) {
                    H3Index parent = H3_EXPORT(h3ToParent)(currIndex, parentRes);
                    // Modulus hash the parent into the temp array
                    int loc = (int)(parent % numRemainingHexes);
                    int loopCount = 0;
                    while (hashSetArray[loc] != 0) {
                        if (loopCount > numRemainingHexes) {  // LCOV_EXCL_BR_LINE
                            // LCOV_EXCL_START
                            // This case should not be possible because at most one
                            // index is placed into hashSetArray per
                            // numRemainingHexes.
                            H3_MEMORY(free)(remainingHexes);
                            H3_MEMORY(free)(hashSetArray);
                            return COMPACT_LOOP_EXCEEDED;
                            // LCOV_EXCL_STOP
                        }
                        H3Index tempIndex =
                            hashSetArray[loc] & H3_RESERVED_MASK_NEGATIVE;
                        if (tempIndex == parent) {
                            int count = H3_GET_RESERVED_BITS(hashSetArray[loc]) + 1;
                            int limitCount = 7;
                            if (H3_EXPORT(h3IsPentagon)(
                                    tempIndex & H3_RESERVED_MASK_NEGATIVE)) {
                                limitCount--;
                            }
                            // One is added to count for this check to match one
                            // being added to count later in this function when
                            // checking for all children being present.
                            if (count + 1 > limitCount) {
                                // Only possible on duplicate input
                                H3_MEMORY(free)(remainingHexes);
                                H3_MEMORY(free)(hashSetArray);
                                return COMPACT_DUPLICATE;
                            }
                            H3_SET_RESERVED_BITS(parent, count);
                            hashSetArray[loc] = H3_NULL;
                        } else {
                            loc = (loc + 1) % numRemainingHexes;
                        }
                        loopCount++;
                    }
                    hashSetArray[loc] = parent;
                }
            }
            // Determine which parent hexagons have a complete set
            // of children and put them in the compactableHexes array
            int compactableCount = 0;
            int maxCompactableCount =
                numRemainingHexes / 6;  // Somehow all pentagons; conservative
            if (maxCompactableCount == 0) {
                memcpy(compactedSetOffset, remainingHexes,
                       numRemainingHexes * sizeof(remainingHexes[0]));
                break;
            }
            H3Index* compactableHexes =
                H3_MEMORY(calloc)(maxCompactableCount, sizeof(H3Index));
            if (!compactableHexes) {
                H3_MEMORY(free)(remainingHexes);
                H3_MEMORY(free)(hashSetArray);
                return COMPACT_ALLOC_FAILED;
            }
            for (int i = 0; i < numRemainingHexes; i++) {
                if (hashSetArray[i] == 0) continue;
                int count = H3_GET_RESERVED_BITS(hashSetArray[i]) + 1;
                // Include the deleted direction for pentagons as implicitly "there"
                if (H3_EXPORT(h3IsPentagon)(hashSetArray[i] &
                                            H3_RESERVED_MASK_NEGATIVE)) {
                    // We need this later on, no need to recalculate
                    H3_SET_RESERVED_BITS(hashSetArray[i], count);
                    // Increment count after setting the reserved bits,
                    // since count is already incremented above, so it
                    // will be the expected value for a complete hexagon.
                    count++;
                }
                if (count == 7) {
                    // Bingo! Full set!
                    compactableHexes[compactableCount] =
                        hashSetArray[i] & H3_RESERVED_MASK_NEGATIVE;
                    compactableCount++;
                }
            }
            // Uncompactable hexes are immediately copied into the
            // output compactedSetOffset
            int uncompactableCount = 0;
            for (int i = 0; i < numRemainingHexes; i++) {
                H3Index currIndex = remainingHexes[i];
                if (currIndex != H3_NULL) {
                    H3Index parent = H3_EXPORT(h3ToParent)(currIndex, parentRes);
                    // Modulus hash the parent into the temp array
                    // to determine if this index was included in
                    // the compactableHexes array
                    int loc = (int)(parent % numRemainingHexes);
                    int loopCount = 0;
                    bool isUncompactable = true;
                    do {
                        if (loopCount > numRemainingHexes) {  // LCOV_EXCL_BR_LINE
                            // LCOV_EXCL_START
                            // This case should not be possible because at most one
                            // index is placed into hashSetArray per input hexagon.
                            H3_MEMORY(free)(compactableHexes);
                            H3_MEMORY(free)(remainingHexes);
                            H3_MEMORY(free)(hashSetArray);
                            return COMPACT_LOOP_EXCEEDED;
                            // LCOV_EXCL_STOP
                        }
                        H3Index tempIndex =
                            hashSetArray[loc] & H3_RESERVED_MASK_NEGATIVE;
                        if (tempIndex == parent) {
                            int count = H3_GET_RESERVED_BITS(hashSetArray[loc]) + 1;
                            if (count == 7) {
                                isUncompactable = false;
                            }
                            break;
                        } else {
                            loc = (loc + 1) % numRemainingHexes;
                        }
                        loopCount++;
                    } while (hashSetArray[loc] != parent);
                    if (isUncompactable) {
                        compactedSetOffset[uncompactableCount] = remainingHexes[i];
                        uncompactableCount++;
                    }
                }
            }
            // Set up for the next loop
            memset(hashSetArray, 0, numHexes * sizeof(H3Index));
            compactedSetOffset += uncompactableCount;
            memcpy(remainingHexes, compactableHexes,
                   compactableCount * sizeof(H3Index));
            numRemainingHexes = compactableCount;
            H3_MEMORY(free)(compactableHexes);
        }
        H3_MEMORY(free)(remainingHexes);
        H3_MEMORY(free)(hashSetArray);
        return COMPACT_SUCCESS;
        */
        todo!()
    }

    /**
     * uncompact takes a compressed set of hexagons and expands back to the
     * original set of hexagons.
     * @param compactedSet Set of hexagons
     * @param numHexes The number of hexes in the input set
     * @param h3Set Output array of decompressed hexagons (preallocated)
     * @param maxHexes The size of the output array to bound check against
     * @param res The hexagon resolution to decompress to
     * @return An error code if output array is too small or any hexagon is
     * smaller than the output resolution.
     */
    pub fn uncompact(
        compactedSet: Vec<H3Index>,
        res: Resolution,
        maxHexes: usize,
    ) -> Result<Vec<H3Index>, i32> {
        let numHexes = compactedSet.len();

        let mut h3Set = Vec::new();

        for i in 0..numHexes {
            if compactedSet[i] == H3Index::H3_NULL {
                continue;
            }

            if h3Set.len() > maxHexes {
                // We went too far, abort!
                return Err(-1);
            }

            let currentRes = compactedSet[i].get_resolution();
            if !currentRes._isValidChildRes(&res) {
                // Nonsensical. Abort.
                return Err(-2);
            }

            if currentRes == res {
                // Just copy and move along
                h3Set.push(compactedSet[i]);
            } else {
                // Bigger hexagon to reduce in size
                let numHexesToGen = compactedSet[i].maxH3ToChildrenSize(res);

                if h3Set.len() + numHexesToGen as usize > maxHexes {
                    // We're about to go too far, abort!
                    return Err(-1);
                }

                todo!()
                //H3_EXPORT(h3ToChildren)(compactedSet[i], res, h3Set + outOffset);
            }
        }

        Ok(h3Set)
    }

    /**
     * h3ToChildren takes the given hexagon id and generates all of the children
     * at the specified resolution storing them into the provided memory pointer.
     * It's assumed that maxH3ToChildrenSize was used to determine the allocation.
     *
     * @param h H3Index to find the children of
     * @param childRes int the child level to produce
     * @param children H3Index* the memory to store the resulting addresses in
     */
    pub fn h3ToChildren(&self, childRes: Resolution) -> Vec<H3Index> {
        let parentRes = self.get_resolution();

        let mut results = Vec::new();

        if !parentRes._isValidChildRes(&childRes) {
            return results;
        } else if parentRes == childRes {
            results.push(*self);
            return results;
        }

        let bufferSize = self.maxH3ToChildrenSize(childRes);
        let bufferChildStep = bufferSize / 7;
        let isAPentagon = self.is_pentagon();

        for i in 0..7 {
            if isAPentagon && i == usize::from(Direction::K_AXES_DIGIT) {
                /*
                H3Index* nextChild = children + bufferChildStep;
                while (children < nextChild) {
                    *children = H3_NULL;
                    children++;
                }
                */
            } else {
                let children = self.makeDirectChild(i as u64).h3ToChildren(childRes);
                results.extend(children);
                //H3_EXPORT(h3ToChildren)(makeDirectChild(h, i), childRes, children);
                //children += bufferChildStep;
            }
        }

        results
    }

    /**
     * makeDirectChild takes an index and immediately returns the immediate child
     * index based on the specified cell number. Bit operations only, could generate
     * invalid indexes if not careful (deleted cell under a pentagon).
     *
     * @param h H3Index to find the direct child of
     * @param cellNumber int id of the direct child (0-6)
     *
     * @return The new H3Index for the child
     */
    fn makeDirectChild(&self, cellNumber: u64) -> H3Index {
        let childRes = self.get_resolution() + 1;
        let mut childH = *self;
        childH.set_resolution(childRes);
        childH.set_index_digit(childRes, cellNumber);

        childH
    }

    /**
     * uncompact takes a compressed set of hexagons and expands back to the
     * original set of hexagons.
     * @param compactedSet Set of hexagons
     * @param numHexes The number of hexes in the input set
     * @param h3Set Output array of decompressed hexagons (preallocated)
     * @param maxHexes The size of the output array to bound check against
     * @param res The hexagon resolution to decompress to
     * @return An error code if output array is too small or any hexagon is smaller than the output resolution.
     */
    pub fn uncompact_x(compactedSet: Vec<H3Index>, res: Resolution) -> Result<Vec<H3Index>, i32> {
        let mut results = Vec::new();

        for h in compactedSet {
            if h == H3Index::H3_NULL {
                continue;
            }

            let currentRes = h.get_resolution();
            if !currentRes._isValidChildRes(&res) {
                // Nonsensical. Abort.
                return Err(-2);
            }

            if currentRes == res {
                // Just copy and move along
                results.push(h);
            } else {
                // Bigger hexagon to reduce in size
                todo!()
                /*
                let numHexesToGen = H3_EXPORT(maxH3ToChildrenSize)(compactedSet[i], res);
                if (outOffset + numHexesToGen > maxHexes) {
                    // We're about to go too far, abort!
                    return Err(-1);
                }
                H3_EXPORT(h3ToChildren)(compactedSet[i], res, h3Set + outOffset);
                outOffset += numHexesToGen;
                */
            }
        }

        Ok(results)
    }

    /**
     * Find all icosahedron faces intersected by a given H2 index, represented
     * as integers from -1-19. The array is sparse; since 0 is a valid value,
     * invalid array values are represented as -2. It is the responsibility of
     * the caller to filter out invalid values.
     *
     * @param h2 The H3 index
     * @param out Output array. Must be of size maxFaceCount(h2).
     */
    pub fn h3GetFaces(&self) -> HashSet<i32> {
        let mut res = self.get_resolution();
        let isPentagon = self.is_pentagon();

        // We can't use the vertex-based approach here for class II pentagons,
        // because all their vertices are on the icosahedron edges. Their
        // direct child pentagons cross the same faces, so use those instead.
        if isPentagon && !res.isResClassIII() {
            // Note that this would not work for res 14, but this is only run on
            // Class II pentagons, it should never be invoked for a res 14 index.
            let child_pentagon = self.makeDirectChild(0);
            let out = child_pentagon.h3GetFaces();
            return out;
        }

        let mut out = HashSet::new();

        // convert to FaceIJK
        let mut fijk = self._h3ToFaceIjk();

        // Get all vertices as FaceIJK addresses. For simplicity, always
        // initialize the array with 5 verts, ignoring the last one for pentagons
        if isPentagon {
            for vert in fijk._faceIjkPentToVerts(&mut res).iter_mut() {
                // Adjust overage, determining whether this vertex is on another face
                vert._adjustPentVertOverage(res);

                // Save the face to the output array
                out.insert(vert.face);
            }
        } else {
            for vert in fijk._faceIjkToVerts(&mut res).iter_mut() {
                // Adjust overage, determining whether this vertex is on another face
                vert._adjustOverageClassII(res, false, true);

                // Save the face to the output array
                out.insert(vert.face);
            }
        }

        out
    }

    /**
     * _hexRadiusKm returns the radius of a given hexagon in Km
     *
     * @param h3Index the index of the hexagon
     * @return the radius of the hexagon in Km
     */
    pub(crate) fn _hexRadiusKm(&self) -> f64 {
        // There is probably a cheaper way to determine the radius of a hexagon, but this way is conceptually simple
        let h3Center: GeoCoord = self.h3ToGeo();
        let h3Boundary: GeoBoundary = self.h3ToGeoBoundary();
        GeoCoord::pointDistKm(&h3Center, &h3Boundary.verts[0])
    }
}

impl From<H3Index> for u64 {
    fn from(h3: H3Index) -> Self {
        h3.0
    }
}

impl ToString for H3Index {
    fn to_string(&self) -> String {
        format!("{:x}", self.0)
    }
}

impl FromStr for H3Index {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u64 = u64::from_str_radix(s, 16).map_err(|_| ())?;
        Ok(H3Index(n))
    }
}

/// H3 index modes
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum H3Mode {
    H3_HEXAGON_MODE = 1,
    H3_UNIEDGE_MODE = 2,
    H3_EDGE_MODE = 3,
    H3_VERTEX_MODE = 4,
}

impl From<u64> for H3Mode {
    fn from(v: u64) -> Self {
        match v {
            1 => H3Mode::H3_HEXAGON_MODE,
            2 => H3Mode::H3_UNIEDGE_MODE,
            3 => H3Mode::H3_EDGE_MODE,
            4 => H3Mode::H3_VERTEX_MODE,
            _ => panic!("Unexpected value {} for H3Mode", v),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const PADDED_COUNT: usize = 16;

    #[test]
    fn pentagon_indexes_property_tests() {
        let expectedCount = H3Index::pentagonIndexCount();
        assert_eq!(expectedCount, 12); // this is a constant, but the assert is just for our info

        for res in 0..=15 {
            let h3Indexes = H3Index::getPentagonIndexes(res.into());

            let mut numFound = 0;

            for i in 0..PADDED_COUNT {
                let h3Index = h3Indexes[i];

                if h3Index != H3Index::H3_NULL {
                    eprintln!("h3Index = {:?}", h3Index);

                    numFound += 1;
                    assert!(h3Index.is_valid(), "index should be valid");
                    assert!(h3Index.is_pentagon(), "index should be pentagon");
                    assert_eq!(
                        h3Index.get_resolution(),
                        res.into(),
                        "index should have correct resolution"
                    );

                    // verify uniqueness
                    for j in i + 1..PADDED_COUNT {
                        if h3Indexes[j] == h3Index {
                            assert!(false, "index should be seen only once");
                        }
                    }
                }
            }

            assert_eq!(
                numFound, expectedCount,
                "there should be exactly 12 pentagons for resolution {}",
                res
            );
        }
    }

    #[test]
    fn invalid_pentagons() {
        let h3 = H3Index(0);
        assert!(!h3.is_pentagon(), "0 is not a pentagon");

        let h3 = H3Index(0x7fffffffffffffff);
        assert!(!h3.is_pentagon(), "all but high bit is not a pentagon");
    }

    #[test]
    fn bit_twiddling() {
        assert_eq!(H3Index::H3_MODE_OFFSET, 59);
        assert_eq!(H3Index::H3_MODE_MASK, 8646911284551352320);
        assert_eq!(H3Index::H3_MODE_MASK_NEGATIVE, 9799832789158199295);

        let mut h = H3Index::H3_NULL;
        h.set_mode(H3Mode::H3_EDGE_MODE);
        assert_eq!(h.0, 3 << 59); // edge mode is 3, placement is 59 bits shifted left

        let mode = h.get_mode();
        assert_eq!(mode, H3Mode::H3_EDGE_MODE);

        h.set_resolution(Resolution::R13);
        assert_eq!(h.get_resolution(), Resolution::R13);

        h.set_base_cell(BaseCell::from(123));
        assert_eq!(h.get_base_cell(), BaseCell::from(123));

        let digit = Direction::JK_AXES_DIGIT;
        h.set_index_digit(Resolution::R0, digit.into());
        assert_eq!(h.get_index_digit(Resolution::R0), digit);
    }

    #[test]
    fn test_index_digits() {
        let mut h = H3Index::H3_INIT;

        for res in Resolution::RESOLUTIONS.iter() {
            for dir in Direction::VALID_DIRECTIONS.iter() {
                h.set_index_digit(*res, u64::from(*dir));
                assert_eq!(h.get_index_digit(*res), *dir);
            }
        }
    }

    fn verifyCountAndUniqueness(children: &Vec<H3Index>, paddedCount: usize, expectedCount: usize) {
        let mut numFound = 0;
        for i in 0..paddedCount {
            let currIndex = children[i];
            if currIndex == H3Index::H3_NULL {
                continue;
            }
            numFound += 1;

            // verify uniqueness
            let mut indexSeen = 0;
            for j in i + 1..paddedCount {
                if children[j] == currIndex {
                    indexSeen += 1;
                }
            }
            assert_eq!(indexSeen, 0, "index was seen only once");
        }
        assert_eq!(numFound, expectedCount, "got expected number of children");
    }

    const sf: GeoCoord = GeoCoord::new(0.659966917655, 2. * 3.14159 - 2.1364398519396);
    //let sfHex8 : H3Index = sf.geoToH3(8);

    #[test]
    fn geoToH3ExtremeCoordinates() {
        // Check that none of these cause crashes.
        let g = GeoCoord::new(0., 1e45);
        let _h3 = g.geoToH3(Resolution::R14);

        let g2 = GeoCoord::new(1e46, 1e45);
        let _h3 = g2.geoToH3(Resolution::R15);

        let g4 = GeoCoord::new(2., -3e39);
        let _h3 = g4.geoToH3(Resolution::R0);
    }

    #[test]
    fn h3IsValidDigits() {
        let geoCoord = GeoCoord::default();
        let h3 = geoCoord.geoToH3(Resolution::R1);

        // Set a bit for an unused digit to something else.
        let h3 = h3.0 ^ 1;
        let h3 = H3Index(h3);
        assert!(!h3.is_valid(), "h3IsValid failed on invalid unused digits");
    }

    #[test]
    fn h3IsValidAtResolution() {
        for i in Resolution::RESOLUTIONS.iter() {
            let geoCoord = GeoCoord::default();
            let h3 = geoCoord.geoToH3(*i);

            assert!(h3.is_valid(), "h3IsValid failed on resolution {:?}", i);
        }
    }

    #[test]
    fn h3IsValidBaseCell() {
        for i in 0..BaseCell::NUM_BASE_CELLS {
            let mut h = H3Index::H3_INIT;
            h.set_mode(H3Mode::H3_HEXAGON_MODE);
            h.set_base_cell(i.into());

            assert!(h.is_valid(), "h3IsValid failed on base cell {}", i);

            let recovered = h.get_base_cell().0;
            assert_eq!(recovered, i as i32, "failed to recover base cell");
        }
    }

    #[test]
    fn h3IsValidBaseCellInvalid() {
        let mut hWrongBaseCell = H3Index::H3_INIT;
        hWrongBaseCell.set_mode(H3Mode::H3_HEXAGON_MODE);
        hWrongBaseCell.set_base_cell(BaseCell::NUM_BASE_CELLS.into());
        assert!(
            !hWrongBaseCell.is_valid(),
            "h3IsValid failed on invalid base cell"
        );
    }

    #[test]
    fn h3IsValidWithMode() {
        const H3_MODES: [H3Mode; 4] = [
            H3Mode::H3_HEXAGON_MODE,
            H3Mode::H3_UNIEDGE_MODE,
            H3Mode::H3_EDGE_MODE,
            H3Mode::H3_VERTEX_MODE,
        ];

        for mode in H3_MODES.iter() {
            let mut h = H3Index::H3_INIT;
            h.set_mode(*mode);

            if *mode == H3Mode::H3_HEXAGON_MODE {
                assert!(h.is_valid(), "h3IsValid succeeds on valid mode");
            } else {
                assert!(!h.is_valid(), "h3IsValid failed on mode {:?}", mode);
            }
        }
    }

    #[test]
    fn h3IsValidHighBit() {
        let mut h = H3Index::H3_INIT;
        h.set_mode(H3Mode::H3_HEXAGON_MODE);
        h.set_high_bit(1);

        assert!(!h.is_valid(), "h3IsValid failed on high bit");
    }

    #[test]
    fn h3BadDigitInvalid() {
        let mut h = H3Index::H3_INIT;

        // By default the first index digit is out of range.
        h.set_mode(H3Mode::H3_HEXAGON_MODE);
        h.set_resolution(Resolution::R1);
        assert!(!h.is_valid(), "h3IsValid failed on too large digit");
    }

    #[test]
    fn h3DeletedSubsequenceInvalid() {
        let h = H3Index::setH3Index(Resolution::R1, BaseCell::new(4), Direction::K_AXES_DIGIT);

        // Create an index located in a deleted subsequence of a pentagon.
        assert!(!h.is_valid(), "h3IsValid failed on deleted subsequence");
    }

    #[test]
    fn setH3Index() {
        let h = H3Index::setH3Index(Resolution::R5, BaseCell::new(12), Direction::K_AXES_DIGIT);

        assert_eq!(h.get_resolution(), Resolution::R5, "resolution as expected");
        assert_eq!(h.get_base_cell().0, 12, "base cell as expected");
        assert_eq!(h.get_mode(), H3Mode::H3_HEXAGON_MODE, "mode as expected");

        for i in 1..=5 {
            assert_eq!(
                h.get_index_digit(i.into()),
                Direction::K_AXES_DIGIT,
                "digit as expected"
            );
        }

        for i in 6..=Resolution::MAX_H3_RES {
            assert_eq!(
                h.get_index_digit(i.into()),
                Direction::INVALID_DIGIT,
                "blanked digit as expected"
            );
        }

        assert_eq!(h.0, 0x85184927fffffff, "index matches expected");
    }

    //#[test]
    fn h3IsResClassIII() {
        let coord = GeoCoord::default();

        for i in Resolution::RESOLUTIONS.iter() {
            let h = coord.geoToH3(*i);

            //t_assert(H3_EXPORT(h3IsResClassIII)(h) == isResClassIII(i), "matches existing definition");
        }
    }

    #[test]
    fn h3IsValidReservedBits() {
        for i in 0..8 {
            let mut h = H3Index::H3_INIT;
            h.set_mode(H3Mode::H3_HEXAGON_MODE);
            h.set_reserved_bits(i);

            if i == 0 {
                assert!(h.is_valid(), "h3IsValid succeeds on valid reserved bits");
            } else {
                assert!(!h.is_valid(), "h3IsValid failed on reserved bits {}", i);
            }
        }
    }

    #[test]
    fn h3ToString() {
        let h = H3Index(0xcafe);
        let buf = h.to_string();
        assert_eq!(buf, "cafe", "h3ToString failed to produce base 16 results");

        let h = H3Index(0xffffffffffffffff);
        let buf = h.to_string();
        assert_eq!(buf, "ffffffffffffffff", "h3ToString failed on large input");
    }

    #[test]
    fn stringToH3() {
        let h = "".parse::<H3Index>();
        assert!(h.is_err(), "got an index from nothing");

        let h = "**".parse::<H3Index>();
        assert!(h.is_err(), "got an index from junk");

        let h = "ffffffffffffffff".parse::<H3Index>();
        assert_eq!(h, Ok(H3Index(0xffffffffffffffff)), "failed on large input");
    }

    mod h3index {
        use super::*;
    }

    mod h3ToParent {
        use super::*;
        #[test]
        fn h3ToParent_ancestorsForEachRes() {
            //H3Index child;
            //H3Index comparisonParent;
            //H3Index parent;

            for res in Resolution::RESOLUTIONS.iter().skip(1) {
                for step in 0..*res as i32 {
                    let mut child = sf.geoToH3(*res);
                    let parent = child.h3ToParent(*res - step);

                    let comparison_parent = sf.geoToH3(*res - step);
                    assert_eq!(parent, comparison_parent, "Got expected parent");
                }
            }
        }

        #[test]
        fn h3ToParent_invalidInputs() {
            let mut child = sf.geoToH3(Resolution::R5);

            assert_eq!(
                child.h3ToParent(Resolution::R6),
                H3Index::H3_NULL,
                "Higher resolution fails"
            );
            //assert_eq!(child.h3ToParent(-1), 0, "Invalid resolution fails");
            assert_eq!(
                child.h3ToParent(Resolution::R15),
                H3Index::H3_NULL,
                "Invalid resolution fails"
            );
            //assert_eq!( child.h3ToParent(16), 0, "Invalid resolution fails");
        }
    }

    mod h3_to_children {
        use super::*;

        fn verifyCountAndUniqueness(
            children: &Vec<H3Index>,
            paddedCount: usize,
            expectedCount: usize,
        ) {
            let mut num_found = 0;
            for i in 0..paddedCount {
                let currIndex = children[i];

                if currIndex == H3Index::H3_NULL {
                    continue;
                }

                num_found += 1;

                // verify uniqueness
                let mut indexSeen = 0;
                for j in (i + 1)..paddedCount {
                    if children[j] == currIndex {
                        indexSeen += 1;
                    }
                }
                assert_eq!(indexSeen, 0, "index was seen only once");
            }
            assert_eq!(num_found, expectedCount, "got expected number of children");
        }

        #[test]
        fn multipleResSteps() {
            // Lots of children. Will just confirm number and uniqueness
            const EXPECTED_COUNT: usize = 49;
            const PADDED_COUNT: usize = 60;

            let sfHex8 = sf.geoToH3(Resolution::R8);
            let children = sfHex8.h3ToChildren(Resolution::R10);

            verifyCountAndUniqueness(&children, PADDED_COUNT, EXPECTED_COUNT);
        }

        #[test]
        fn sameRes() {
            const EXPECTED_COUNT: usize = 1;
            const PADDED_COUNT: usize = 7;

            let sfHex8 = sf.geoToH3(Resolution::R8);
            let children = sfHex8.h3ToChildren(Resolution::R8);

            verifyCountAndUniqueness(&children, PADDED_COUNT, EXPECTED_COUNT);
        }

        #[test]
        fn childResTooCoarse() {
            const EXPECTED_COUNT: usize = 0;
            const PADDED_COUNT: usize = 7;

            let sfHex8 = sf.geoToH3(Resolution::R8);
            let children = sfHex8.h3ToChildren(Resolution::R7);

            verifyCountAndUniqueness(&children, PADDED_COUNT, EXPECTED_COUNT);
        }

        //#[test]
        fn childResTooFine() {
            const EXPECTED_COUNT: usize = 0;
            const PADDED_COUNT: usize = 7;

            let sfHex8 = sf.geoToH3(Resolution::R8);
            let children = sfHex8.h3ToChildren(Resolution::R15);

            let sfHexMax = sf.geoToH3(Resolution::R15);

            //H3_EXPORT(h3ToChildren)(sfHexMax, MAX_H3_RES + 1, children);
            //verifyCountAndUniqueness(&children, PADDED_COUNT, EXPECTED_COUNT);
        }

        //#[test]
        fn pentagonChildren() {
            let pentagon = H3Index::setH3Index(Resolution::R1, 4.into(), Direction::CENTER_DIGIT);
            todo!()

            //const expectedCount : usize = (5 * 7) + 6;
            //const paddedCount : usize = pentagon H3_EXPORT(maxH3ToChildrenSize)(pentagon, 3);

            //H3Index* children = calloc(paddedCount, sizeof(H3Index));
            //H3_EXPORT(h3ToChildren)(sfHex8, 10, children);
            //H3_EXPORT(h3ToChildren)(pentagon, 3, children);

            //verifyCountAndUniqueness(children, paddedCount, expectedCount);
            //free(children);
        }
    }
}

mod algos;
mod h3UniEdge;
mod localij;
mod vertex;

use crate::{baseCells::baseCellData, direction::NUM_DIGITS, overage::Overage, GeoCoord};
use crate::{baseCells::BaseCell, GeoBoundary};
use crate::{constants::*, direction::Direction};
use crate::{coordijk::CoordIJK, faceijk::*};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct H3Index(u64);

pub type Resolution = usize;

// constants and macros for bitwise manipulation of H3Index

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
const H3_HIGH_BIT_MASK: u64 = 1 << H3_MAX_OFFSET;

/// 0 in the highest bit, 1's everywhere else.
const H3_HIGH_BIT_MASK_NEGATIVE: u64 = !H3_HIGH_BIT_MASK;

/// 1's in the 4 mode bits, 0's everywhere else.
const H3_MODE_MASK: u64 = 15 << H3_MODE_OFFSET;

/// 0's in the 4 mode bits, 1's everywhere else.
const H3_MODE_MASK_NEGATIVE: u64 = !H3_MODE_MASK;

/// 1's in the 7 base cell bits, 0's everywhere else.
const H3_BC_MASK: u64 = 127 << H3_BC_OFFSET;

/// 0's in the 7 base cell bits, 1's everywhere else.
const H3_BC_MASK_NEGATIVE: u64 = !H3_BC_MASK;

/// 1's in the 4 resolution bits, 0's everywhere else.
const H3_RES_MASK: u64 = 15 << H3_RES_OFFSET;

/// 0's in the 4 resolution bits, 1's everywhere else.
const H3_RES_MASK_NEGATIVE: u64 = !H3_RES_MASK;

/// 1's in the 3 reserved bits, 0's everywhere else.
const H3_RESERVED_MASK: u64 = 7 << H3_RESERVED_OFFSET;

/// 0's in the 3 reserved bits, 1's everywhere else.
const H3_RESERVED_MASK_NEGATIVE: u64 = !H3_RESERVED_MASK;

/// 1's in the 3 bits of res 15 digit bits, 0's everywhere else.
const H3_DIGIT_MASK: u64 = 7;

/// 0's in the 7 base cell bits, 1's everywhere else.
const H3_DIGIT_MASK_NEGATIVE: u64 = !H3_DIGIT_MASK;

impl H3Index {
    /// Invalid index used to indicate an error from geoToH3 and related functions
    /// or missing data in arrays of h3 indices. Analogous to NaN in floating point.
    pub const H3_NULL: H3Index = H3Index(0);

    // Return codes for compact
    const COMPACT_SUCCESS: i32 = 0;
    const COMPACT_LOOP_EXCEEDED: i32 = -1;
    const COMPACT_DUPLICATE: i32 = -2;
    const COMPACT_ALLOC_FAILED: i32 = -3;

    /**
     * H3 index with mode 0, res 0, base cell 0, and 7 for all index digits.
     * Typically used to initialize the creation of an H3 cell index, which
     * expects all direction digits to be 7 beyond the cell's resolution.
     */
    pub fn H3_INIT() -> Self {
        Self(35184372088831)
    }

    /// Gets the highest bit of the H3 index.
    pub(crate) fn H3_GET_HIGH_BIT(&self) -> u64 {
        //const H3_GET_HIGH_BIT(h3) ((int)((((h3)&H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET)))
        (self.0 & H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET
    }

    /// Sets the highest bit of the h3 to v.
    pub(crate) fn H3_SET_HIGH_BIT(&mut self, v: u64) {
        self.0 = (self.0 & H3_HIGH_BIT_MASK_NEGATIVE) | (v << H3_MAX_OFFSET);
        //(h3) = (((h3)&H3_HIGH_BIT_MASK_NEGATIVE) | \
        //        (((uint64_t)(v)) << H3_MAX_OFFSET))
    }

    /// Gets the integer mode of h3.
    //pub(crate) fn H3_GET_MODE(&self) -> u64 {
    //    (&self.0 & H3_MODE_MASK) >> H3_MODE_OFFSET
    // }
    pub(crate) fn H3_GET_MODE(&self) -> H3_MODE {
        let v = (&self.0 & H3_MODE_MASK) >> H3_MODE_OFFSET;
        match v {
            1 => H3_MODE::HEXAGON,
            2 => H3_MODE::UNIEDGE,
            3 => H3_MODE::EDGE,
            4 => H3_MODE::VERTEX,
            v => panic!("Unexpected mode value '{}'", v),
        }
    }

    // Sets the integer mode of h3 to v.
    //pub(crate) fn H3_SET_MODE(&mut self, v: u64) {
    //(h3) = (((h3)&H3_MODE_MASK_NEGATIVE) | (((uint64_t)(v)) << H3_MODE_OFFSET))
    //self.0 = (self.0 & H3_MODE_MASK_NEGATIVE) | (v << H3_MODE_OFFSET);
    //}

    /// Sets the integer mode of h3 to v.
    pub(crate) fn H3_SET_MODE(&mut self, mode: H3_MODE) {
        let v = match mode {
            H3_MODE::HEXAGON => 1,
            H3_MODE::UNIEDGE => 2,
            H3_MODE::EDGE => 3,
            H3_MODE::VERTEX => 4,
        };

        //(h3) = (((h3)&H3_MODE_MASK_NEGATIVE) | (((uint64_t)(v)) << H3_MODE_OFFSET))
        self.0 = (self.0 & H3_MODE_MASK_NEGATIVE) | (v << H3_MODE_OFFSET);
    }

    /// Gets the integer base cell of h3.
    pub(crate) fn H3_GET_BASE_CELL(&self) -> BaseCell {
        let v = (self.0 & H3_BC_MASK) >> H3_BC_OFFSET;
        BaseCell(v as i32)
    }

    /// Sets the integer base cell of h3 to bc.
    //pub(crate) fn H3_SET_BASE_CELL(&mut self, bc: u64)
    pub(crate) fn H3_SET_BASE_CELL(&mut self, bc: BaseCell) {
        let BaseCell(bc) = bc;
        //(h3) = (((h3)&H3_BC_MASK_NEGATIVE) | (((uint64_t)(bc)) << H3_BC_OFFSET))
        self.0 = (self.0 & H3_BC_MASK_NEGATIVE) | ((bc as u64) << H3_BC_OFFSET);
    }

    /// Gets the integer resolution of h3.
    pub(crate) fn H3_GET_RESOLUTION(&self) -> Resolution {
        ((self.0 & H3_RES_MASK) >> H3_RES_OFFSET) as Resolution
    }

    /// Sets the integer resolution of h3.
    pub(crate) fn H3_SET_RESOLUTION(&mut self, res: Resolution) {
        //(h3) = (((h3)&H3_RES_MASK_NEGATIVE) | (((uint64_t)(res)) << H3_RES_OFFSET))
        self.0 = (self.0 & H3_RES_MASK_NEGATIVE) | ((res as u64) << H3_RES_OFFSET);
    }

    /// Gets the resolution res integer digit (0-7) of h3.
    pub(crate) fn H3_GET_INDEX_DIGIT(&self, res: Resolution) -> Direction {
        //((Direction)((((h3) >> ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)) & \
        //              H3_DIGIT_MASK)))
        let d = (self.0 >> ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)) & H3_DIGIT_MASK;
        d.into()
    }

    /// Sets a value in the reserved space. Setting to non-zero may produce invalid indexes.
    pub(crate) fn H3_SET_RESERVED_BITS(&mut self, v: u64) {
        self.0 = (self.0 & H3_RESERVED_MASK_NEGATIVE) | (v << H3_RESERVED_OFFSET);
    }

    /// Gets a value in the reserved space. Should always be zero for valid indexes.
    pub(crate) fn H3_GET_RESERVED_BITS(&self) -> u64 {
        (self.0 & H3_RESERVED_MASK) >> H3_RESERVED_OFFSET
    }

    /// Sets the resolution res digit of h3 to the integer digit (0-7)
    pub(crate) fn H3_SET_INDEX_DIGIT(&mut self, res: Resolution, digit: u64) {
        self.0 = (self.0 & !(H3_DIGIT_MASK << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)))
            | (digit << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET));
    }

    /**
     * Returns the H3 resolution of an H3 index.
     * @param h The H3 index.
     * @return The resolution of the H3 index argument.
     */
    pub(crate) fn h3GetResolution(&self) -> Resolution {
        self.H3_GET_RESOLUTION()
    }

    /**
     * Returns the H3 base cell "number" of an H3 cell (hexagon or pentagon).
     *
     * Note: Technically works on H3 edges, but will return base cell of the
     * origin cell.
     *
     * @param h The H3 cell.
     * @return The base cell "number" of the H3 cell argument.
     */
    pub(crate) fn h3GetBaseCell(&self) -> BaseCell {
        self.H3_GET_BASE_CELL()
    }

    /**
     * Converts a string representation of an H3 index into an H3 index.
     * @param str The string representation of an H3 index.
     * @return The H3 index corresponding to the string argument, or H3_NULL if
     * invalid.
     */
    // TODO: this should be handled with `Parse`
    pub(crate) fn stringToH3(s: &str) -> Self {
        //H3Index h = H3_NULL;
        // If failed, h will be unmodified and we should return H3_NULL anyways.
        //h
        todo!("sscanf(str, \"%\" PRIx64, &h);")
    }

    /**
     * Converts an H3 index into a string representation.
     * @param h The H3 index to convert.
     * @param str The string representation of the H3 index.
     * @param sz Size of the buffer `str`
     */
    pub(crate) fn h3ToString(&self) -> String {
        todo!("sprintf(str, \"%\" PRIx64, h);");
    }

    /**
     * Returns whether or not an H3 index is a valid cell (hexagon or pentagon).
     * @param h The H3 index to validate.
     * @return 1 if the H3 index if valid, and 0 if it is not.
     */
    pub(crate) fn h3IsValid(&self) -> bool {
        if self.H3_GET_HIGH_BIT() != 0 {
            return false;
        }

        if self.H3_GET_MODE() != H3_MODE::HEXAGON {
            return false;
        }

        if self.H3_GET_RESERVED_BITS() != 0 {
            return false;
        }

        let baseCell = self.H3_GET_BASE_CELL();
        if baseCell.0 >= NUM_BASE_CELLS {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return false;
        }

        let res = self.H3_GET_RESOLUTION();
        if res < 0 || res > MAX_H3_RES {
            // LCOV_EXCL_BR_LINE
            // Resolutions less than zero can not be represented in an index
            return false;
        }

        let mut foundFirstNonZeroDigit = false;
        for r in 1..=res {
            let digit: Direction = self.H3_GET_INDEX_DIGIT(r);

            if !foundFirstNonZeroDigit && digit != Direction::CENTER_DIGIT {
                foundFirstNonZeroDigit = true;
                if baseCell._isBaseCellPentagon() && digit == Direction::K_AXES_DIGIT {
                    return false;
                }
            }

            if digit < Direction::CENTER_DIGIT || digit >= NUM_DIGITS {
                return false;
            }
        }

        for r in res + 1..=MAX_H3_RES {
            let digit: Direction = self.H3_GET_INDEX_DIGIT(r);
            if digit != Direction::INVALID {
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
    pub(crate) fn setH3Index(res: Resolution, baseCell: i32, initDigit: Direction) -> Self {
        let mut h = H3Index::H3_INIT();

        h.H3_SET_MODE(H3_MODE::HEXAGON);
        h.H3_SET_RESOLUTION(res);
        h.H3_SET_BASE_CELL(baseCell);

        for r in 1..=res {
            h.H3_SET_INDEX_DIGIT(r, initDigit);
        }

        h
    }

    /**
     * h3ToParent produces the parent index for a given H3 index
     *
     * @param h H3Index to find parent of
     * @param parentRes The resolution to switch to (parent, grandparent, etc)
     *
     * @return H3Index of the parent, or H3_NULL if you actually asked for a child
     */
    pub(crate) fn h3ToParent(&self, parentRes: i32) -> Self {
        let childRes: u64 = self.H3_GET_RESOLUTION();

        if parentRes < 0 || parentRes > MAX_H3_RES {
            H3_NULL
        } else if parentRes > childRes {
            H3_NULL
        } else if parentRes == childRes {
            *self
        } else {
            let mut parentH = self.H3_SET_RESOLUTION(parentRes);
            for i in parentRes + 1..=childRes {
                parentH.H3_SET_INDEX_DIGIT(i, H3_DIGIT_MASK);
            }
            parentH
        }
    }

    /**
     * Determines whether one resolution is a valid child resolution of another.
     * Each resolution is considered a valid child resolution of itself.
     *
     * @param parentRes int resolution of the parent
     * @param childRes int resolution of the child
     *
     * @return The validity of the child resolution
     */
    pub(crate) fn _isValidChildRes(parentRes: Resolution, childRes: Resolution) -> bool {
        if childRes < parentRes || childRes > MAX_H3_RES {
            false
        } else {
            true
        }
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
    pub(crate) fn maxH3ToChildrenSize(&self, childRes: i32) -> i64 {
        let parentRes = self.H3_GET_RESOLUTION();
        if !H3Index::_isValidChildRes(parentRes, childRes) {
            return 0;
        }

        7_i32.pow(childRes - parentRes)
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
    pub(crate) fn makeDirectChild(&self, cellNumber: i32) -> Self {
        let childRes = self.H3_GET_RESOLUTION() + 1;
        let mut childH = self.H3_SET_RESOLUTION(childRes);
        childH.H3_SET_INDEX_DIGIT(childRes, cellNumber);
        childH
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
    pub(crate) fn h3ToChildren(&self /*h*/, childRes: i32) -> Vec<H3Index> /* children  ?? */ {
        todo!()
        /*
          int parentRes = h.H3_GET_RESOLUTION();
          if (!_isValidChildRes(parentRes, childRes)) {
          return;
          } else if (parentRes == childRes) {
        *children = h;
        return;
        }
        int bufferSize = maxH3ToChildrenSize(h, childRes);
        int bufferChildStep = (bufferSize / 7);
        int isAPentagon = h3IsPentagon(h);
        for (int i = 0; i < 7; i++) {
        if (isAPentagon && i == K_AXES_DIGIT) {
        H3Index* nextChild = children + bufferChildStep;
        while (children < nextChild) {
        *children = H3_NULL;
        children++;
        }
        } else {
        h3ToChildren(makeDirectChild(h, i), childRes, children);
        children += bufferChildStep;
        }
        }
        */
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
    pub(crate) fn h3ToCenterChild(&self, childRes: Resolution) -> Self {
        let parentRes = self.H3_GET_RESOLUTION();
        if !H3Index::_isValidChildRes(parentRes, childRes) {
            return H3Index::H3_NULL;
        } else if childRes == parentRes {
            return *self;
        }

        let mut child = self.H3_SET_RESOLUTION(childRes);
        for i in parentRes + 1..=childRes {
            child.H3_SET_INDEX_DIGIT(i, 0);
        }

        child
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
    pub(crate) fn compact(&self /*h3Set*/, compactedSet: &H3Index, numHexes: i32) -> u64 {
        if numHexes == 0 {
            return Self::COMPACT_SUCCESS;
        }
        let res = h3Set[0].H3_GET_RESOLUTION();
        if res == 0 {
            // No compaction possible, just copy the set to output
            for i in 0..numHexes {
                compactedSet[i] = h3Set[i];
            }

            return Self::COMPACT_SUCCESS;
        }

        todo!()

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
                       res = remainingHexes[0].H3_GET_RESOLUTION();
                       int parentRes = res - 1;
                    // Put the parents of the hexagons into the temp array
                    // via a hashing mechanism, and use the reserved bits
                    // to track how many times a parent is duplicated
                    for (int i = 0; i < numRemainingHexes; i++) {
                    H3Index currIndex = remainingHexes[i];
                    if (currIndex != 0) {
                    H3Index parent = h3ToParent(currIndex, parentRes);
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
                    int count = hashSetArray[loc].H3_GET_RESERVED_BITS() + 1;
                    int limitCount = 7;
                    if (h3IsPentagon(
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
            int count = hashSetArray[i].H3_GET_RESERVED_BITS() + 1;
            // Include the deleted direction for pentagons as implicitly "there"
            if (h3IsPentagon(hashSetArray[i] &
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
                H3Index parent = h3ToParent(currIndex, parentRes);
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
                        int count = hashSetArray[loc].H3_GET_RESERVED_BITS() + 1;
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
    pub(crate) fn uncompact(
        &self, /*compactedSet*/
        numHexes: i32,
        h3Set: &mut Self,
        maxHexes: i32,
        res: Resolution,
    ) -> i32 {
        let mut outOffset = 0;
        for i in 0..numHexes {
            if compactedSet[i] == 0 {
                continue;
            }

            if outOffset >= maxHexes {
                // We went too far, abort!
                return -1;
            }

            let currentRes = compactedSet[i].H3_GET_RESOLUTION();
            if !H3Index::_isValidChildRes(currentRes, res) {
                // Nonsensical. Abort.
                return -2;
            }

            if currentRes == res {
                // Just copy and move along
                h3Set[outOffset] = compactedSet[i];
                outOffset += 1;
            } else {
                // Bigger hexagon to reduce in size
                let numHexesToGen = compactedSet[i].maxH3ToChildrenSize(res);
                if outOffset + numHexesToGen > maxHexes {
                    // We're about to go too far, abort!
                    return -1;
                }
                compactedSet[i].h3ToChildren(res, h3Set + outOffset);
                outOffset += numHexesToGen;
            }
        }

        0
    }

    /**
     * maxUncompactSize takes a compacted set of hexagons are provides an
     * upper-bound estimate of the size of the uncompacted set of hexagons.
     * @param compactedSet Set of hexagons
     * @param numHexes The number of hexes in the input set
     * @param res The hexagon resolution to decompress to
     * @return The number of hexagons to allocate memory for, or a negative
     * number if an error occurs.
     */
    pub(crate) fn maxUncompactSize(compactedSet: Vec<H3Index>, res: Resolution) -> i32 {
        let mut maxNumHexagons = 0;
        for h in compactedSet.iter() {
            if h == 0 {
                continue;
            }

            let currentRes = h.H3_GET_RESOLUTION();
            if !H3Index::_isValidChildRes(currentRes, res) {
                // Nonsensical. Abort.
                return -1;
            }
            if currentRes == res {
                maxNumHexagons += 1;
            } else {
                // Bigger hexagon to reduce in size
                let numHexesToGen = h.maxH3ToChildrenSize(res);
                maxNumHexagons += numHexesToGen;
            }
        }

        maxNumHexagons
    }

    /**
     * h3IsResClassIII takes a hexagon ID and determines if it is in a
     * Class III resolution (rotated versus the icosahedron and subject
     * to shape distortion adding extra points on icosahedron edges, making
     * them not true hexagons).
     * @param h The H3Index to check.
     * @return Returns 1 if the hexagon is class III, otherwise 0.
     */
    pub(crate) fn h3IsResClassIII(&self) -> bool {
        self.H3_GET_RESOLUTION() % 2 == 1
    }

    /**
     * h3IsPentagon takes an H3Index and determines if it is actually a
     * pentagon.
     * @param h The H3Index to check.
     * @return Returns 1 if it is a pentagon, otherwise 0.
     */
    pub(crate) fn h3IsPentagon(&self) -> bool {
        self.H3_GET_BASE_CELL()._isBaseCellPentagon() && !self._h3LeadingNonZeroDigit()
    }

    /**
     * Returns the highest resolution non-zero digit in an H3Index.
     * @param h The H3Index.
     * @return The highest resolution non-zero digit in the H3Index.
     */
    pub(crate) fn _h3LeadingNonZeroDigit(&self) -> Direction {
        for r in 1..=self.H3_GET_RESOLUTION() {
            if self.H3_GET_INDEX_DIGIT(r) {
                return self.H3_GET_INDEX_DIGIT(r);
            }
        }

        // if we're here it's all 0's
        Direction::CENTER_DIGIT
    }

    /**
     * Rotate an H3Index 60 degrees counter-clockwise about a pentagonal center.
     * @param h The H3Index.
     */
    pub(crate) fn _h3RotatePent60ccw(&mut self) -> Self {
        // rotate in place; skips any leading 1 digits (k-axis)

        let mut foundFirstNonZeroDigit = false;
        let res = self.H3_GET_RESOLUTION();

        let mut h = self.clone();

        for r in 1..=res {
            // rotate this digit
            h.H3_SET_INDEX_DIGIT(r, h.H3_GET_INDEX_DIGIT(r)._rotate60ccw());

            // look for the first non-zero digit so we
            // can adjust for deleted k-axes sequence
            // if necessary
            if !foundFirstNonZeroDigit && h.H3_GET_INDEX_DIGIT(r) != 0 {
                foundFirstNonZeroDigit = true;

                // adjust for deleted k-axes sequence
                if h._h3LeadingNonZeroDigit() == Direction::K_AXES_DIGIT {
                    h._h3Rotate60ccw();
                }
            }
        }

        h
    }

    /**
     * Rotate an H3Index 60 degrees clockwise about a pentagonal center.
     * @param h The H3Index.
     */
    pub(crate) fn _h3RotatePent60cw(&self) -> Self {
        // rotate in place; skips any leading 1 digits (k-axis)

        let mut foundFirstNonZeroDigit = false;
        let res = self.H3_GET_RESOLUTION();
        let mut h = self.clone();
        for r in 1..=res {
            // rotate this digit
            h.H3_SET_INDEX_DIGIT(r, h.H3_GET_INDEX_DIGIT(r)._rotate60cw());

            // look for the first non-zero digit so we
            // can adjust for deleted k-axes sequence
            // if necessary
            if !foundFirstNonZeroDigit && h.H3_GET_INDEX_DIGIT(r) != 0 {
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
     * Rotate an H3Index 60 degrees counter-clockwise.
     * @param h The H3Index.
     */
    pub(crate) fn _h3Rotate60ccw(&self) -> Self {
        let mut h = self.clone();
        let res = h.H3_GET_RESOLUTION();

        for r in 1..=res {
            let oldDigit: Direction = h.H3_GET_INDEX_DIGIT(r);
            h.H3_SET_INDEX_DIGIT(r, oldDigit._rotate60ccw());
        }

        h
    }

    /**
     * Rotate an H3Index 60 degrees clockwise.
     * @param h The H3Index.
     */
    pub(crate) fn _h3Rotate60cw(&self) -> Self {
        let res = self.H3_GET_RESOLUTION();
        let mut h = self.clone();
        for r in 1..=res {
            h.H3_SET_INDEX_DIGIT(r, h.H3_GET_INDEX_DIGIT(r)._rotate60cw());
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
    pub(crate) fn _h3ToFaceIjkWithInitializedFijk(&self, fijk: FaceIJK) -> bool {
        let mut ijk: CoordIJK = fijk.coord.clone();
        let res = self.H3_GET_RESOLUTION();

        // center base cell hierarchy is entirely on this face
        let mut possibleOverage = true;
        if !self.H3_GET_BASE_CELL()._isBaseCellPentagon()
            && (res == 0 || (fijk.coord.i == 0 && fijk.coord.j == 0 && fijk.coord.k == 0))
        {
            possibleOverage = false;
        }

        for r in 1..=res {
            if Self::isResClassIII(r) {
                // Class III == rotate ccw
                ijk._downAp7();
            } else {
                // Class II == rotate cw
                ijk._downAp7r();
            }

            ijk._neighbor(self.H3_GET_INDEX_DIGIT(r));
        }

        possibleOverage
    }

    /**
     * Convert an H3Index to a FaceIJK address.
     * @param h The H3Index.
     * @param fijk The corresponding FaceIJK address.
     */
    pub(crate) fn _h3ToFaceIjk(&self) -> FaceIJK {
        let baseCell = self.H3_GET_BASE_CELL();
        if baseCell < 0 || baseCell >= NUM_BASE_CELLS {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            // TODO: Indicate an error to the caller
            // To prevent reading uninitialized memory, we zero the output.
            return FaceIJK::default();
        }
        // adjust for the pentagonal missing sequence; all of sub-sequence 5 needs
        // to be adjusted (and some of sub-sequence 4 below)
        if baseCell._isBaseCellPentagon() && self._h3LeadingNonZeroDigit() == 5 {
            self._h3Rotate60cw();
        }

        // start with the "home" face and ijk+ coordinates for the base cell of c
        let mut fijk = baseCellData[baseCell.0 as usize].homeFijk;
        if !self._h3ToFaceIjkWithInitializedFijk(fijk) {
            // no overage is possible; h lies on this face
            return fijk;
        }

        // if we're here we have the potential for an "overage"; i.e., it is
        // possible that c lies on an adjacent face

        let origIJK: CoordIJK = fijk.coord;

        // if we're in Class III, drop into the next finer Class II grid
        let res = self.H3_GET_RESOLUTION();
        if Self::isResClassIII(res) {
            // Class III
            fijk.coord._downAp7r();
            res += 1;
        }

        // adjust for overage if needed
        // a pentagon base cell with a leading 4 digit requires special handling
        let pentLeading4 = baseCell._isBaseCellPentagon() && self._h3LeadingNonZeroDigit() == 4;
        if fijk._adjustOverageClassII(res, pentLeading4, 0) != Overage::NO_OVERAGE {
            // if the base cell is a pentagon we have the potential for secondary
            // overages
            if baseCell._isBaseCellPentagon() {
                while fijk._adjustOverageClassII(res, 0, 0) != Overage::NO_OVERAGE {
                    continue;
                }
            }

            if res != self.H3_GET_RESOLUTION() {
                fijk.coord._upAp7r();
            }
        } else if res != self.H3_GET_RESOLUTION() {
            fijk.coord = origIJK;
        }

        fijk
    }

    /**
     * Determines the spherical coordinates of the center point of an H3 index.
     *
     * @param h3 The H3 index.
     * @param g The spherical coordinates of the H3 cell center.
     */
    pub(crate) fn h3ToGeo(&self /*h3*/) -> GeoCoord {
        let fijk = self._h3ToFaceIjk();
        fijk._faceIjkToGeo(self.H3_GET_RESOLUTION())
    }

    /**
     * Determines the cell boundary in spherical coordinates for an H3 index.
     *
     * @param h3 The H3 index.
     * @param gb The boundary of the H3 cell in spherical coordinates.
     */
    pub(crate) fn h3ToGeoBoundary(&self) -> GeoBoundary {
        let fijk: FaceIJK = self._h3ToFaceIjk();

        if self.h3IsPentagon() {
            fijk._faceIjkPentToGeoBoundary(self.H3_GET_RESOLUTION(), 0, NUM_PENT_VERTS)
        } else {
            fijk._faceIjkToGeoBoundary(self.H3_GET_RESOLUTION(), 0, NUM_HEX_VERTS)
        }
    }

    /**
     * Returns the max number of possible icosahedron faces an H3 index
     * may intersect.
     *
     * @return int count of faces
     */
    pub(crate) fn maxFaceCount(&self) -> i32 {
        // a pentagon always intersects 5 faces, a hexagon never intersects more
        // than 2 (but may only intersect 1)
        if self.h3IsPentagon() {
            5
        } else {
            2
        }
    }

    /**
     * Find all icosahedron faces intersected by a given H3 index, represented
     * as integers from 0-19. The array is sparse; since 0 is a valid value,
     * invalid array values are represented as -1. It is the responsibility of
     * the caller to filter out invalid values.
     *
     * @param h3 The H3 index
     * @param out Output array. Must be of size maxFaceCount(h3).
     */
    pub(crate) fn h3GetFaces(&self) -> Vec<i32> {
        let res = self.H3_GET_RESOLUTION();
        let isPentagon = self.h3IsPentagon();

        // We can't use the vertex-based approach here for class II pentagons,
        // because all their vertices are on the icosahedron edges. Their
        // direct child pentagons cross the same faces, so use those instead.
        if isPentagon && !Self::isResClassIII(res) {
            // Note that this would not work for res 15, but this is only run on
            // Class II pentagons, it should never be invoked for a res 15 index.
            let childPentagon: H3Index = self.makeDirectChild(0);
            return childPentagon.h3GetFaces();
        }

        // convert to FaceIJK
        let fijk: FaceIJK = self._h3ToFaceIjk();

        // Get all vertices as FaceIJK addresses. For simplicity, always
        // initialize the array with 6 verts, ignoring the last one for pentagons
        let mut fijkVerts: [FaceIJK; NUM_HEX_VERTS];

        let vertexCount;
        if isPentagon {
            vertexCount = NUM_PENT_VERTS;
            fijk._faceIjkPentToVerts(&res, fijkVerts);
        } else {
            vertexCount = NUM_HEX_VERTS;
            fijk._faceIjkToVerts(&res, fijkVerts);
        }

        // We may not use all of the slots in the output array,
        // so fill with invalid values to indicate unused slots
        let faceCount = self.maxFaceCount();
        let mut out = Vec::with_capacity(faceCount as usize);
        for i in 0..faceCount {
            out.push(INVALID_FACE);
        }

        // add each vertex face, using the output array as a hash set
        for i in 0..vertexCount {
            let vert: FaceIJK = &fijkVerts[i];

            // Adjust overage, determining whether this vertex is
            // on another face
            if isPentagon {
                vert._adjustPentVertOverage(res);
            } else {
                vert._adjustOverageClassII(res, 0, 1);
            }

            // Save the face to the output array
            let face = vert.face;
            let mut pos = 0;
            // Find the first empty output position, or the first position
            // matching the current face
            while out[pos] != INVALID_FACE && out[pos] != face {
                pos += 1;
            }
            out[pos] = face;
        }

        out
    }

    /// pentagonIndexCount returns the number of pentagons (same at any resolution)
    ///
    ///@return int count of pentagon indexes
    pub(crate) fn pentagonIndexCount() -> i32 {
        NUM_PENTAGONS
    }

    ///
    ///Generates all pentagons at the specified resolution
    ///
    ///@param res The resolution to produce pentagons at.
    ///@param out Output array. Must be of size pentagonIndexCount().
    pub fn getPentagonIndexes(res: Resolution) -> Vec<H3Index> {
        let mut out = Vec::new();

        for bc in 0..NUM_BASE_CELLS {
            if BaseCell(bc)._isBaseCellPentagon() {
                let pentagon = Self::setH3Index(res, bc, 0);
                out.push(pentagon);
            }
        }

        out
    }

    /**
     * Returns whether or not a resolution is a Class III grid. Note that odd
     * resolutions are Class III and even resolutions are Class II.
     * @param res The H3 resolution.
     * @return 1 if the resolution is a Class III grid, and 0 if the resolution is
     *         a Class II grid.
     */
    pub(crate) fn isResClassIII(res: Resolution) -> bool {
        res % 2 == 1
    }

    /// Area of H3 cell in radians^2.
    /// The area is calculated by breaking the cell into spherical triangles and summing up their areas. Note that some H3 cells (hexagons and pentagons) are irregular, and have more than 6 or 5 sides.
    ///
    /// todo: optimize the computation by re-using the edges shared between triangles
    ///
    /// @param  cell  H3 cell
    /// @return cell area in radians^2
    pub(crate) fn cellAreaRads2(&self) -> f64 {
        let c: GeoCoord = self.h3ToGeo();
        let gb: GeoBoundary = self.h3ToGeoBoundary();

        let mut area = 0.0;
        for i in 0..gb.numVerts {
            let j = (i + 1) % gb.numVerts;
            area += GeoCoord::triangleArea(&gb.verts[i], &gb.verts[j], &c);
        }

        area
    }

    /// Area of H3 cell in kilometers^2
    pub(crate) fn cellAreaKm2(&self) -> f64 {
        self.cellAreaRads2() * EARTH_RADIUS_KM * EARTH_RADIUS_KM
    }

    /// Area of H3 cell in meters^2.
    pub(crate) fn cellAreaM2(&self) -> f64 {
        self.cellAreaKm2() * 1000.0 * 1000.0
    }

    /// Length of a unidirectional edge in kilometers.
    pub(crate) fn exactEdgeLengthKm(&self) -> f64 {
        self.exactEdgeLengthRads() * EARTH_RADIUS_KM
    }

    /// Length of a unidirectional edge in meters.
    pub(crate) fn exactEdgeLengthM(&self) -> f64 {
        self.exactEdgeLengthKm() * 1000.0
    }

    /// Length of a unidirectional edge in radians.  
    ///@param   edge  H3 unidirectional edge
    ///
    ///@return        length in radians
    pub(crate) fn exactEdgeLengthRads(&self) -> f64 {
        let gb: GeoBoundary = self.getH3UnidirectionalEdgeBoundary();

        let mut length = 0.0;
        for i in 0..gb.0.len() - 1 {
            length += gb.0[i].pointDistRads(&gb.0[i + 1]);
        }

        length
    }

    /// _hexRadiusKm returns the radius of a given hexagon in Km
    pub(crate) fn _hexRadiusKm(&self) -> f64 {
        // There is probably a cheaper way to determine the radius of a
        // hexagon, but this way is conceptually simple
        let h3Center: GeoCoord = self.h3ToGeo();
        let h3Boundary: GeoBoundary = self.h3ToGeoBoundary();
        h3Center.pointDistKm(h3Boundary.verts)
    }
}

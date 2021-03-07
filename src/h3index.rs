#[derive(Copy, Clone, Eq)]
struct H3Index(u64);

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
const H3_HIGH_BIT_MASK: u64 = (1 << H3_MAX_OFFSET);

/// 0 in the highest bit, 1's everywhere else.
const H3_HIGH_BIT_MASK_NEGATIVE: u64 = (!H3_HIGH_BIT_MASK);

/// 1's in the 4 mode bits, 0's everywhere else.
const H3_MODE_MASK: u64 = (15 << H3_MODE_OFFSET);

/// 0's in the 4 mode bits, 1's everywhere else.
const H3_MODE_MASK_NEGATIVE: u64 = (!H3_MODE_MASK);

/// 1's in the 7 base cell bits, 0's everywhere else.
const H3_BC_MASK: u64 = (127 << H3_BC_OFFSET);

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
const H3_DIGIT_MASK_NEGATIVE: u64 = (!H3_DIGIT_MASK);

impl H3Index {
    // Return codes for compact
    const COMPACT_SUCCESS: i32 = 0;
    const COMPACT_LOOP_EXCEEDED: i32 = -1;
    const COMPACT_DUPLICATE: i32 = -2;
    const COMPACT_ALLOC_FAILED: i32 = -3;

    /// Invalid index used to indicate an error from geoToH3 and related functions
    /// or missing data in arrays of h3 indices. Analogous to NaN in floating point.
    const H3_NULL: Self = Self(0);

    /**
     * H3 index with mode 0, res 0, base cell 0, and 7 for all index digits.
     * Typically used to initialize the creation of an H3 cell index, which
     * expects all direction digits to be 7 beyond the cell's resolution.
     */
    pub fn H3_INIT() -> Self {
        Self(35184372088831)
    }

    /// Gets the highest bit of the H3 index.
    fn H3_GET_HIGH_BIT(&self) -> u64 {
        //const H3_GET_HIGH_BIT(h3) ((int)((((h3)&H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET)))
        (self.0 & H3_HIGH_BIT_MASK) >> H3_MAX_OFFSET
    }

    /// Sets the highest bit of the h3 to v.
    fn H3_SET_HIGH_BIT(&mut self, v: u64) {
        *self.0 = (*self.0 & H3_HIGH_BIT_MASK_NEGATIVE) | (v << H3_MAX_OFFSET);
        //(h3) = (((h3)&H3_HIGH_BIT_MASK_NEGATIVE) | \
        //        (((uint64_t)(v)) << H3_MAX_OFFSET))
    }

    /// Gets the integer mode of h3.
    fn H3_GET_MODE(&self) -> u64 {
        (&self.0 & H3_MODE_MASK) >> H3_MODE_OFFSET
    }

    fn H3_GET_RESOLUTION(&self) -> u64 {
        (*self.0 & H3_RES_MASK) >> H3_RES_OFFSET
    }

    /// Sets the integer mode of h3 to v.
    fn H3_SET_MODE(&mut self, v: u64) {
        //(h3) = (((h3)&H3_MODE_MASK_NEGATIVE) | (((uint64_t)(v)) << H3_MODE_OFFSET))
        *self.0 = (*self.0 & H3_MODE_MASK_NEGATIVE) | (v << H3_MODE_OFFSET);
    }

    /// Gets the integer base cell of h3.
    fn H3_GET_BASE_CELL(&self) -> u64 {
        (*self.0 & H3_BC_MASK) >> H3_BC_OFFSET
    }

    /// Sets the integer base cell of h3 to bc.
    fn H3_SET_BASE_CELL(&mut self, bc: u64) {
        //(h3) = (((h3)&H3_BC_MASK_NEGATIVE) | (((uint64_t)(bc)) << H3_BC_OFFSET))
        *self.0 = (*self.0 & H3_BC_MASK_NEGATIVE) | (bc << H3_BC_OFFSET);
    }

    /// Gets the integer resolution of h3.
    fn H3_GET_RESOLUTION(&self) -> u64 {
        (*self.0 & H3_RES_MASK) >> H3_RES_OFFSET
    }

    /// Sets the integer resolution of h3.
    fn H3_SET_RESOLUTION(&mut self, res: u64) {
        //(h3) = (((h3)&H3_RES_MASK_NEGATIVE) | (((uint64_t)(res)) << H3_RES_OFFSET))
        *self.0 = (*self.0 & H3_RES_MASK_NEGATIVE) | (res << H3_RES_OFFSET);
    }

    /// Gets the resolution res integer digit (0-7) of h3.
    fn H3_GET_INDEX_DIGIT(&self, res: i32) -> Direction {
        //((Direction)((((h3) >> ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)) & \
        //              H3_DIGIT_MASK)))
        let d = (*self.0 >> ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)) & H3_DIGIT_MASK;
        d.into()
    }

    /// Sets a value in the reserved space. Setting to non-zero may produce invalid indexes.
    fn H3_SET_RESERVED_BITS(&mut self, v: u64) {
        *self.0 = (*self.0 & H3_RESERVED_MASK_NEGATIVE) | (v << H3_RESERVED_OFFSET);
    }

    /// Gets a value in the reserved space. Should always be zero for valid indexes.
    fn H3_GET_RESERVED_BITS(&self) -> u64 {
        (*self.0 & H3_RESERVED_MASK) >> H3_RESERVED_OFFSET
    }

    /// Sets the resolution res digit of h3 to the integer digit (0-7)
    fn H3_SET_INDEX_DIGIT(&mut self, res: u64, digit: u64) {
        *self.0 = ((*self.0 & !(H3_DIGIT_MASK << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)))
            | (digit << ((MAX_H3_RES - res) * H3_PER_DIGIT_OFFSET)));
    }

    /**
     * Returns the H3 resolution of an H3 index.
     * @param h The H3 index.
     * @return The resolution of the H3 index argument.
     */
    fn h3GetResolution(&self) {
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
    fn h3GetBaseCell(&self) -> u64 {
        self.H3_GET_BASE_CELL()
    }

    /**
     * Converts a string representation of an H3 index into an H3 index.
     * @param str The string representation of an H3 index.
     * @return The H3 index corresponding to the string argument, or H3_NULL if
     * invalid.
     */
    // TODO: this should be handled with `Parse`
    fn stringToH3(s: &str) -> Self {
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
    fn h3ToString(&self) -> String {
        todo!("sprintf(str, \"%\" PRIx64, h);");
    }

    /**
     * Returns whether or not an H3 index is a valid cell (hexagon or pentagon).
     * @param h The H3 index to validate.
     * @return 1 if the H3 index if valid, and 0 if it is not.
     */
    fn h3IsValid(&self) -> bool {
        if self.H3_GET_HIGH_BIT() != 0 {
            return false;
        }

        if (self.H3_GET_MODE() != H3_HEXAGON_MODE) {
            return false;
        }

        if (self.H3_GET_RESERVED_BITS() != 0) {
            return false;
        }

        let baseCell = self.H3_GET_BASE_CELL();
        if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return false;
        }

        let res = self.H3_GET_RESOLUTION();
        if (res < 0 || res > MAX_H3_RES) {
            // LCOV_EXCL_BR_LINE
            // Resolutions less than zero can not be represented in an index
            return false;
        }

        let mut foundFirstNonZeroDigit = false;
        for r in 1..=res {
            let digit: Direction = self.H3_GET_INDEX_DIGIT(r);

            if (!foundFirstNonZeroDigit && digit != CENTER_DIGIT) {
                foundFirstNonZeroDigit = true;
                if (_isBaseCellPentagon(baseCell) && digit == K_AXES_DIGIT) {
                    return false;
                }
            }

            if (digit < CENTER_DIGIT || digit >= NUM_DIGITS) {
                return false;
            }
        }

        for r in (res + 1..=MAX_H3_Res) {
            let digit: Direction = h.H3_GET_INDEX_DIGIT(r);
            if (digit != INVALID_DIGIT) {
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
    fn setH3Index(res: i32, baseCell: i32, initDigit: Direction) -> Self {
        let mut h = H3Index::H3_INIT();

        h.H3_SET_MODE(H3_HEXAGON_MODE);
        h.H3_SET_RESOLUTION(res);
        h.H3_SET_BASE_CELL(baseCell);

        for r in (1..=res) {
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
    fn h3ToParent(&self, parentRes: i32) -> Self {
        let childRes: u64 = h.H3_GET_RESOLUTION();

        if (parentRes < 0 || parentRes > MAX_H3_RES) {
            H3_NULL
        } else if (parentRes > childRes) {
            H3_NULL
        } else if (parentRes == childRes) {
            h
        } else {
            let parentH = H3_SET_RESOLUTION(h, parentRes);
            for i in (parentRes + 1..=childRes) {
                H3_SET_INDEX_DIGIT(parentH, i, H3_DIGIT_MASK);
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
    fn _isValidChildRes(parentRes: i32, childRes: i32) -> bool {
        if (childRes < parentRes || childRes > MAX_H3_RES) {
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
    fn maxH3ToChildrenSize(&self, childRes: i32) -> i64 {
        let parentRes = self.H3_GET_RESOLUTION();
        if (!_isValidChildRes(parentRes, childRes)) {
            return 0;
        }

        7.pow(childRes - parentRes)
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
    fn makeDirectChild(&self, cellNumber: i32) -> Self {
        let childRes = h.H3_GET_RESOLUTION() + 1;
        let mut childH = h.H3_SET_RESOLUTION(childRes);
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
    fn h3ToChildren(&self /*h*/, childRes: i32) -> Vec<H3Index> /* children  ?? */ {
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
    fn h3ToCenterChild(&self, childRes: i32) -> Self {
        let parentRes = h.H3_GET_RESOLUTION();
        if (!_isValidChildRes(parentRes, childRes)) {
            return H3_NULL;
        } else if (childRes == parentRes) {
            return h;
        }

        let mut child = H3_SET_RESOLUTION(h, childRes);
        for i in (parentRes + 1..=childRes) {
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
    fn compact(&self /*h3Set*/, compactedSet: &H3Index, numHexes: i32) -> u64 {
        if (numHexes == 0) {
            return COMPACT_SUCCESS;
        }
        let res = h3Set[0].H3_GET_RESOLUTION();
        if (res == 0) {
            // No compaction possible, just copy the set to output
            for i in 0..numHexes {
                compactedSet[i] = h3Set[i];
            }
            return COMPACT_SUCCESS;
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
    fn uncompact(
        &self, /*compactedSet*/
        numHexes: i32,
        h3Set: &mut Self,
        maxHexes: i32,
        res: i32,
    ) -> i32 {
        let mut outOffset = 0;
        for i in 0..numHexes {
            if (compactedSet[i] == 0) {
                continue;
            }

            if (outOffset >= maxHexes) {
                // We went too far, abort!
                return -1;
            }

            let currentRes = compactedSet[i].H3_GET_RESOLUTION();
            if (!_isValidChildRes(currentRes, res)) {
                // Nonsensical. Abort.
                return -2;
            }

            if (currentRes == res) {
                // Just copy and move along
                h3Set[outOffset] = compactedSet[i];
                outOffset += 1;
            } else {
                // Bigger hexagon to reduce in size
                let numHexesToGen = compactedSet[i].maxH3ToChildrenSize(res);
                if (outOffset + numHexesToGen > maxHexes) {
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
    fn maxUncompactSize(compactedSet: Vec<H3Index>, res: i32) -> i32 {
        let mut maxNumHexagons = 0;
        for h in compactedSet.iter() {
            if (h == 0) {
                continue;
            }

            let currentRes = h.H3_GET_RESOLUTION();
            if (!_isValidChildRes(currentRes, res)) {
                // Nonsensical. Abort.
                return -1;
            }
            if (currentRes == res) {
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
    fn h3IsResClassIII(&self) -> bool {
        h.H3_GET_RESOLUTION() % 2 == 1
    }

    /**
     * h3IsPentagon takes an H3Index and determines if it is actually a
     * pentagon.
     * @param h The H3Index to check.
     * @return Returns 1 if it is a pentagon, otherwise 0.
     */
    fn h3IsPentagon(&self) -> bool {
        _isBaseCellPentagon(self.H3_GET_BASE_CELL()) && !_h3LeadingNonZeroDigit(h)
    }

    /**
     * Returns the highest resolution non-zero digit in an H3Index.
     * @param h The H3Index.
     * @return The highest resolution non-zero digit in the H3Index.
     */
    fn _h3LeadingNonZeroDigit(&self) -> Direction {
        for r in 1..=self.H3_GET_RESOLUTION() {
            if (self.H3_GET_INDEX_DIGIT(r)) {
                return self.H3_GET_INDEX_DIGIT(r);
            }
        }

        // if we're here it's all 0's
        CENTER_DIGIT
    }

    /**
     * Rotate an H3Index 60 degrees counter-clockwise about a pentagonal center.
     * @param h The H3Index.
     */
    fn _h3RotatePent60ccw(&mut self) -> Self {
        // rotate in place; skips any leading 1 digits (k-axis)

        let mut foundFirstNonZeroDigit = false;
        let res = self.H3_GET_RESOLUTION();

        let mut h = self.clone();

        for r in 1..=res {
            // rotate this digit
            h.H3_SET_INDEX_DIGIT(r, _rotate60ccw(h.H3_GET_INDEX_DIGIT(r)));

            // look for the first non-zero digit so we
            // can adjust for deleted k-axes sequence
            // if necessary
            if (!foundFirstNonZeroDigit && h.H3_GET_INDEX_DIGIT(r) != 0) {
                foundFirstNonZeroDigit = true;

                // adjust for deleted k-axes sequence
                if (_h3LeadingNonZeroDigit(h) == K_AXES_DIGIT) {
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
    fn _h3RotatePent60cw(&self) -> Self {
        // rotate in place; skips any leading 1 digits (k-axis)

        let mut foundFirstNonZeroDigit = false;
        let res = h.H3_GET_RESOLUTION();
        let mut h = self.clone();
        for r in 1..=res {
            // rotate this digit
            h.H3_SET_INDEX_DIGIT(r, _rotate60cw(h.H3_GET_INDEX_DIGIT(r)));

            // look for the first non-zero digit so we
            // can adjust for deleted k-axes sequence
            // if necessary
            if (!foundFirstNonZeroDigit && h.H3_GET_INDEX_DIGIT(r) != 0) {
                foundFirstNonZeroDigit = true;

                // adjust for deleted k-axes sequence
                if (_h3LeadingNonZeroDigit(h) == K_AXES_DIGIT) {
                    h = _h3Rotate60cw(h);
                }
            }
        }

        h
    }

    /**
     * Rotate an H3Index 60 degrees counter-clockwise.
     * @param h The H3Index.
     */
    fn _h3Rotate60ccw(&self) -> Self {
        let mut h = self.clone();
        let res = h.H3_GET_RESOLUTION();

        for r in 1..=res {
            let oldDigit: Direction = h.H3_GET_INDEX_DIGIT(r);
            h.H3_SET_INDEX_DIGIT(r, _rotate60ccw(oldDigit));
        }

        h
    }

    /**
     * Rotate an H3Index 60 degrees clockwise.
     * @param h The H3Index.
     */
    fn _h3Rotate60cw(&self) -> Self {
        let res = h.H3_GET_RESOLUTION();
        let mut h = self.clone();
        for r in 1..=res {
            h.H3_SET_INDEX_DIGIT(r, _rotate60cw(h.H3_GET_INDEX_DIGIT(r)));
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
    fn _h3ToFaceIjkWithInitializedFijk(&self, fijk: FaceIJK) -> bool {
        let mut ijk: CoordIJK = fijk.coord.clone();
        let res = self.H3_GET_RESOLUTION();

        // center base cell hierarchy is entirely on this face
        let mut possibleOverage = true;
        if (!_isBaseCellPentagon(self.H3_GET_BASE_CELL())
            && (res == 0 || (fijk.coord.i == 0 && fijk.coord.j == 0 && fijk.coord.k == 0)))
        {
            possibleOverage = false;
        }

        for r in 1..=res {
            if (isResClassIII(r)) {
                // Class III == rotate ccw
                _downAp7(ijk);
            } else {
                // Class II == rotate cw
                _downAp7r(ijk);
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
    fn _h3ToFaceIjk(&self) -> FaceIJK {
        let baseCell = h.H3_GET_BASE_CELL();
        if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            // TODO: Indicate an error to the caller
            // To prevent reading uninitialized memory, we zero the output.
            return FaceIJK::default();
        }
        // adjust for the pentagonal missing sequence; all of sub-sequence 5 needs
        // to be adjusted (and some of sub-sequence 4 below)
        if (_isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) == 5) {
            h._h3Rotate60cw();
        }

        // start with the "home" face and ijk+ coordinates for the base cell of c
        let fijk = baseCellData[baseCell].homeFijk;
        if (!_h3ToFaceIjkWithInitializedFijk(h, fijk)) {
            // no overage is possible; h lies on this face
            return fijk;
        }

        // if we're here we have the potential for an "overage"; i.e., it is
        // possible that c lies on an adjacent face

        let origIJK: CoordIJK = fijk.coord;

        // if we're in Class III, drop into the next finer Class II grid
        let res = h.H3_GET_RESOLUTION();
        if (isResClassIII(res)) {
            // Class III
            _downAp7r(&fijk.coord);
            res += 1;
        }

        // adjust for overage if needed
        // a pentagon base cell with a leading 4 digit requires special handling
        let pentLeading4 = (_isBaseCellPentagon(baseCell) && _h3LeadingNonZeroDigit(h) == 4);
        if (_adjustOverageClassII(fijk, res, pentLeading4, 0) != NO_OVERAGE) {
            // if the base cell is a pentagon we have the potential for secondary
            // overages
            if (_isBaseCellPentagon(baseCell)) {
                while (_adjustOverageClassII(fijk, res, 0, 0) != NO_OVERAGE) {
                    continue;
                }
            }

            if (res != h.H3_GET_RESOLUTION()) {
                _upAp7r(&fijk.coord);
            }
        } else if (res != h.H3_GET_RESOLUTION()) {
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
    fn h3ToGeo(&self /*h3*/) -> GeoCoord {
        let fijk = self._h3ToFaceIjk();
        fijk._faceIjkToGeo(h3.H3_GET_RESOLUTION())
    }

    /**
     * Determines the cell boundary in spherical coordinates for an H3 index.
     *
     * @param h3 The H3 index.
     * @param gb The boundary of the H3 cell in spherical coordinates.
     */
    fn h3ToGeoBoundary(&self) -> GeoBoundary {
        let fijk: FaceIJK = self._h3ToFaceIjk();

        if (self.h3IsPentagon()) {
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
    fn maxFaceCount(&self) -> i32 {
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
    fn h3GetFaces(&self) -> Vec<i32> {
        let res = self.H3_GET_RESOLUTION();
        let isPentagon = self.h3IsPentagon();

        // We can't use the vertex-based approach here for class II pentagons,
        // because all their vertices are on the icosahedron edges. Their
        // direct child pentagons cross the same faces, so use those instead.
        if (isPentagon && !isResClassIII(res)) {
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
        if (isPentagon) {
            vertexCount = NUM_PENT_VERTS;
            _faceIjkPentToVerts(&fijk, &res, fijkVerts);
        } else {
            vertexCount = NUM_HEX_VERTS;
            _faceIjkToVerts(&fijk, &res, fijkVerts);
        }

        // We may not use all of the slots in the output array,
        // so fill with invalid values to indicate unused slots
        let faceCount = self.maxFaceCount();
        let mut out = Vec::with_capacity(faceCount);
        for i in 0..faceCount {
            out.push(INVALID_FACE);
        }

        // add each vertex face, using the output array as a hash set
        for i in 0..vertexCount {
            let vert: FaceIJK = &fijkVerts[i];

            // Adjust overage, determining whether this vertex is
            // on another face
            if (isPentagon) {
                _adjustPentVertOverage(vert, res);
            } else {
                _adjustOverageClassII(vert, res, 0, 1);
            }

            // Save the face to the output array
            let face = vert.face;
            let mut pos = 0;
            // Find the first empty output position, or the first position
            // matching the current face
            while (out[pos] != INVALID_FACE && out[pos] != face) {
                pos += 1;
            }
            out[pos] = face;
        }

        out
    }

    /// pentagonIndexCount returns the number of pentagons (same at any resolution)
    ///
    ///@return int count of pentagon indexes
    fn pentagonIndexCount() -> i32 {
        NUM_PENTAGONS
    }

    ///
    ///Generates all pentagons at the specified resolution
    ///
    ///@param res The resolution to produce pentagons at.
    ///@param out Output array. Must be of size pentagonIndexCount().
    fn getPentagonIndexes(res: i32) -> Vec<H3Index> {
        let mut out = Vec::new();

        for bc in 0..NUM_BASE_CELLS {
            if _isBaseCellPentagon(bc) {
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
    fn isResClassIII(res: i32) -> bool {
        res % 2 == 1
    }

    /// Area of H3 cell in radians^2.
    /// The area is calculated by breaking the cell into spherical triangles and summing up their areas. Note that some H3 cells (hexagons and pentagons) are irregular, and have more than 6 or 5 sides.
    ///
    /// todo: optimize the computation by re-using the edges shared between triangles
    ///
    /// @param  cell  H3 cell
    /// @return cell area in radians^2
    fn cellAreaRads2(&self) -> f64 {
        let c: GeoCoord = self.h3ToGeo();
        let gb: GeoBoundary = self.h3ToGeoBoundary();

        let mut area = 0.0;
        for i in 0..gb.numVerts {
            let j = (i + 1) % gb.numVerts;
            area += triangleArea(&gb.verts[i], &gb.verts[j], &c);
        }

        area
    }

    /// Area of H3 cell in kilometers^2.
    fn cellAreaKm2(&self) -> f64 {
        self.cellAreaRads2() * EARTH_RADIUS_KM * EARTH_RADIUS_KM
    }

    /// Area of H3 cell in meters^2.
    fn cellAreaM2(&self) -> f64 {
        self.cellAreaKm2() * 1000.0 * 1000.0
    }

    /// Length of a unidirectional edge in radians.
    fn exactEdgeLengthRads(&self) -> f64 {
        let gb: GeoBoundary = self.getH3UnidirectionalEdgeBoundary();
        //let gb: GeoBoundary = getH3UnidirectionalEdgeBoundary(edge, &gb);

        let mut length = 0.0;
        for i in 0..(gb.numVerts - 1) {
            length += db.verts[i].pointDistRads(&gb.verts[i + 1]);
        }

        length
    }

    /// Length of a unidirectional edge in kilometers.
    fn exactEdgeLengthKm(&self) -> f64 {
        self.exactEdgeLengthRads() * EARTH_RADIUS_KM
    }

    /// Length of a unidirectional edge in meters.
    fn exactEdgeLengthM(&self) -> f64 {
        self.exactEdgeLengthKm() * 1000.0
    }

    /// Area of H3 cell in kilometers^2
    fn cellAreaKm2(&self) -> f64 {
        self.cellAreaRads2() * EARTH_RADIUS_KM * EARTH_RADIUS_KM
    }

    /// Area of H3 cell in meters^2
    fn cellAreaM2(&self) -> f64 {
        self.cellAreaKm2() * 1000 * 1000
    }

    /// Length of a unidirectional edge in radians.  
    ///@param   edge  H3 unidirectional edge
    ///
    ///@return        length in radians
    fn exactEdgeLengthRads(&self) -> f64 {
        let gb: GeoBoundary = self.getH3UnidirectionalEdgeBoundary();

        let mut length = 0.0;
        for i in 0..(gb.numVerts - 1) {
            length += gb.verts[i].pointDistRads(&gb.verts[i + 1]);
        }

        length
    }

    /// Length of a unidirectional edge in kilometers.
    fn exactEdgeLengthKm(&self) -> f64 {
        edge.exactEdgeLengthRads() * EARTH_RADIUS_KM
    }

    /// Length of a unidirectional edge in meters.
    fn exactEdgeLengthM(&self) -> f64 {
        self.exactEdgeLengthKm() * 1000
    }

    /// Area of H3 cell in radians^2.
    ///
    ///The area is calculated by breaking the cell into spherical triangles and
    ///summing up their areas. Note that some H3 cells (hexagons and pentagons)
    ///are irregular, and have more than 6 or 5 sides.
    ///
    ///todo: optimize the computation by re-using the edges shared between triangles
    ///
    ///@param   cell  H3 cell
    ///
    ///@return        cell area in radians^2
    fn cellAreaRads2(&self) -> f64 {
        let c: GeoCoord = self.h3ToGeo();
        let gb: GeoBoundary = self.h3ToGeoBoundary();

        let mut area = 0.0;
        for i in (0..gb.numVerts) {
            let j = (i + 1) % gb.numVerts;
            area += triangleArea(&gb.verts[i], &gb.verts[j], &c);
        }

        area
    }

    /// _hexRadiusKm returns the radius of a given hexagon in Km
    fn _hexRadiusKm(&self) -> f64 {
        // There is probably a cheaper way to determine the radius of a
        // hexagon, but this way is conceptually simple
        let h3Center: GeoCoord = self.h3ToGeo();
        let h3Boundary: GeoBoundary = self.h3ToGeoBoundary();
        h3Center.pointDistKm(h3Boundary.verts)
    }

    /* h3UniEdge */
    /**
     * Returns whether or not the provided H3Indexes are neighbors.
     * @param origin The origin H3 index.
     * @param destination The destination H3 index.
     * @return 1 if the indexes are neighbors, 0 otherwise;
     */
    fn h3IndexesAreNeighbors(&self, destination: &Self) -> bool {
        // Make sure they're hexagon indexes
        if (self.H3_GET_MODE() != H3_HEXAGON_MODE || destination.H3_GET_MODE() != H3_HEXAGON_MODE) {
            return false;
        }

        // Hexagons cannot be neighbors with themselves
        if (self == destination) {
            return false;
        }

        // Only hexagons in the same resolution can be neighbors
        if (self.H3_GET_RESOLUTION() != destination.H3_GET_RESOLUTION()) {
            return false;
        }

        // H3 Indexes that share the same parent are very likely to be neighbors
        // Child 0 is neighbor with all of its parent's 'offspring', the other
        // children are neighbors with 3 of the 7 children. So a simple comparison
        // of origin and destination parents and then a lookup table of the children
        // is a super-cheap way to possibly determine they are neighbors.
        let parentRes = self.H3_GET_RESOLUTION() - 1;
        if (parentRes > 0 && (self.h3ToParent(parentRes) == destination.h3ToParent(parentRes))) {
            let originResDigit: Direction = self.H3_GET_INDEX_DIGIT(parentRes + 1);
            let destinationResDigit: Direction = destination.H3_GET_INDEX_DIGIT(parentRes + 1);
            if (originResDigit == CENTER_DIGIT || destinationResDigit == CENTER_DIGIT) {
                return true;
            }

            // These sets are the relevant neighbors in the clockwise
            // and counter-clockwise
            const neighborSetClockwise: [Direction; 7] = [
                CENTER_DIGIT,
                JK_AXES_DIGIT,
                IJ_AXES_DIGIT,
                J_AXES_DIGIT,
                IK_AXES_DIGIT,
                K_AXES_DIGIT,
                I_AXES_DIGIT,
            ];
            const neighborSetCounterclockwise: [Direction; 7] = [
                CENTER_DIGIT,
                IK_AXES_DIGIT,
                JK_AXES_DIGIT,
                K_AXES_DIGIT,
                IJ_AXES_DIGIT,
                I_AXES_DIGIT,
                J_AXES_DIGIT,
            ];
            if (neighborSetClockwise[originResDigit] == destinationResDigit
                || neighborSetCounterclockwise[originResDigit] == destinationResDigit)
            {
                return true;
            }
        }

        // Otherwise, we have to determine the neighbor relationship the "hard" way.
        let neighborRing: [H3Index; 7] = [H3Index::default(); 7];
        kRing(self, 1, neighborRing);
        for i in 0..7 {
            if (neighborRing[i] == destination) {
                return true;
            }
        }

        // Made it here, they definitely aren't neighbors
        false
    }

    /**
     * Returns a unidirectional edge H3 index based on the provided origin and
     * destination
     * @param origin The origin H3 hexagon index
     * @param destination The destination H3 hexagon index
     * @return The unidirectional edge H3Index, or H3_NULL on failure.
     */
    fn getH3UnidirectionalEdge(&self, destination: &Self) -> Self {
        // Determine the IJK direction from the origin to the destination
        let direction: Direction = self.directionForNeighbor(destination);

        // The direction will be invalid if the cells are not neighbors
        if (direction == INVALID_DIGIT) {
            return Self::H3_NULL;
        }

        // Create the edge index for the neighbor direction
        let mut output = self.clone();
        output.H3_SET_MODE(H3_UNIEDGE_MODE);
        output.H3_SET_RESERVED_BITS(direction);

        output
    }

    /**
     * Returns the origin hexagon from the unidirectional edge H3Index
     * @param edge The edge H3 index
     * @return The origin H3 hexagon index, or H3_NULL on failure
     */
    fn getOriginH3IndexFromUnidirectionalEdge(&self) -> Self {
        if (self.H3_GET_MODE() != H3_UNIEDGE_MODE) {
            return Self::H3_NULL;
        }
        let mut origin = self.clone();
        origin.H3_SET_MODE(H3_HEXAGON_MODE);
        origin.H3_SET_RESERVED_BITS(0);
        origin
    }

    /**
     * Returns the destination hexagon from the unidirectional edge H3Index
     * @param edge The edge H3 index
     * @return The destination H3 hexagon index, or H3_NULL on failure
     */
    fn getDestinationH3IndexFromUnidirectionalEdge(&self) -> Self {
        if (H3_GET_MODE(edge) != H3_UNIEDGE_MODE) {
            return Self::H3_NULL;
        }

        let direction: Direction = self.H3_GET_RESERVED_BITS();
        let mut rotations = 0;
        let destination = h3NeighborRotations(
            self.getOriginH3IndexFromUnidirectionalEdge(),
            direction,
            &mut rotations,
        );

        destination
    }

    /**
     * Determines if the provided H3Index is a valid unidirectional edge index
     * @param edge The unidirectional edge H3Index
     * @return 1 if it is a unidirectional edge H3Index, otherwise 0.
     */
    fn h3UnidirectionalEdgeIsValid(&self) -> bool {
        if (self.H3_GET_MODE() != H3_UNIEDGE_MODE) {
            return false;
        }

        let neighborDirection: Direction = self.H3_GET_RESERVED_BITS();
        if (neighborDirection <= CENTER_DIGIT || neighborDirection >= NUM_DIGITS) {
            return false;
        }

        let origin = self.getOriginH3IndexFromUnidirectionalEdge();
        if (h3IsPentagon(origin) && neighborDirection == K_AXES_DIGIT) {
            return false;
        }

        origin.h3IsValid()
    }

    /**
     * Returns the origin, destination pair of hexagon IDs for the given edge ID
     * @param edge The unidirectional edge H3Index
     * @param originDestination Pointer to memory to store origin and destination
     * IDs
     */
    fn getH3IndexesFromUnidirectionalEdge(&self) -> (Self, Self) {
        (
            self.getOriginH3IndexFromUnidirectionalEdge(),
            self.getDestinationH3IndexFromUnidirectionalEdge(),
        )
    }

    /**
     * Provides all of the unidirectional edges from the current H3Index.
     * @param origin The origin hexagon H3Index to find edges for.
     * @param edges The memory to store all of the edges inside.
     */
    fn getH3UnidirectionalEdgesFromHexagon(&self) {
        // Determine if the origin is a pentagon and special treatment needed.
        let isPentagon = self.h3IsPentagon();

        let mut edges = [H3Index; 6] = [Self::H3_Null; 6];

        // This is actually quite simple. Just modify the bits of the origin
        // slightly for each direction, except the 'k' direction in pentagons,
        // which is zeroed.
        for i in 0..6 {
            if (isPentagon && i == 0) {
                edges[i] = H3_NULL;
            } else {
                edges[i] = origin;
                edges[i].H3_SET_MODE(H3_UNIEDGE_MODE);
                edges[i].H3_SET_RESERVED_BITS(i + 1);
            }
        }

        edges
    }

    /**
     * Provides the coordinates defining the unidirectional edge.
     * @param edge The unidirectional edge H3Index
     * @param gb The geoboundary object to store the edge coordinates.
     */
    fn getH3UnidirectionalEdgeBoundary(&self) -> GeoBoundary {
        // Get the origin and neighbor direction from the edge
        let direction: Direction = self.H3_GET_RESERVED_BITS();
        let origin = self.getOriginH3IndexFromUnidirectionalEdge();

        // Get the start vertex for the edge
        let startVertex = origin.vertexNumForDirection(direction);
        if (startVertex == INVALID_VERTEX_NUM) {
            // This is not actually an edge (i.e. no valid direction), so return no vertices.
            return GeoBoundary::default();
        }

        // Get the geo boundary for the appropriate vertexes of the origin. Note
        // that while there are always 2 topological vertexes per edge, the
        // resulting edge boundary may have an additional distortion vertex if it
        // crosses an edge of the icosahedron.
        let fijk: FaceIJK = origin._h3ToFaceIjk();
        let res = origin.H3_GET_RESOLUTION();
        let isPentagon = origin.h3IsPentagon();

        if (isPentagon) {
            fijk._faceIjkPentToGeoBoundary(res, startVertex, 2)
        } else {
            fijk._faceIjkToGeoBoundary(res, startVertex, 2)
        }
    }
    /* /h3UniEdge */

    /* localij */
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

    /* localij */

    /* vertex */

    /**
     * Whether the input is a valid H3 vertex
     * @param  vertex H3 index possibly describing a vertex
     * @return        Whether the input is valid
     */
    fn isValidVertex(&self) -> bool {
        if (self.H3_GET_MODE() != H3_VERTEX_MODE) {
            return false;
        }

        let vertexNum = self.H3_GET_RESERVED_BITS();
        let mut owner: H3Index = vertex;
        owner.H3_SET_MODE(H3_HEXAGON_MODE);
        owner.H3_SET_RESERVED_BITS(0);

        if (!owner.h3IsValid()) {
            return false;
        }

        // The easiest way to ensure that the owner + vertex number is valid,
        // and that the vertex is canonical, is to recreate and compare.
        let canonical: H3Index = owner.cellToVertex(vertexNum);

        vertex == canonical
    }
    /* /vertex */
}

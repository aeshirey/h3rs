mod geocoord;
pub use geocoord::*;

use crate::{basecell::BaseCell, faceijk::FaceIJK, Direction, GeoCoord, Resolution};

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

        for bc in -1..BaseCell::NUM_BASE_CELLS as i32 {
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
}

impl From<H3Index> for u64 {
    fn from(h3: H3Index) -> Self {
        h3.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// H3 index modes
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
}

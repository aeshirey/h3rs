mod geocoord;
pub use geocoord::*;

use crate::{basecell::BaseCell, Direction, GeoCoord, Resolution};

mod h3UniEdge;

/// The H3Index fits within a 64-bit unsigned integer
pub struct H3Index(u64);

impl H3Index {
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

    /**
     * H3 index with mode 0, res 0, base cell 0, and 7 for all index digits.
     * Typically used to initialize the creation of an H3 cell index, which
     * expects all direction digits to be 7 beyond the cell's resolution.
     */
    const H3_INIT: H3Index = H3Index(35184372088831);

    /// Gets the highest bit of the H3 index.
    fn get_high_bit(&self) -> u64 {
        self.0 >> Self::H3_MAX_OFFSET
    }

    /// Gets the integer mode of h3.
    pub(crate) fn get_mode(&self) -> u64 {
        (self.0 & Self::H3_MODE_MASK) >> Self::H3_MODE_OFFSET
    }

    /// Sets the integer mode of h3 to v.
    pub(crate) fn set_mode(&mut self, v: u64) {
        //(h3) = (((h3)&H3_MODE_MASK_NEGATIVE) | (((uint64_t)(v)) << H3_MODE_OFFSET))
        todo!()
    }

    /// Gets the integer base cell of h3.
    pub(crate) fn get_base_cell(&self) -> BaseCell {
        let bc = (self.0 & Self::H3_BC_MASK) >> Self::H3_BC_OFFSET;
        todo!()
    }

    /// Sets the integer base cell of h3 to bc.
    pub(crate) fn set_base_cell(&self, bc: BaseCell) {
        todo!();
        //(h3) = (((h3)&H3_BC_MASK_NEGATIVE) | (((uint64_t)(bc)) << H3_BC_OFFSET))
    }

    /// Gets the integer resolution of h3.
    pub(crate) fn get_resolution(&self) -> Resolution {
        //(self.0 & Self::H3_RES_MASK) >> Self::H3_RES_OFFSET
        todo!()
    }

    /// Sets the integer resolution of h3.
    pub(crate) fn set_resolution(&mut self, res: Resolution) {
        //(self.0 & H3_RES_MASK_NEGATIVE) | (((uint64_t)(res)) << Self::H3_RES_OFFSET)
        todo!()
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
        todo!()
        //for (int r = 1; r <= H3_GET_RESOLUTION(h); r++)
        //    if (H3_GET_INDEX_DIGIT(h, r)) return H3_GET_INDEX_DIGIT(h, r);

        // if we're here it's all 0's
        //Direction::CENTER_DIGIT
    }
}

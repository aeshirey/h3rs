use crate::{coordij::CoordIJ, coordijk::CoordIJK};

use super::H3Index;

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

    // @param out ijk+ coordinates of the index will be placed here on success
    // @return 0 on success, or another value on failure.
    pub(crate) fn h3ToLocalIjk(&self /*origin*/, h3: &Self) -> Result<CoordIJK, i32> {
        todo!()
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
    pub fn experimentalLocalIjToH3(&self, ij: &CoordIJ) -> Result<Self, ()> {
        // This function is currently experimental. Once ready to be part of the
        // non-experimental API, this function (with the experimental prefix) will
        // be marked as deprecated and to be removed in the next major version. It
        // will be replaced with a non-prefixed function name.
        let ijk: CoordIJK = ij.into();

        self.localIjkToH3(&ijk)
    }

    // @param out The index will be placed here on success
    // @return 0 on success, or another value on failure.
    fn localIjkToH3(&self, ij: &CoordIJK) -> Result<Self, ()> {
        todo!()
    }
}

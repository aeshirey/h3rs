use crate::coordijk::CoordIJK;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
/// IJ hexagon coordinates
///
/// Each axis is spaced 120 degrees apart.
pub struct CoordIJ {
    /// i component
    pub i: i32,
    /// j component
    pub j: i32,
}

impl CoordIJ {
    pub(crate) const fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }

    /**
     * Transforms coordinates from the IJ coordinate system to the IJK+ coordinate
     * system.
     *
     * @param ij The input IJ coordinates
     * @param ijk The output IJK+ coordinates
     */
    pub(crate) fn ijToIjk(&self) -> CoordIJK {
        let mut ijk = CoordIJK {
            i: self.i,
            j: self.j,
            k: 0,
        };
        ijk.normalize();

        ijk
    }
}

impl From<(i32, i32)> for CoordIJ {
    fn from(ij: (i32, i32)) -> Self {
        let (i, j) = ij;
        CoordIJ { i, j }
    }
}

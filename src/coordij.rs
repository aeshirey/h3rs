/// IJ hexagon coordinates
///
/// Each axis is spaced 120 degrees apart.
pub struct CoordIJ {
    /// i component
    i: i32,

    /// j component
    j: i32,
}

impl CoordIJ {
    /// Transforms coordinates from the IJ coordinate system to the IJK+ coordinate system.
    pub fn ijToIjk(&self) -> CoordIJK {
        let mut ijk = CoordIJK {
            i: self.i,
            j: self.j,
            k: 0,
        };

        ijk._ijkNormalize();

        ijk
    }
}

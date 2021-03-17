#[derive(Copy, Clone, Debug, PartialEq)]
/// IJ hexagon coordinates
///
/// Each axis is spaced 120 degrees apart.
pub struct CoordIJ {
    /// i component
    pub i: i32,
    /// j component
    pub j: i32,
}

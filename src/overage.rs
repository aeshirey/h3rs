/// Digit representing overage type
pub enum Overage {
    /// No overage (on original face)
    NO_OVERAGE,
    /// On face edge (only occurs on substrate grids)
    FACE_EDGE,
    /// Overage on new face interior
    NEW_FACE,
}

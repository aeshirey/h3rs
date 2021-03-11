use crate::constants::NUM_PENT_VERTS;

/// The faces in each axial direction of a given pentagon base cell
pub struct PentagonDirectionFaces {
    /// base cell number
    pub baseCell: i32,
    /// face numbers for each axial direction, in order, starting with J
    pub faces: [i32; NUM_PENT_VERTS],
}

impl PentagonDirectionFaces {
    pub(crate) const fn new(baseCell: i32, faces: [i32; NUM_PENT_VERTS]) -> Self {
        Self { baseCell, faces }
    }
}

/// Invalid vertex number
pub(crate) const INVALID_VERTEX_NUM: i32 = -1;

/// Max number of faces a base cell's descendants may appear on */
pub(crate) const MAX_BASE_CELL_FACES: i32 = 5;

//int vertexNumForDirection(const H3Index origin, const Direction direction);
//Direction directionForVertexNum(const H3Index origin, const int vertexNum);

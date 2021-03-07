/// The faces in each axial direction of a given pentagon base cell
pub struct PentagonDirectionFaces {
    /// base cell number
    baseCell: i32,
    /// face numbers for each axial direction, in order, starting with J
    faces: Vec<i32>,
    //int faces[NUM_PENT_VERTS];
}

/// Invalid vertex number 
const INVALID_VERTEX_NUM : i32 = -1;

/// Max number of faces a base cell's descendants may appear on */
const MAX_BASE_CELL_FACES : i32 =5;

//int vertexNumForDirection(const H3Index origin, const Direction direction);
//Direction directionForVertexNum(const H3Index origin, const int vertexNum);



/// Face number and ijk coordinates on that face-centered coordinate system
pub struct FaceIJK {
    /// face number
    face: i32,       
    /// ijk coordinates on that face
    coord : CoordIJK ,
}

///Information to transform into an adjacent face IJK system
pub struct FaceOrientIJK{
    /// face number
    face : i32,          
    /// res 0 translation relative to primary face
    translate: CoordIJK ,
    /// number of 60 degree ccw rotations relative to primary face
    ccwRot60 :i32, 
}

// indexes for faceNeighbors table
/// IJ quadrant faceNeighbors table direction 
const IJ : i32 = 1;
/// KI quadrant faceNeighbors table direction
const KI : i32 = 2;
/// JK quadrant faceNeighbors table direction 
const JK : i32 = 3;

/// Invalid face index */
const INVALID_FACE : i32 -1;

/// Digit representing overage type 
pub enum Overage {
    /// No overage (on original face) 
    NO_OVERAGE,
    /// On face edge (only occurs on substrate grids) 
    FACE_EDGE,
    /// Overage on new face interior
    NEW_FACE,
} 


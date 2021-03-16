use crate::{constants::NUM_ICOSA_FACES, coordijk::CoordIJK};

/// Face number and ijk coordinates on that face-centered coordinate system
pub(crate) struct FaceIJK {
    /// face number
    face: i32,
    /// ijk coordinates on that face
    coord: CoordIJK,
}

impl FaceIJK {
    pub(crate) const fn new(face: i32, coord: (i32, i32, i32)) -> Self {
        let coord = CoordIJK::new(coord.0, coord.1, coord.2);

        Self { face, coord }
    }
}

/// Information to transform into an adjacent face IJK system
pub(crate) struct FaceOrientIJK {
    /// face number
    face: i32,

    /// res 0 translation relative to primary face
    translate: CoordIJK,

    /// number of 60 degree ccw rotations relative to primary face
    ccwRot60: i32,
}

impl FaceOrientIJK {
    const fn new(face: i32, translate: (i32, i32, i32), ccwRot60: i32) -> Self {
        let translate = CoordIJK::new(translate.0, translate.1, translate.2);

        Self {
            face,
            translate,
            ccwRot60,
        }
    }
}

/// Definition of which faces neighbor each other.
const faceNeighbors: [[FaceOrientIJK; 4]; NUM_ICOSA_FACES] = [
    [
        // face 0
        FaceOrientIJK::new(0, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(4, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(1, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(5, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 1
        FaceOrientIJK::new(1, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(0, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(2, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(6, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 2
        FaceOrientIJK::new(2, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(1, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(3, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(7, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 3
        FaceOrientIJK::new(3, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(2, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(4, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(8, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 4
        FaceOrientIJK::new(4, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(3, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(0, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(9, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 5
        FaceOrientIJK::new(5, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(10, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(14, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(0, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 6
        FaceOrientIJK::new(6, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(11, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(10, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(1, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 7
        FaceOrientIJK::new(7, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(12, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(11, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(2, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 8
        FaceOrientIJK::new(8, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(13, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(12, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(3, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 9
        FaceOrientIJK::new(9, (0, 0, 0), 0),  // central face
        FaceOrientIJK::new(14, (2, 2, 0), 3), // ij quadrant
        FaceOrientIJK::new(13, (2, 0, 2), 3), // ki quadrant
        FaceOrientIJK::new(4, (0, 2, 2), 3),  // jk quadrant
    ],
    [
        // face 10
        FaceOrientIJK::new(10, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(5, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(6, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(15, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 11
        FaceOrientIJK::new(11, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(6, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(7, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(16, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 12
        FaceOrientIJK::new(12, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(7, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(8, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(17, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 13
        FaceOrientIJK::new(13, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(8, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(9, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(18, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 14
        FaceOrientIJK::new(14, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(9, (2, 2, 0), 3),  // ij quadrant
        FaceOrientIJK::new(5, (2, 0, 2), 3),  // ki quadrant
        FaceOrientIJK::new(19, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 15
        FaceOrientIJK::new(15, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(16, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(19, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(10, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 16
        FaceOrientIJK::new(16, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(17, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(15, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(11, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 17
        FaceOrientIJK::new(17, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(18, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(16, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(12, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 18
        FaceOrientIJK::new(18, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(19, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(17, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(13, (0, 2, 2), 3), // jk quadrant
    ],
    [
        // face 19
        FaceOrientIJK::new(19, (0, 0, 0), 0), // central face
        FaceOrientIJK::new(15, (2, 0, 2), 1), // ij quadrant
        FaceOrientIJK::new(18, (2, 2, 0), 5), // ki quadrant
        FaceOrientIJK::new(14, (0, 2, 2), 3), // jk quadrant
    ],
];

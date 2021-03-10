use crate::{
    constants::{INVALID_BASE_CELL, NUM_BASE_CELLS, NUM_ICOSA_FACES},
    direction::Direction,
    faceijk::FaceIJK,
    h3index::H3Index,
};

/// Base cell related lookup tables and access functions.

/// information on a single base cell
pub struct BaseCellData {
    /// "home" face and normalized ijk coordinates on that face
    homeFijk: FaceIJK,
    /// is this base cell a pentagon?
    isPentagon: bool,
    /// if a pentagon, what are its two clockwise offset faces?
    cwOffsetPent: [i32; 2],
}

/// Maximum input for any component to face-to-base-cell lookup functions
pub(crate) const MAX_FACE_COORD: i32 = 2;

/// Invalid number of rotations
pub(crate) const INVALID_ROTATIONS: i32 = -1;

impl BaseCellData {
    pub const fn new(face: i32, coord: [i32; 3], isPentagon: bool, cwOffsetPent: [i32; 2]) -> Self {
        Self {
            homeFijk: FaceIJK::new(face, coord),
            isPentagon,
            cwOffsetPent,
        }
    }
}

/// base cell at a given ijk and required rotations into its system
struct BaseCellRotation {
    /// base cell number
    baseCell: i32,
    /// number of ccw 60 degree rotations relative to current
    ccwRot60: i32,
}

impl BaseCellRotation {
    pub const fn new(baseCell: i32, ccwRot60: i32) -> Self {
        Self { baseCell, ccwRot60 }
    }
}

/// Neighboring base cell ID in each IJK direction.
///
///For each base cell, for each direction, the neighboring base
///cell ID is given. 127 indicates there is no neighbor in that direction.
pub(crate) const baseCellNeighbors: [[i32; 7]; 122] = [
    [0, 1, 5, 2, 4, 3, 8],                             // base cell 0
    [1, 7, 6, 9, 0, 3, 2],                             // base cell 1
    [2, 6, 10, 11, 0, 1, 5],                           // base cell 2
    [3, 13, 1, 7, 4, 12, 0],                           // base cell 3
    [4, INVALID_BASE_CELL, 15, 8, 3, 0, 12],           // base cell 4 (pentagon)
    [5, 2, 18, 10, 8, 0, 16],                          // base cell 5
    [6, 14, 11, 17, 1, 9, 2],                          // base cell 6
    [7, 21, 9, 19, 3, 13, 1],                          // base cell 7
    [8, 5, 22, 16, 4, 0, 15],                          // base cell 8
    [9, 19, 14, 20, 1, 7, 6],                          // base cell 9
    [10, 11, 24, 23, 5, 2, 18],                        // base cell 10
    [11, 17, 23, 25, 2, 6, 10],                        // base cell 11
    [12, 28, 13, 26, 4, 15, 3],                        // base cell 12
    [13, 26, 21, 29, 3, 12, 7],                        // base cell 13
    [14, INVALID_BASE_CELL, 17, 27, 9, 20, 6],         // base cell 14 (pentagon)
    [15, 22, 28, 31, 4, 8, 12],                        // base cell 15
    [16, 18, 33, 30, 8, 5, 22],                        // base cell 16
    [17, 11, 14, 6, 35, 25, 27],                       // base cell 17
    [18, 24, 30, 32, 5, 10, 16],                       // base cell 18
    [19, 34, 20, 36, 7, 21, 9],                        // base cell 19
    [20, 14, 19, 9, 40, 27, 36],                       // base cell 20
    [21, 38, 19, 34, 13, 29, 7],                       // base cell 21
    [22, 16, 41, 33, 15, 8, 31],                       // base cell 22
    [23, 24, 11, 10, 39, 37, 25],                      // base cell 23
    [24, INVALID_BASE_CELL, 32, 37, 10, 23, 18],       // base cell 24 (pentagon)
    [25, 23, 17, 11, 45, 39, 35],                      // base cell 25
    [26, 42, 29, 43, 12, 28, 13],                      // base cell 26
    [27, 40, 35, 46, 14, 20, 17],                      // base cell 27
    [28, 31, 42, 44, 12, 15, 26],                      // base cell 28
    [29, 43, 38, 47, 13, 26, 21],                      // base cell 29
    [30, 32, 48, 50, 16, 18, 33],                      // base cell 30
    [31, 41, 44, 53, 15, 22, 28],                      // base cell 31
    [32, 30, 24, 18, 52, 50, 37],                      // base cell 32
    [33, 30, 49, 48, 22, 16, 41],                      // base cell 33
    [34, 19, 38, 21, 54, 36, 51],                      // base cell 34
    [35, 46, 45, 56, 17, 27, 25],                      // base cell 35
    [36, 20, 34, 19, 55, 40, 54],                      // base cell 36
    [37, 39, 52, 57, 24, 23, 32],                      // base cell 37
    [38, INVALID_BASE_CELL, 34, 51, 29, 47, 21],       // base cell 38 (pentagon)
    [39, 37, 25, 23, 59, 57, 45],                      // base cell 39
    [40, 27, 36, 20, 60, 46, 55],                      // base cell 40
    [41, 49, 53, 61, 22, 33, 31],                      // base cell 41
    [42, 58, 43, 62, 28, 44, 26],                      // base cell 42
    [43, 62, 47, 64, 26, 42, 29],                      // base cell 43
    [44, 53, 58, 65, 28, 31, 42],                      // base cell 44
    [45, 39, 35, 25, 63, 59, 56],                      // base cell 45
    [46, 60, 56, 68, 27, 40, 35],                      // base cell 46
    [47, 38, 43, 29, 69, 51, 64],                      // base cell 47
    [48, 49, 30, 33, 67, 66, 50],                      // base cell 48
    [49, INVALID_BASE_CELL, 61, 66, 33, 48, 41],       // base cell 49 (pentagon)
    [50, 48, 32, 30, 70, 67, 52],                      // base cell 50
    [51, 69, 54, 71, 38, 47, 34],                      // base cell 51
    [52, 57, 70, 74, 32, 37, 50],                      // base cell 52
    [53, 61, 65, 75, 31, 41, 44],                      // base cell 53
    [54, 71, 55, 73, 34, 51, 36],                      // base cell 54
    [55, 40, 54, 36, 72, 60, 73],                      // base cell 55
    [56, 68, 63, 77, 35, 46, 45],                      // base cell 56
    [57, 59, 74, 78, 37, 39, 52],                      // base cell 57
    [58, INVALID_BASE_CELL, 62, 76, 44, 65, 42],       // base cell 58 (pentagon)
    [59, 63, 78, 79, 39, 45, 57],                      // base cell 59
    [60, 72, 68, 80, 40, 55, 46],                      // base cell 60
    [61, 53, 49, 41, 81, 75, 66],                      // base cell 61
    [62, 43, 58, 42, 82, 64, 76],                      // base cell 62
    [63, INVALID_BASE_CELL, 56, 45, 79, 59, 77],       // base cell 63 (pentagon)
    [64, 47, 62, 43, 84, 69, 82],                      // base cell 64
    [65, 58, 53, 44, 86, 76, 75],                      // base cell 65
    [66, 67, 81, 85, 49, 48, 61],                      // base cell 66
    [67, 66, 50, 48, 87, 85, 70],                      // base cell 67
    [68, 56, 60, 46, 90, 77, 80],                      // base cell 68
    [69, 51, 64, 47, 89, 71, 84],                      // base cell 69
    [70, 67, 52, 50, 83, 87, 74],                      // base cell 70
    [71, 89, 73, 91, 51, 69, 54],                      // base cell 71
    [72, INVALID_BASE_CELL, 73, 55, 80, 60, 88],       // base cell 72 (pentagon)
    [73, 91, 72, 88, 54, 71, 55],                      // base cell 73
    [74, 78, 83, 92, 52, 57, 70],                      // base cell 74
    [75, 65, 61, 53, 94, 86, 81],                      // base cell 75
    [76, 86, 82, 96, 58, 65, 62],                      // base cell 76
    [77, 63, 68, 56, 93, 79, 90],                      // base cell 77
    [78, 74, 59, 57, 95, 92, 79],                      // base cell 78
    [79, 78, 63, 59, 93, 95, 77],                      // base cell 79
    [80, 68, 72, 60, 99, 90, 88],                      // base cell 80
    [81, 85, 94, 101, 61, 66, 75],                     // base cell 81
    [82, 96, 84, 98, 62, 76, 64],                      // base cell 82
    [83, INVALID_BASE_CELL, 74, 70, 100, 87, 92],      // base cell 83 (pentagon)
    [84, 69, 82, 64, 97, 89, 98],                      // base cell 84
    [85, 87, 101, 102, 66, 67, 81],                    // base cell 85
    [86, 76, 75, 65, 104, 96, 94],                     // base cell 86
    [87, 83, 102, 100, 67, 70, 85],                    // base cell 87
    [88, 72, 91, 73, 99, 80, 105],                     // base cell 88
    [89, 97, 91, 103, 69, 84, 71],                     // base cell 89
    [90, 77, 80, 68, 106, 93, 99],                     // base cell 90
    [91, 73, 89, 71, 105, 88, 103],                    // base cell 91
    [92, 83, 78, 74, 108, 100, 95],                    // base cell 92
    [93, 79, 90, 77, 109, 95, 106],                    // base cell 93
    [94, 86, 81, 75, 107, 104, 101],                   // base cell 94
    [95, 92, 79, 78, 109, 108, 93],                    // base cell 95
    [96, 104, 98, 110, 76, 86, 82],                    // base cell 96
    [97, INVALID_BASE_CELL, 98, 84, 103, 89, 111],     // base cell 97 (pentagon)
    [98, 110, 97, 111, 82, 96, 84],                    // base cell 98
    [99, 80, 105, 88, 106, 90, 113],                   // base cell 99
    [100, 102, 83, 87, 108, 114, 92],                  // base cell 100
    [101, 102, 107, 112, 81, 85, 94],                  // base cell 101
    [102, 101, 87, 85, 114, 112, 100],                 // base cell 102
    [103, 91, 97, 89, 116, 105, 111],                  // base cell 103
    [104, 107, 110, 115, 86, 94, 96],                  // base cell 104
    [105, 88, 103, 91, 113, 99, 116],                  // base cell 105
    [106, 93, 99, 90, 117, 109, 113],                  // base cell 106
    [107, INVALID_BASE_CELL, 101, 94, 115, 104, 112],  // base cell 107 (pentagon)
    [108, 100, 95, 92, 118, 114, 109],                 // base cell 108
    [109, 108, 93, 95, 117, 118, 106],                 // base cell 109
    [110, 98, 104, 96, 119, 111, 115],                 // base cell 110
    [111, 97, 110, 98, 116, 103, 119],                 // base cell 111
    [112, 107, 102, 101, 120, 115, 114],               // base cell 112
    [113, 99, 116, 105, 117, 106, 121],                // base cell 113
    [114, 112, 100, 102, 118, 120, 108],               // base cell 114
    [115, 110, 107, 104, 120, 119, 112],               // base cell 115
    [116, 103, 119, 111, 113, 105, 121],               // base cell 116
    [117, INVALID_BASE_CELL, 109, 118, 113, 121, 106], // base cell 117 (pentagon)
    [118, 120, 108, 114, 117, 121, 109],               // base cell 118
    [119, 111, 115, 110, 121, 116, 120],               // base cell 119
    [120, 115, 114, 112, 121, 119, 118],               // base cell 120
    [121, 116, 120, 119, 117, 113, 118],               // base cell 121
];

/** @brief Neighboring base cell rotations in each IJK direction.
 *
 * For each base cell, for each direction, the number of 60 degree
 * CCW rotations to the coordinate system of the neighbor is given.
 * -1 indicates there is no neighbor in that direction.
 */
pub(crate) const baseCellNeighbor60CCWRots: [[i32; 7]; 122] = [
    [0, 5, 0, 0, 1, 5, 1],  // base cell 0
    [0, 0, 1, 0, 1, 0, 1],  // base cell 1
    [0, 0, 0, 0, 0, 5, 0],  // base cell 2
    [0, 5, 0, 0, 2, 5, 1],  // base cell 3
    [0, -1, 1, 0, 3, 4, 2], // base cell 4 (pentagon)
    [0, 0, 1, 0, 1, 0, 1],  // base cell 5
    [0, 0, 0, 3, 5, 5, 0],  // base cell 6
    [0, 0, 0, 0, 0, 5, 0],  // base cell 7
    [0, 5, 0, 0, 0, 5, 1],  // base cell 8
    [0, 0, 1, 3, 0, 0, 1],  // base cell 9
    [0, 0, 1, 3, 0, 0, 1],  // base cell 10
    [0, 3, 3, 3, 0, 0, 0],  // base cell 11
    [0, 5, 0, 0, 3, 5, 1],  // base cell 12
    [0, 0, 1, 0, 1, 0, 1],  // base cell 13
    [0, -1, 3, 0, 5, 2, 0], // base cell 14 (pentagon)
    [0, 5, 0, 0, 4, 5, 1],  // base cell 15
    [0, 0, 0, 0, 0, 5, 0],  // base cell 16
    [0, 3, 3, 3, 3, 0, 3],  // base cell 17
    [0, 0, 0, 3, 5, 5, 0],  // base cell 18
    [0, 3, 3, 3, 0, 0, 0],  // base cell 19
    [0, 3, 3, 3, 0, 3, 0],  // base cell 20
    [0, 0, 0, 3, 5, 5, 0],  // base cell 21
    [0, 0, 1, 0, 1, 0, 1],  // base cell 22
    [0, 3, 3, 3, 0, 3, 0],  // base cell 23
    [0, -1, 3, 0, 5, 2, 0], // base cell 24 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 25
    [0, 0, 0, 0, 0, 5, 0],  // base cell 26
    [0, 3, 0, 0, 0, 3, 3],  // base cell 27
    [0, 0, 1, 0, 1, 0, 1],  // base cell 28
    [0, 0, 1, 3, 0, 0, 1],  // base cell 29
    [0, 3, 3, 3, 0, 0, 0],  // base cell 30
    [0, 0, 0, 0, 0, 5, 0],  // base cell 31
    [0, 3, 3, 3, 3, 0, 3],  // base cell 32
    [0, 0, 1, 3, 0, 0, 1],  // base cell 33
    [0, 3, 3, 3, 3, 0, 3],  // base cell 34
    [0, 0, 3, 0, 3, 0, 3],  // base cell 35
    [0, 0, 0, 3, 0, 0, 3],  // base cell 36
    [0, 3, 0, 0, 0, 3, 3],  // base cell 37
    [0, -1, 3, 0, 5, 2, 0], // base cell 38 (pentagon)
    [0, 3, 0, 0, 3, 3, 0],  // base cell 39
    [0, 3, 0, 0, 3, 3, 0],  // base cell 40
    [0, 0, 0, 3, 5, 5, 0],  // base cell 41
    [0, 0, 0, 3, 5, 5, 0],  // base cell 42
    [0, 3, 3, 3, 0, 0, 0],  // base cell 43
    [0, 0, 1, 3, 0, 0, 1],  // base cell 44
    [0, 0, 3, 0, 0, 3, 3],  // base cell 45
    [0, 0, 0, 3, 0, 3, 0],  // base cell 46
    [0, 3, 3, 3, 0, 3, 0],  // base cell 47
    [0, 3, 3, 3, 0, 3, 0],  // base cell 48
    [0, -1, 3, 0, 5, 2, 0], // base cell 49 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 50
    [0, 3, 0, 0, 0, 3, 3],  // base cell 51
    [0, 0, 3, 0, 3, 0, 3],  // base cell 52
    [0, 3, 3, 3, 0, 0, 0],  // base cell 53
    [0, 0, 3, 0, 3, 0, 3],  // base cell 54
    [0, 0, 3, 0, 0, 3, 3],  // base cell 55
    [0, 3, 3, 3, 0, 0, 3],  // base cell 56
    [0, 0, 0, 3, 0, 3, 0],  // base cell 57
    [0, -1, 3, 0, 5, 2, 0], // base cell 58 (pentagon)
    [0, 3, 3, 3, 3, 3, 0],  // base cell 59
    [0, 3, 3, 3, 3, 3, 0],  // base cell 60
    [0, 3, 3, 3, 3, 0, 3],  // base cell 61
    [0, 3, 3, 3, 3, 0, 3],  // base cell 62
    [0, -1, 3, 0, 5, 2, 0], // base cell 63 (pentagon)
    [0, 0, 0, 3, 0, 0, 3],  // base cell 64
    [0, 3, 3, 3, 0, 3, 0],  // base cell 65
    [0, 3, 0, 0, 0, 3, 3],  // base cell 66
    [0, 3, 0, 0, 3, 3, 0],  // base cell 67
    [0, 3, 3, 3, 0, 0, 0],  // base cell 68
    [0, 3, 0, 0, 3, 3, 0],  // base cell 69
    [0, 0, 3, 0, 0, 3, 3],  // base cell 70
    [0, 0, 0, 3, 0, 3, 0],  // base cell 71
    [0, -1, 3, 0, 5, 2, 0], // base cell 72 (pentagon)
    [0, 3, 3, 3, 0, 0, 3],  // base cell 73
    [0, 3, 3, 3, 0, 0, 3],  // base cell 74
    [0, 0, 0, 3, 0, 0, 3],  // base cell 75
    [0, 3, 0, 0, 0, 3, 3],  // base cell 76
    [0, 0, 0, 3, 0, 5, 0],  // base cell 77
    [0, 3, 3, 3, 0, 0, 0],  // base cell 78
    [0, 0, 1, 3, 1, 0, 1],  // base cell 79
    [0, 0, 1, 3, 1, 0, 1],  // base cell 80
    [0, 0, 3, 0, 3, 0, 3],  // base cell 81
    [0, 0, 3, 0, 3, 0, 3],  // base cell 82
    [0, -1, 3, 0, 5, 2, 0], // base cell 83 (pentagon)
    [0, 0, 3, 0, 0, 3, 3],  // base cell 84
    [0, 0, 0, 3, 0, 3, 0],  // base cell 85
    [0, 3, 0, 0, 3, 3, 0],  // base cell 86
    [0, 3, 3, 3, 3, 3, 0],  // base cell 87
    [0, 0, 0, 3, 0, 5, 0],  // base cell 88
    [0, 3, 3, 3, 3, 3, 0],  // base cell 89
    [0, 0, 0, 0, 0, 0, 1],  // base cell 90
    [0, 3, 3, 3, 0, 0, 0],  // base cell 91
    [0, 0, 0, 3, 0, 5, 0],  // base cell 92
    [0, 5, 0, 0, 5, 5, 0],  // base cell 93
    [0, 0, 3, 0, 0, 3, 3],  // base cell 94
    [0, 0, 0, 0, 0, 0, 1],  // base cell 95
    [0, 0, 0, 3, 0, 3, 0],  // base cell 96
    [0, -1, 3, 0, 5, 2, 0], // base cell 97 (pentagon)
    [0, 3, 3, 3, 0, 0, 3],  // base cell 98
    [0, 5, 0, 0, 5, 5, 0],  // base cell 99
    [0, 0, 1, 3, 1, 0, 1],  // base cell 100
    [0, 3, 3, 3, 0, 0, 3],  // base cell 101
    [0, 3, 3, 3, 0, 0, 0],  // base cell 102
    [0, 0, 1, 3, 1, 0, 1],  // base cell 103
    [0, 3, 3, 3, 3, 3, 0],  // base cell 104
    [0, 0, 0, 0, 0, 0, 1],  // base cell 105
    [0, 0, 1, 0, 3, 5, 1],  // base cell 106
    [0, -1, 3, 0, 5, 2, 0], // base cell 107 (pentagon)
    [0, 5, 0, 0, 5, 5, 0],  // base cell 108
    [0, 0, 1, 0, 4, 5, 1],  // base cell 109
    [0, 3, 3, 3, 0, 0, 0],  // base cell 110
    [0, 0, 0, 3, 0, 5, 0],  // base cell 111
    [0, 0, 0, 3, 0, 5, 0],  // base cell 112
    [0, 0, 1, 0, 2, 5, 1],  // base cell 113
    [0, 0, 0, 0, 0, 0, 1],  // base cell 114
    [0, 0, 1, 3, 1, 0, 1],  // base cell 115
    [0, 5, 0, 0, 5, 5, 0],  // base cell 116
    [0, -1, 1, 0, 3, 4, 2], // base cell 117 (pentagon)
    [0, 0, 1, 0, 0, 5, 1],  // base cell 118
    [0, 0, 0, 0, 0, 0, 1],  // base cell 119
    [0, 5, 0, 0, 5, 5, 0],  // base cell 120
    [0, 0, 1, 0, 1, 5, 1],  // base cell 121
];

/** @brief Resolution 0 base cell lookup table for each face.
 *
 * Given the face number and a resolution 0 ijk+ coordinate in that face's
 * face-centered ijk coordinate system, gives the base cell located at that
 * coordinate and the number of 60 ccw rotations to rotate into that base
 * cell's orientation.
 *
 * Valid lookup coordinates are from (0, 0, 0) to (2, 2, 2).
 *
 * This table can be accessed using the functions `_faceIjkToBaseCell` and
 * `_faceIjkToBaseCellCCWrot60`
 */
pub(crate) const faceIjkBaseCells: [[[[BaseCellRotation; 3]; 3]; 3]; NUM_ICOSA_FACES] = [
    [
        // face 0
        [
            // i 0
            [
                BaseCellRotation::new(16, 0),
                BaseCellRotation::new(18, 0),
                BaseCellRotation::new(24, 0),
            ], // j 0
            [
                BaseCellRotation::new(33, 0),
                BaseCellRotation::new(30, 0),
                BaseCellRotation::new(32, 3),
            ], // j 1
            [
                BaseCellRotation::new(49, 1),
                BaseCellRotation::new(48, 3),
                BaseCellRotation::new(50, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(8, 0),
                BaseCellRotation::new(5, 5),
                BaseCellRotation::new(10, 5),
            ], // j 0
            [
                BaseCellRotation::new(22, 0),
                BaseCellRotation::new(16, 0),
                BaseCellRotation::new(18, 0),
            ], // j 1
            [
                BaseCellRotation::new(41, 1),
                BaseCellRotation::new(33, 0),
                BaseCellRotation::new(30, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(4, 0),
                BaseCellRotation::new(0, 5),
                BaseCellRotation::new(2, 5),
            ], // j 0
            [
                BaseCellRotation::new(15, 1),
                BaseCellRotation::new(8, 0),
                BaseCellRotation::new(5, 5),
            ], // j 1
            [
                BaseCellRotation::new(31, 1),
                BaseCellRotation::new(22, 0),
                BaseCellRotation::new(16, 0),
            ], // j 2
        ],
    ],
    [
        // face 1
        [
            // i 0
            [
                BaseCellRotation::new(2, 0),
                BaseCellRotation::new(6, 0),
                BaseCellRotation::new(14, 0),
            ], // j 0
            [
                BaseCellRotation::new(10, 0),
                BaseCellRotation::new(11, 0),
                BaseCellRotation::new(17, 3),
            ], // j 1
            [
                BaseCellRotation::new(24, 1),
                BaseCellRotation::new(23, 3),
                BaseCellRotation::new(25, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(0, 0),
                BaseCellRotation::new(1, 5),
                BaseCellRotation::new(9, 5),
            ], // j 0
            [
                BaseCellRotation::new(5, 0),
                BaseCellRotation::new(2, 0),
                BaseCellRotation::new(6, 0),
            ], // j 1
            [
                BaseCellRotation::new(18, 1),
                BaseCellRotation::new(10, 0),
                BaseCellRotation::new(11, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(4, 1),
                BaseCellRotation::new(3, 5),
                BaseCellRotation::new(7, 5),
            ], // j 0
            [
                BaseCellRotation::new(8, 1),
                BaseCellRotation::new(0, 0),
                BaseCellRotation::new(1, 5),
            ], // j 1
            [
                BaseCellRotation::new(16, 1),
                BaseCellRotation::new(5, 0),
                BaseCellRotation::new(2, 0),
            ], // j 2
        ],
    ],
    [
        // face 2
        [
            // i 0
            [
                BaseCellRotation::new(7, 0),
                BaseCellRotation::new(21, 0),
                BaseCellRotation::new(38, 0),
            ], // j 0
            [
                BaseCellRotation::new(9, 0),
                BaseCellRotation::new(19, 0),
                BaseCellRotation::new(34, 3),
            ], // j 1
            [
                BaseCellRotation::new(14, 1),
                BaseCellRotation::new(20, 3),
                BaseCellRotation::new(36, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(3, 0),
                BaseCellRotation::new(13, 5),
                BaseCellRotation::new(29, 5),
            ], // j 0
            [
                BaseCellRotation::new(1, 0),
                BaseCellRotation::new(7, 0),
                BaseCellRotation::new(21, 0),
            ], // j 1
            [
                BaseCellRotation::new(6, 1),
                BaseCellRotation::new(9, 0),
                BaseCellRotation::new(19, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(4, 2),
                BaseCellRotation::new(12, 5),
                BaseCellRotation::new(26, 5),
            ], // j 0
            [
                BaseCellRotation::new(0, 1),
                BaseCellRotation::new(3, 0),
                BaseCellRotation::new(13, 5),
            ], // j 1
            [
                BaseCellRotation::new(2, 1),
                BaseCellRotation::new(1, 0),
                BaseCellRotation::new(7, 0),
            ], // j 2
        ],
    ],
    [
        // face 3
        [
            // i 0
            [
                BaseCellRotation::new(26, 0),
                BaseCellRotation::new(42, 0),
                BaseCellRotation::new(58, 0),
            ], // j 0
            [
                BaseCellRotation::new(29, 0),
                BaseCellRotation::new(43, 0),
                BaseCellRotation::new(62, 3),
            ], // j 1
            [
                BaseCellRotation::new(38, 1),
                BaseCellRotation::new(47, 3),
                BaseCellRotation::new(64, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(12, 0),
                BaseCellRotation::new(28, 5),
                BaseCellRotation::new(44, 5),
            ], // j 0
            [
                BaseCellRotation::new(13, 0),
                BaseCellRotation::new(26, 0),
                BaseCellRotation::new(42, 0),
            ], // j 1
            [
                BaseCellRotation::new(21, 1),
                BaseCellRotation::new(29, 0),
                BaseCellRotation::new(43, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(4, 3),
                BaseCellRotation::new(15, 5),
                BaseCellRotation::new(31, 5),
            ], // j 0
            [
                BaseCellRotation::new(3, 1),
                BaseCellRotation::new(12, 0),
                BaseCellRotation::new(28, 5),
            ], // j 1
            [
                BaseCellRotation::new(7, 1),
                BaseCellRotation::new(13, 0),
                BaseCellRotation::new(26, 0),
            ], // j 2
        ],
    ],
    [
        // face 4
        [
            // i 0
            [
                BaseCellRotation::new(31, 0),
                BaseCellRotation::new(41, 0),
                BaseCellRotation::new(49, 0),
            ], // j 0
            [
                BaseCellRotation::new(44, 0),
                BaseCellRotation::new(53, 0),
                BaseCellRotation::new(61, 3),
            ], // j 1
            [
                BaseCellRotation::new(58, 1),
                BaseCellRotation::new(65, 3),
                BaseCellRotation::new(75, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(15, 0),
                BaseCellRotation::new(22, 5),
                BaseCellRotation::new(33, 5),
            ], // j 0
            [
                BaseCellRotation::new(28, 0),
                BaseCellRotation::new(31, 0),
                BaseCellRotation::new(41, 0),
            ], // j 1
            [
                BaseCellRotation::new(42, 1),
                BaseCellRotation::new(44, 0),
                BaseCellRotation::new(53, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(4, 4),
                BaseCellRotation::new(8, 5),
                BaseCellRotation::new(16, 5),
            ], // j 0
            [
                BaseCellRotation::new(12, 1),
                BaseCellRotation::new(15, 0),
                BaseCellRotation::new(22, 5),
            ], // j 1
            [
                BaseCellRotation::new(26, 1),
                BaseCellRotation::new(28, 0),
                BaseCellRotation::new(31, 0),
            ], // j 2
        ],
    ],
    [
        // face 5
        [
            // i 0
            [
                BaseCellRotation::new(50, 0),
                BaseCellRotation::new(48, 0),
                BaseCellRotation::new(49, 3),
            ], // j 0
            [
                BaseCellRotation::new(32, 0),
                BaseCellRotation::new(30, 3),
                BaseCellRotation::new(33, 3),
            ], // j 1
            [
                BaseCellRotation::new(24, 3),
                BaseCellRotation::new(18, 3),
                BaseCellRotation::new(16, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(70, 0),
                BaseCellRotation::new(67, 0),
                BaseCellRotation::new(66, 3),
            ], // j 0
            [
                BaseCellRotation::new(52, 3),
                BaseCellRotation::new(50, 0),
                BaseCellRotation::new(48, 0),
            ], // j 1
            [
                BaseCellRotation::new(37, 3),
                BaseCellRotation::new(32, 0),
                BaseCellRotation::new(30, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(83, 0),
                BaseCellRotation::new(87, 3),
                BaseCellRotation::new(85, 3),
            ], // j 0
            [
                BaseCellRotation::new(74, 3),
                BaseCellRotation::new(70, 0),
                BaseCellRotation::new(67, 0),
            ], // j 1
            [
                BaseCellRotation::new(57, 1),
                BaseCellRotation::new(52, 3),
                BaseCellRotation::new(50, 0),
            ], // j 2
        ],
    ],
    [
        // face 6
        [
            // i 0
            [
                BaseCellRotation::new(25, 0),
                BaseCellRotation::new(23, 0),
                BaseCellRotation::new(24, 3),
            ], // j 0
            [
                BaseCellRotation::new(17, 0),
                BaseCellRotation::new(11, 3),
                BaseCellRotation::new(10, 3),
            ], // j 1
            [
                BaseCellRotation::new(14, 3),
                BaseCellRotation::new(6, 3),
                BaseCellRotation::new(2, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(45, 0),
                BaseCellRotation::new(39, 0),
                BaseCellRotation::new(37, 3),
            ], // j 0
            [
                BaseCellRotation::new(35, 3),
                BaseCellRotation::new(25, 0),
                BaseCellRotation::new(23, 0),
            ], // j 1
            [
                BaseCellRotation::new(27, 3),
                BaseCellRotation::new(17, 0),
                BaseCellRotation::new(11, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(63, 0),
                BaseCellRotation::new(59, 3),
                BaseCellRotation::new(57, 3),
            ], // j 0
            [
                BaseCellRotation::new(56, 3),
                BaseCellRotation::new(45, 0),
                BaseCellRotation::new(39, 0),
            ], // j 1
            [
                BaseCellRotation::new(46, 3),
                BaseCellRotation::new(35, 3),
                BaseCellRotation::new(25, 0),
            ], // j 2
        ],
    ],
    [
        // face 7
        [
            // i 0
            [
                BaseCellRotation::new(36, 0),
                BaseCellRotation::new(20, 0),
                BaseCellRotation::new(14, 3),
            ], // j 0
            [
                BaseCellRotation::new(34, 0),
                BaseCellRotation::new(19, 3),
                BaseCellRotation::new(9, 3),
            ], // j 1
            [
                BaseCellRotation::new(38, 3),
                BaseCellRotation::new(21, 3),
                BaseCellRotation::new(7, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(55, 0),
                BaseCellRotation::new(40, 0),
                BaseCellRotation::new(27, 3),
            ], // j 0
            [
                BaseCellRotation::new(54, 3),
                BaseCellRotation::new(36, 0),
                BaseCellRotation::new(20, 0),
            ], // j 1
            [
                BaseCellRotation::new(51, 3),
                BaseCellRotation::new(34, 0),
                BaseCellRotation::new(19, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(72, 0),
                BaseCellRotation::new(60, 3),
                BaseCellRotation::new(46, 3),
            ], // j 0
            [
                BaseCellRotation::new(73, 3),
                BaseCellRotation::new(55, 0),
                BaseCellRotation::new(40, 0),
            ], // j 1
            [
                BaseCellRotation::new(71, 3),
                BaseCellRotation::new(54, 3),
                BaseCellRotation::new(36, 0),
            ], // j 2
        ],
    ],
    [
        // face 8
        [
            // i 0
            [
                BaseCellRotation::new(64, 0),
                BaseCellRotation::new(47, 0),
                BaseCellRotation::new(38, 3),
            ], // j 0
            [
                BaseCellRotation::new(62, 0),
                BaseCellRotation::new(43, 3),
                BaseCellRotation::new(29, 3),
            ], // j 1
            [
                BaseCellRotation::new(58, 3),
                BaseCellRotation::new(42, 3),
                BaseCellRotation::new(26, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(84, 0),
                BaseCellRotation::new(69, 0),
                BaseCellRotation::new(51, 3),
            ], // j 0
            [
                BaseCellRotation::new(82, 3),
                BaseCellRotation::new(64, 0),
                BaseCellRotation::new(47, 0),
            ], // j 1
            [
                BaseCellRotation::new(76, 3),
                BaseCellRotation::new(62, 0),
                BaseCellRotation::new(43, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(97, 0),
                BaseCellRotation::new(89, 3),
                BaseCellRotation::new(71, 3),
            ], // j 0
            [
                BaseCellRotation::new(98, 3),
                BaseCellRotation::new(84, 0),
                BaseCellRotation::new(69, 0),
            ], // j 1
            [
                BaseCellRotation::new(96, 3),
                BaseCellRotation::new(82, 3),
                BaseCellRotation::new(64, 0),
            ], // j 2
        ],
    ],
    [
        // face 9
        [
            // i 0
            [
                BaseCellRotation::new(75, 0),
                BaseCellRotation::new(65, 0),
                BaseCellRotation::new(58, 3),
            ], // j 0
            [
                BaseCellRotation::new(61, 0),
                BaseCellRotation::new(53, 3),
                BaseCellRotation::new(44, 3),
            ], // j 1
            [
                BaseCellRotation::new(49, 3),
                BaseCellRotation::new(41, 3),
                BaseCellRotation::new(31, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(94, 0),
                BaseCellRotation::new(86, 0),
                BaseCellRotation::new(76, 3),
            ], // j 0
            [
                BaseCellRotation::new(81, 3),
                BaseCellRotation::new(75, 0),
                BaseCellRotation::new(65, 0),
            ], // j 1
            [
                BaseCellRotation::new(66, 3),
                BaseCellRotation::new(61, 0),
                BaseCellRotation::new(53, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(107, 0),
                BaseCellRotation::new(104, 3),
                BaseCellRotation::new(96, 3),
            ], // j 0
            [
                BaseCellRotation::new(101, 3),
                BaseCellRotation::new(94, 0),
                BaseCellRotation::new(86, 0),
            ], // j 1
            [
                BaseCellRotation::new(85, 3),
                BaseCellRotation::new(81, 3),
                BaseCellRotation::new(75, 0),
            ], // j 2
        ],
    ],
    [
        // face 10
        [
            // i 0
            [
                BaseCellRotation::new(57, 0),
                BaseCellRotation::new(59, 0),
                BaseCellRotation::new(63, 3),
            ], // j 0
            [
                BaseCellRotation::new(74, 0),
                BaseCellRotation::new(78, 3),
                BaseCellRotation::new(79, 3),
            ], // j 1
            [
                BaseCellRotation::new(83, 3),
                BaseCellRotation::new(92, 3),
                BaseCellRotation::new(95, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(37, 0),
                BaseCellRotation::new(39, 3),
                BaseCellRotation::new(45, 3),
            ], // j 0
            [
                BaseCellRotation::new(52, 0),
                BaseCellRotation::new(57, 0),
                BaseCellRotation::new(59, 0),
            ], // j 1
            [
                BaseCellRotation::new(70, 3),
                BaseCellRotation::new(74, 0),
                BaseCellRotation::new(78, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(24, 0),
                BaseCellRotation::new(23, 3),
                BaseCellRotation::new(25, 3),
            ], // j 0
            [
                BaseCellRotation::new(32, 3),
                BaseCellRotation::new(37, 0),
                BaseCellRotation::new(39, 3),
            ], // j 1
            [
                BaseCellRotation::new(50, 3),
                BaseCellRotation::new(52, 0),
                BaseCellRotation::new(57, 0),
            ], // j 2
        ],
    ],
    [
        // face 11
        [
            // i 0
            [
                BaseCellRotation::new(46, 0),
                BaseCellRotation::new(60, 0),
                BaseCellRotation::new(72, 3),
            ], // j 0
            [
                BaseCellRotation::new(56, 0),
                BaseCellRotation::new(68, 3),
                BaseCellRotation::new(80, 3),
            ], // j 1
            [
                BaseCellRotation::new(63, 3),
                BaseCellRotation::new(77, 3),
                BaseCellRotation::new(90, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(27, 0),
                BaseCellRotation::new(40, 3),
                BaseCellRotation::new(55, 3),
            ], // j 0
            [
                BaseCellRotation::new(35, 0),
                BaseCellRotation::new(46, 0),
                BaseCellRotation::new(60, 0),
            ], // j 1
            [
                BaseCellRotation::new(45, 3),
                BaseCellRotation::new(56, 0),
                BaseCellRotation::new(68, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(14, 0),
                BaseCellRotation::new(20, 3),
                BaseCellRotation::new(36, 3),
            ], // j 0
            [
                BaseCellRotation::new(17, 3),
                BaseCellRotation::new(27, 0),
                BaseCellRotation::new(40, 3),
            ], // j 1
            [
                BaseCellRotation::new(25, 3),
                BaseCellRotation::new(35, 0),
                BaseCellRotation::new(46, 0),
            ], // j 2
        ],
    ],
    [
        // face 12
        [
            // i 0
            [
                BaseCellRotation::new(71, 0),
                BaseCellRotation::new(89, 0),
                BaseCellRotation::new(97, 3),
            ], // j 0
            [
                BaseCellRotation::new(73, 0),
                BaseCellRotation::new(91, 3),
                BaseCellRotation::new(103, 3),
            ], // j 1
            [
                BaseCellRotation::new(72, 3),
                BaseCellRotation::new(88, 3),
                BaseCellRotation::new(105, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(51, 0),
                BaseCellRotation::new(69, 3),
                BaseCellRotation::new(84, 3),
            ], // j 0
            [
                BaseCellRotation::new(54, 0),
                BaseCellRotation::new(71, 0),
                BaseCellRotation::new(89, 0),
            ], // j 1
            [
                BaseCellRotation::new(55, 3),
                BaseCellRotation::new(73, 0),
                BaseCellRotation::new(91, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(38, 0),
                BaseCellRotation::new(47, 3),
                BaseCellRotation::new(64, 3),
            ], // j 0
            [
                BaseCellRotation::new(34, 3),
                BaseCellRotation::new(51, 0),
                BaseCellRotation::new(69, 3),
            ], // j 1
            [
                BaseCellRotation::new(36, 3),
                BaseCellRotation::new(54, 0),
                BaseCellRotation::new(71, 0),
            ], // j 2
        ],
    ],
    [
        // face 13
        [
            // i 0
            [
                BaseCellRotation::new(96, 0),
                BaseCellRotation::new(104, 0),
                BaseCellRotation::new(107, 3),
            ], // j 0
            [
                BaseCellRotation::new(98, 0),
                BaseCellRotation::new(110, 3),
                BaseCellRotation::new(115, 3),
            ], // j 1
            [
                BaseCellRotation::new(97, 3),
                BaseCellRotation::new(111, 3),
                BaseCellRotation::new(119, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(76, 0),
                BaseCellRotation::new(86, 3),
                BaseCellRotation::new(94, 3),
            ], // j 0
            [
                BaseCellRotation::new(82, 0),
                BaseCellRotation::new(96, 0),
                BaseCellRotation::new(104, 0),
            ], // j 1
            [
                BaseCellRotation::new(84, 3),
                BaseCellRotation::new(98, 0),
                BaseCellRotation::new(110, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(58, 0),
                BaseCellRotation::new(65, 3),
                BaseCellRotation::new(75, 3),
            ], // j 0
            [
                BaseCellRotation::new(62, 3),
                BaseCellRotation::new(76, 0),
                BaseCellRotation::new(86, 3),
            ], // j 1
            [
                BaseCellRotation::new(64, 3),
                BaseCellRotation::new(82, 0),
                BaseCellRotation::new(96, 0),
            ], // j 2
        ],
    ],
    [
        // face 14
        [
            // i 0
            [
                BaseCellRotation::new(85, 0),
                BaseCellRotation::new(87, 0),
                BaseCellRotation::new(83, 3),
            ], // j 0
            [
                BaseCellRotation::new(101, 0),
                BaseCellRotation::new(102, 3),
                BaseCellRotation::new(100, 3),
            ], // j 1
            [
                BaseCellRotation::new(107, 3),
                BaseCellRotation::new(112, 3),
                BaseCellRotation::new(114, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(66, 0),
                BaseCellRotation::new(67, 3),
                BaseCellRotation::new(70, 3),
            ], // j 0
            [
                BaseCellRotation::new(81, 0),
                BaseCellRotation::new(85, 0),
                BaseCellRotation::new(87, 0),
            ], // j 1
            [
                BaseCellRotation::new(94, 3),
                BaseCellRotation::new(101, 0),
                BaseCellRotation::new(102, 3),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(49, 0),
                BaseCellRotation::new(48, 3),
                BaseCellRotation::new(50, 3),
            ], // j 0
            [
                BaseCellRotation::new(61, 3),
                BaseCellRotation::new(66, 0),
                BaseCellRotation::new(67, 3),
            ], // j 1
            [
                BaseCellRotation::new(75, 3),
                BaseCellRotation::new(81, 0),
                BaseCellRotation::new(85, 0),
            ], // j 2
        ],
    ],
    [
        // face 15
        [
            // i 0
            [
                BaseCellRotation::new(95, 0),
                BaseCellRotation::new(92, 0),
                BaseCellRotation::new(83, 0),
            ], // j 0
            [
                BaseCellRotation::new(79, 0),
                BaseCellRotation::new(78, 0),
                BaseCellRotation::new(74, 3),
            ], // j 1
            [
                BaseCellRotation::new(63, 1),
                BaseCellRotation::new(59, 3),
                BaseCellRotation::new(57, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(109, 0),
                BaseCellRotation::new(108, 0),
                BaseCellRotation::new(100, 5),
            ], // j 0
            [
                BaseCellRotation::new(93, 1),
                BaseCellRotation::new(95, 0),
                BaseCellRotation::new(92, 0),
            ], // j 1
            [
                BaseCellRotation::new(77, 1),
                BaseCellRotation::new(79, 0),
                BaseCellRotation::new(78, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(117, 4),
                BaseCellRotation::new(118, 5),
                BaseCellRotation::new(114, 5),
            ], // j 0
            [
                BaseCellRotation::new(106, 1),
                BaseCellRotation::new(109, 0),
                BaseCellRotation::new(108, 0),
            ], // j 1
            [
                BaseCellRotation::new(90, 1),
                BaseCellRotation::new(93, 1),
                BaseCellRotation::new(95, 0),
            ], // j 2
        ],
    ],
    [
        // face 16
        [
            // i 0
            [
                BaseCellRotation::new(90, 0),
                BaseCellRotation::new(77, 0),
                BaseCellRotation::new(63, 0),
            ], // j 0
            [
                BaseCellRotation::new(80, 0),
                BaseCellRotation::new(68, 0),
                BaseCellRotation::new(56, 3),
            ], // j 1
            [
                BaseCellRotation::new(72, 1),
                BaseCellRotation::new(60, 3),
                BaseCellRotation::new(46, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(106, 0),
                BaseCellRotation::new(93, 0),
                BaseCellRotation::new(79, 5),
            ], // j 0
            [
                BaseCellRotation::new(99, 1),
                BaseCellRotation::new(90, 0),
                BaseCellRotation::new(77, 0),
            ], // j 1
            [
                BaseCellRotation::new(88, 1),
                BaseCellRotation::new(80, 0),
                BaseCellRotation::new(68, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(117, 3),
                BaseCellRotation::new(109, 5),
                BaseCellRotation::new(95, 5),
            ], // j 0
            [
                BaseCellRotation::new(113, 1),
                BaseCellRotation::new(106, 0),
                BaseCellRotation::new(93, 0),
            ], // j 1
            [
                BaseCellRotation::new(105, 1),
                BaseCellRotation::new(99, 1),
                BaseCellRotation::new(90, 0),
            ], // j 2
        ],
    ],
    [
        // face 17
        [
            // i 0
            [
                BaseCellRotation::new(105, 0),
                BaseCellRotation::new(88, 0),
                BaseCellRotation::new(72, 0),
            ], // j 0
            [
                BaseCellRotation::new(103, 0),
                BaseCellRotation::new(91, 0),
                BaseCellRotation::new(73, 3),
            ], // j 1
            [
                BaseCellRotation::new(97, 1),
                BaseCellRotation::new(89, 3),
                BaseCellRotation::new(71, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(113, 0),
                BaseCellRotation::new(99, 0),
                BaseCellRotation::new(80, 5),
            ], // j 0
            [
                BaseCellRotation::new(116, 1),
                BaseCellRotation::new(105, 0),
                BaseCellRotation::new(88, 0),
            ], // j 1
            [
                BaseCellRotation::new(111, 1),
                BaseCellRotation::new(103, 0),
                BaseCellRotation::new(91, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(117, 2),
                BaseCellRotation::new(106, 5),
                BaseCellRotation::new(90, 5),
            ], // j 0
            [
                BaseCellRotation::new(121, 1),
                BaseCellRotation::new(113, 0),
                BaseCellRotation::new(99, 0),
            ], // j 1
            [
                BaseCellRotation::new(119, 1),
                BaseCellRotation::new(116, 1),
                BaseCellRotation::new(105, 0),
            ], // j 2
        ],
    ],
    [
        // face 18
        [
            // i 0
            [
                BaseCellRotation::new(119, 0),
                BaseCellRotation::new(111, 0),
                BaseCellRotation::new(97, 0),
            ], // j 0
            [
                BaseCellRotation::new(115, 0),
                BaseCellRotation::new(110, 0),
                BaseCellRotation::new(98, 3),
            ], // j 1
            [
                BaseCellRotation::new(107, 1),
                BaseCellRotation::new(104, 3),
                BaseCellRotation::new(96, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(121, 0),
                BaseCellRotation::new(116, 0),
                BaseCellRotation::new(103, 5),
            ], // j 0
            [
                BaseCellRotation::new(120, 1),
                BaseCellRotation::new(119, 0),
                BaseCellRotation::new(111, 0),
            ], // j 1
            [
                BaseCellRotation::new(112, 1),
                BaseCellRotation::new(115, 0),
                BaseCellRotation::new(110, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(117, 1),
                BaseCellRotation::new(113, 5),
                BaseCellRotation::new(105, 5),
            ], // j 0
            [
                BaseCellRotation::new(118, 1),
                BaseCellRotation::new(121, 0),
                BaseCellRotation::new(116, 0),
            ], // j 1
            [
                BaseCellRotation::new(114, 1),
                BaseCellRotation::new(120, 1),
                BaseCellRotation::new(119, 0),
            ], // j 2
        ],
    ],
    [
        // face 19
        [
            // i 0
            [
                BaseCellRotation::new(114, 0),
                BaseCellRotation::new(112, 0),
                BaseCellRotation::new(107, 0),
            ], // j 0
            [
                BaseCellRotation::new(100, 0),
                BaseCellRotation::new(102, 0),
                BaseCellRotation::new(101, 3),
            ], // j 1
            [
                BaseCellRotation::new(83, 1),
                BaseCellRotation::new(87, 3),
                BaseCellRotation::new(85, 3),
            ], // j 2
        ],
        [
            // i 1
            [
                BaseCellRotation::new(118, 0),
                BaseCellRotation::new(120, 0),
                BaseCellRotation::new(115, 5),
            ], // j 0
            [
                BaseCellRotation::new(108, 1),
                BaseCellRotation::new(114, 0),
                BaseCellRotation::new(112, 0),
            ], // j 1
            [
                BaseCellRotation::new(92, 1),
                BaseCellRotation::new(100, 0),
                BaseCellRotation::new(102, 0),
            ], // j 2
        ],
        [
            // i 2
            [
                BaseCellRotation::new(117, 0),
                BaseCellRotation::new(121, 5),
                BaseCellRotation::new(119, 5),
            ], // j 0
            [
                BaseCellRotation::new(109, 1),
                BaseCellRotation::new(118, 0),
                BaseCellRotation::new(120, 0),
            ], // j 1
            [
                BaseCellRotation::new(95, 1),
                BaseCellRotation::new(108, 1),
                BaseCellRotation::new(114, 0),
            ], // j 2
        ],
    ],
];

/** @brief Resolution 0 base cell data table.
 *
 * For each base cell, gives the "home" face and ijk+ coordinates on that face,
 * whether or not the base cell is a pentagon. Additionally, if the base cell
 * is a pentagon, the two cw offset rotation adjacent faces are given (-1
 * indicates that no cw offset rotation faces exist for this base cell).
 */
pub(crate) const baseCellData: [BaseCellData; NUM_BASE_CELLS] = [
    BaseCellData::new(1, [1, 0, 0], false, [0, 0]), // base cell 0
    BaseCellData::new(2, [1, 1, 0], false, [0, 0]), // base cell 1
    BaseCellData::new(1, [0, 0, 0], false, [0, 0]), // base cell 2
    BaseCellData::new(2, [1, 0, 0], false, [0, 0]), // base cell 3
    BaseCellData::new(0, [2, 0, 0], true, [-1, -1]), // base cell 4
    BaseCellData::new(1, [1, 1, 0], false, [0, 0]), // base cell 5
    BaseCellData::new(1, [0, 0, 1], false, [0, 0]), // base cell 6
    BaseCellData::new(2, [0, 0, 0], false, [0, 0]), // base cell 7
    BaseCellData::new(0, [1, 0, 0], false, [0, 0]), // base cell 8
    BaseCellData::new(2, [0, 1, 0], false, [0, 0]), // base cell 9
    BaseCellData::new(1, [0, 1, 0], false, [0, 0]), // base cell 10
    BaseCellData::new(1, [0, 1, 1], false, [0, 0]), // base cell 11
    BaseCellData::new(3, [1, 0, 0], false, [0, 0]), // base cell 12
    BaseCellData::new(3, [1, 1, 0], false, [0, 0]), // base cell 13
    BaseCellData::new(11, [2, 0, 0], true, [2, 6]), // base cell 14
    BaseCellData::new(4, [1, 0, 0], false, [0, 0]), // base cell 15
    BaseCellData::new(0, [0, 0, 0], false, [0, 0]), // base cell 16
    BaseCellData::new(6, [0, 1, 0], false, [0, 0]), // base cell 17
    BaseCellData::new(0, [0, 0, 1], false, [0, 0]), // base cell 18
    BaseCellData::new(2, [0, 1, 1], false, [0, 0]), // base cell 19
    BaseCellData::new(7, [0, 0, 1], false, [0, 0]), // base cell 20
    BaseCellData::new(2, [0, 0, 1], false, [0, 0]), // base cell 21
    BaseCellData::new(0, [1, 1, 0], false, [0, 0]), // base cell 22
    BaseCellData::new(6, [0, 0, 1], false, [0, 0]), // base cell 23
    BaseCellData::new(10, [2, 0, 0], true, [1, 5]), // base cell 24
    BaseCellData::new(6, [0, 0, 0], false, [0, 0]), // base cell 25
    BaseCellData::new(3, [0, 0, 0], false, [0, 0]), // base cell 26
    BaseCellData::new(11, [1, 0, 0], false, [0, 0]), // base cell 27
    BaseCellData::new(4, [1, 1, 0], false, [0, 0]), // base cell 28
    BaseCellData::new(3, [0, 1, 0], false, [0, 0]), // base cell 29
    BaseCellData::new(0, [0, 1, 1], false, [0, 0]), // base cell 30
    BaseCellData::new(4, [0, 0, 0], false, [0, 0]), // base cell 31
    BaseCellData::new(5, [0, 1, 0], false, [0, 0]), // base cell 32
    BaseCellData::new(0, [0, 1, 0], false, [0, 0]), // base cell 33
    BaseCellData::new(7, [0, 1, 0], false, [0, 0]), // base cell 34
    BaseCellData::new(11, [1, 1, 0], false, [0, 0]), // base cell 35
    BaseCellData::new(7, [0, 0, 0], false, [0, 0]), // base cell 36
    BaseCellData::new(10, [1, 0, 0], false, [0, 0]), // base cell 37
    BaseCellData::new(12, [2, 0, 0], true, [3, 7]), // base cell 38
    BaseCellData::new(6, [1, 0, 1], false, [0, 0]), // base cell 39
    BaseCellData::new(7, [1, 0, 1], false, [0, 0]), // base cell 40
    BaseCellData::new(4, [0, 0, 1], false, [0, 0]), // base cell 41
    BaseCellData::new(3, [0, 0, 1], false, [0, 0]), // base cell 42
    BaseCellData::new(3, [0, 1, 1], false, [0, 0]), // base cell 43
    BaseCellData::new(4, [0, 1, 0], false, [0, 0]), // base cell 44
    BaseCellData::new(6, [1, 0, 0], false, [0, 0]), // base cell 45
    BaseCellData::new(11, [0, 0, 0], false, [0, 0]), // base cell 46
    BaseCellData::new(8, [0, 0, 1], false, [0, 0]), // base cell 47
    BaseCellData::new(5, [0, 0, 1], false, [0, 0]), // base cell 48
    BaseCellData::new(14, [2, 0, 0], true, [0, 9]), // truease cell 49
    BaseCellData::new(5, [0, 0, 0], false, [0, 0]), // base cell 50
    BaseCellData::new(12, [1, 0, 0], false, [0, 0]), // base cell 51
    BaseCellData::new(10, [1, 1, 0], false, [0, 0]), // base cell 52
    BaseCellData::new(4, [0, 1, 1], false, [0, 0]), // base cell 53
    BaseCellData::new(12, [1, 1, 0], false, [0, 0]), // base cell 54
    BaseCellData::new(7, [1, 0, 0], false, [0, 0]), // base cell 55
    BaseCellData::new(11, [0, 1, 0], false, [0, 0]), // base cell 56
    BaseCellData::new(10, [0, 0, 0], false, [0, 0]), // base cell 57
    BaseCellData::new(13, [2, 0, 0], true, [4, 8]), // base cell 58
    BaseCellData::new(10, [0, 0, 1], false, [0, 0]), // falsease cell 59
    BaseCellData::new(11, [0, 0, 1], false, [0, 0]), // base cell 60
    BaseCellData::new(9, [0, 1, 0], false, [0, 0]), // base cell 61
    BaseCellData::new(8, [0, 1, 0], false, [0, 0]), // base cell 62
    BaseCellData::new(6, [2, 0, 0], true, [11, 15]), // base cell 63
    BaseCellData::new(8, [0, 0, 0], false, [0, 0]), // base cell 64
    BaseCellData::new(9, [0, 0, 1], false, [0, 0]), // base cell 65
    BaseCellData::new(14, [1, 0, 0], false, [0, 0]), // base cell 66
    BaseCellData::new(5, [1, 0, 1], false, [0, 0]), // base cell 67
    BaseCellData::new(16, [0, 1, 1], false, [0, 0]), // base cell 68
    BaseCellData::new(8, [1, 0, 1], false, [0, 0]), // base cell 69
    BaseCellData::new(5, [1, 0, 0], false, [0, 0]), // base cell 70
    BaseCellData::new(12, [0, 0, 0], false, [0, 0]), // base cell 71
    BaseCellData::new(7, [2, 0, 0], true, [12, 16]), // base cell 72
    BaseCellData::new(12, [0, 1, 0], false, [0, 0]), // base cell 73
    BaseCellData::new(10, [0, 1, 0], false, [0, 0]), // base cell 74
    BaseCellData::new(9, [0, 0, 0], false, [0, 0]), // base cell 75
    BaseCellData::new(13, [1, 0, 0], false, [0, 0]), // base cell 76
    BaseCellData::new(16, [0, 0, 1], false, [0, 0]), // base cell 77
    BaseCellData::new(15, [0, 1, 1], false, [0, 0]), // base cell 78
    BaseCellData::new(15, [0, 1, 0], false, [0, 0]), // base cell 79
    BaseCellData::new(16, [0, 1, 0], false, [0, 0]), // base cell 80
    BaseCellData::new(14, [1, 1, 0], false, [0, 0]), // base cell 81
    BaseCellData::new(13, [1, 1, 0], false, [0, 0]), // base cell 82
    BaseCellData::new(5, [2, 0, 0], true, [10, 19]), // base cell 83
    BaseCellData::new(8, [1, 0, 0], false, [0, 0]), // base cell 84
    BaseCellData::new(14, [0, 0, 0], false, [0, 0]), // base cell 85
    BaseCellData::new(9, [1, 0, 1], false, [0, 0]), // base cell 86
    BaseCellData::new(14, [0, 0, 1], false, [0, 0]), // base cell 87
    BaseCellData::new(17, [0, 0, 1], false, [0, 0]), // base cell 88
    BaseCellData::new(12, [0, 0, 1], false, [0, 0]), // base cell 89
    BaseCellData::new(16, [0, 0, 0], false, [0, 0]), // base cell 90
    BaseCellData::new(17, [0, 1, 1], false, [0, 0]), // base cell 91
    BaseCellData::new(15, [0, 0, 1], false, [0, 0]), // base cell 92
    BaseCellData::new(16, [1, 0, 1], false, [0, 0]), // base cell 93
    BaseCellData::new(9, [1, 0, 0], false, [0, 0]), // base cell 94
    BaseCellData::new(15, [0, 0, 0], false, [0, 0]), // base cell 95
    BaseCellData::new(13, [0, 0, 0], false, [0, 0]), // base cell 96
    BaseCellData::new(8, [2, 0, 0], true, [13, 17]), // base cell 97
    BaseCellData::new(13, [0, 1, 0], false, [0, 0]), // base cell 98
    BaseCellData::new(17, [1, 0, 1], false, [0, 0]), // base cell 99
    BaseCellData::new(19, [0, 1, 0], false, [0, 0]), // base cell 100
    BaseCellData::new(14, [0, 1, 0], false, [0, 0]), // base cell 101
    BaseCellData::new(19, [0, 1, 1], false, [0, 0]), // base cell 102
    BaseCellData::new(17, [0, 1, 0], false, [0, 0]), // base cell 103
    BaseCellData::new(13, [0, 0, 1], false, [0, 0]), // base cell 104
    BaseCellData::new(17, [0, 0, 0], false, [0, 0]), // base cell 105
    BaseCellData::new(16, [1, 0, 0], false, [0, 0]), // base cell 106
    BaseCellData::new(9, [2, 0, 0], true, [14, 18]), // base cell 107
    BaseCellData::new(15, [1, 0, 1], false, [0, 0]), // base cell 108
    BaseCellData::new(15, [1, 0, 0], false, [0, 0]), // base cell 109
    BaseCellData::new(18, [0, 1, 1], false, [0, 0]), // base cell 110
    BaseCellData::new(18, [0, 0, 1], false, [0, 0]), // base cell 111
    BaseCellData::new(19, [0, 0, 1], false, [0, 0]), // base cell 112
    BaseCellData::new(17, [1, 0, 0], false, [0, 0]), // base cell 113
    BaseCellData::new(19, [0, 0, 0], false, [0, 0]), // base cell 114
    BaseCellData::new(18, [0, 1, 0], false, [0, 0]), // base cell 115
    BaseCellData::new(18, [1, 0, 1], false, [0, 0]), // base cell 116
    BaseCellData::new(19, [2, 0, 0], true, [-1, -1]), // base cell 117
    BaseCellData::new(19, [1, 0, 0], false, [0, 0]), // base cell 118
    BaseCellData::new(18, [0, 0, 0], false, [0, 0]), // base cell 119
    BaseCellData::new(19, [1, 0, 1], false, [0, 0]), // base cell 120
    BaseCellData::new(18, [1, 0, 0], false, [0, 0]), // base cell 121
];

/// Return whether or not the indicated base cell is a pentagon.
pub(crate) fn _isBaseCellPentagon(baseCell: usize) -> bool {
    if baseCell < 0 || baseCell >= NUM_BASE_CELLS {
        // LCOV_EXCL_BR_LINE
        // Base cells less than zero can not be represented in an index
        return false;
    }

    baseCellData[baseCell].isPentagon
}

/** @brief Return whether the indicated base cell is a pentagon where all
 * neighbors are oriented towards it. */
pub(crate) fn _isBaseCellPolarPentagon(baseCell: usize) -> bool {
    baseCell == 4 || baseCell == 117
}

/**
 * @brief Given a base cell and the face it appears on, return
 *        the number of 60' ccw rotations for that base cell's
 *        coordinate system.
 * @returns The number of rotations, or INVALID_ROTATIONS if the base
 *          cell is not found on the given face
 */
pub(crate) fn _baseCellToCCWrot60(baseCell: usize, face: usize) -> i32 {
    if face < 0 || face > NUM_ICOSA_FACES {
        return INVALID_ROTATIONS;
    }

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                if faceIjkBaseCells[face][i][j][k].baseCell == baseCell as i32 {
                    return faceIjkBaseCells[face][i][j][k].ccwRot60;
                }
            }
        }
    }

    INVALID_ROTATIONS
}

/// Return whether or not the tested face is a cw offset face.
pub(crate) fn _baseCellIsCwOffset(baseCell: usize, testFace: i32) -> bool {
    baseCellData[baseCell as usize].cwOffsetPent[0] == testFace
        || baseCellData[baseCell as usize].cwOffsetPent[1] == testFace
}

/// Return the neighboring base cell in the given direction.
pub(crate) fn _getBaseCellNeighbor(baseCell: i32, dir: Direction) -> i32 {
    baseCellNeighbors[baseCell as usize][dir as usize]
}

/// Return the direction from the origin base cell to the neighbor.
/// Returns INVALID_DIGIT if the base cells are not neighbors.
pub(crate) fn _getBaseCellDirection(originBaseCell: i32, neighboringBaseCell: i32) -> Direction {
    todo!()
    /*
    for (Direction dir = CENTER_DIGIT; dir < NUM_DIGITS; dir++) {
        int testBaseCell = _getBaseCellNeighbor(originBaseCell, dir);
        if (testBaseCell == neighboringBaseCell) {
            return dir;
        }
    }
    */

    //Direction::INVALID_DIGIT
}

/// res0IndexCount returns the number of resolution 0 indexes
///
///@return int count of resolution 0 indexes
pub(crate) fn res0IndexCount() -> usize {
    NUM_BASE_CELLS
}

/**
 * getRes0Indexes generates all base cells storing them into the provided
 * memory pointer. Buffer must be of size NUM_BASE_CELLS * sizeof(H3Index).
 *
 * @param out H3Index* the memory to store the resulting base cells in
 */
pub(crate) fn getRes0Indexes() -> [H3Index; NUM_BASE_CELLS] {
    let mut out: [H3Index; NUM_BASE_CELLS];
    for bc in 0..NUM_BASE_CELLS {
        todo!()
        /*
        H3Index baseCell = H3_INIT;
        H3_SET_MODE(baseCell, H3_HEXAGON_MODE);
        H3_SET_BASE_CELL(baseCell, bc);
        out[bc] = baseCell;
        */
    }

    out
}

use crate::constants::NUM_ICOSA_FACES;

/// base cell at a given ijk and required rotations into its system
pub(crate) struct BaseCellRotation {
    /// base cell number
    pub(crate) baseCell: i32,
    /// number of ccw 60 degree rotations relative to current face
    pub(crate) ccwRot60: i32,
}

impl BaseCellRotation {
    pub(crate) const fn new(baseCell: i32, ccwRot60: i32) -> Self {
        Self { baseCell, ccwRot60 }
    }
}

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

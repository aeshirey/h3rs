use crate::{
    constants::{H3_MODE, NUM_ICOSA_FACES},
    direction::Direction,
    faceijk::FaceIJK,
    h3index::H3Index,
};

/// Base cell related lookup tables and access functions.

/// information on a single base cell
pub struct BaseCellData {
    /// "home" face and normalized ijk coordinates on that face
    pub homeFijk: FaceIJK,
    /// is this base cell a pentagon?
    pub isPentagon: bool,
    /// if a pentagon, what are its two clockwise offset faces?
    pub cwOffsetPent: [i32; 2],
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

#[derive(PartialEq)]
pub(crate) struct BaseCell(pub i32);
//pub(crate) type BaseCell = i32;

impl BaseCell {
    // The number of H2 base cells
    pub const NUM_BASE_CELLS: usize = 122;

    pub const INVALID_BASE_CELL: Self = BaseCell(127);

    /// Return whether or not the indicated base cell is a pentagon.
    pub(crate) fn _isBaseCellPentagon(&self) -> bool {
        let BaseCell(baseCell) = *self;
        if baseCell < 0 || baseCell as usize >= Self::NUM_BASE_CELLS {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return false;
        }

        baseCellData[baseCell as usize].isPentagon
    }

    /// Return the neighboring base cell in the given direction.
    pub(crate) fn _getBaseCellNeighbor(&self, dir: Direction) -> BaseCell {
        baseCellNeighbors[self.0 as usize][dir as usize]
    }

    /** Return whether the indicated base cell is a pentagon where all
     * neighbors are oriented towards it. */
    pub(crate) fn _isBaseCellPolarPentagon(&self) -> bool {
        self.0 == 4 || self.0 == 117
    }

    /**
     * @brief Given a base cell and the face it appears on, return
     *        the number of 60' ccw rotations for that base cell's
     *        coordinate system.
     * @returns The number of rotations, or INVALID_ROTATIONS if the base
     *          cell is not found on the given face
     */
    pub(crate) fn _baseCellToCCWrot60(&self, face: usize) -> i32 {
        if face < 0 || face > NUM_ICOSA_FACES {
            return INVALID_ROTATIONS;
        }

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    if faceIjkBaseCells[face][i][j][k].baseCell == *self {
                        return faceIjkBaseCells[face][i][j][k].ccwRot60;
                    }
                }
            }
        }

        INVALID_ROTATIONS
    }

    /// Return whether or not the tested face is a cw offset face.
    pub(crate) fn _baseCellIsCwOffset(&self, testFace: i32) -> bool {
        baseCellData[self.0 as usize].cwOffsetPent[0] == testFace
            || baseCellData[self.0 as usize].cwOffsetPent[1] == testFace
    }

    /// Find the FaceIJK given a base cell.
    pub(crate) fn _baseCellToFaceIjk(&self) -> FaceIJK {
        baseCellData[self.0 as usize].homeFijk
    }
}

impl From<usize> for BaseCell {
    fn from(v: usize) -> Self {
        Self(v as i32)
    }
}

impl From<i32> for BaseCell {
    fn from(v: i32) -> Self {
        Self(v as i32)
    }
}

/// base cell at a given ijk and required rotations into its system
struct BaseCellRotation {
    /// base cell number
    pub baseCell: BaseCell,
    /// number of ccw 60 degree rotations relative to current
    pub ccwRot60: i32,
}

impl BaseCellRotation {
    pub const fn new(baseCell: i32, ccwRot60: i32) -> Self {
        let baseCell = BaseCell(baseCell);
        Self { baseCell, ccwRot60 }
    }
}

/**
 * getRes0Indexes generates all base cells storing them into the provided
 * memory pointer. Buffer must be of size NUM_BASE_CELLS * sizeof(H3Index).
 *
 * @param out H3Index* the memory to store the resulting base cells in
 */
pub fn getRes0Indexes() -> [H3Index; BaseCell::NUM_BASE_CELLS] {
    let result: [H3Index; BaseCell::NUM_BASE_CELLS] = [H3Index::H3_NULL; BaseCell::NUM_BASE_CELLS];

    for bc in 0..BaseCell::NUM_BASE_CELLS {
        let mut baseCell = H3Index::H3_INIT();
        baseCell.H3_SET_MODE(H3_MODE::HEXAGON);
        baseCell.H3_SET_BASE_CELL(BaseCell(bc as i32));
        result[bc] = baseCell;
    }

    result
}

/// Neighboring base cell ID in each IJK direction.
///
///For each base cell, for each direction, the neighboring base
///cell ID is given. 127 indicates there is no neighbor in that direction.
pub(crate) const baseCellNeighbors: [[BaseCell; 7]; 122] = [
    [
        BaseCell(0),
        BaseCell(1),
        BaseCell(5),
        BaseCell(2),
        BaseCell(4),
        BaseCell(3),
        BaseCell(8),
    ], // base cell 0
    [
        BaseCell(1),
        BaseCell(7),
        BaseCell(6),
        BaseCell(9),
        BaseCell(0),
        BaseCell(3),
        BaseCell(2),
    ], // base cell 1
    [
        BaseCell(2),
        BaseCell(6),
        BaseCell(10),
        BaseCell(11),
        BaseCell(0),
        BaseCell(1),
        BaseCell(5),
    ], // base cell 2
    [
        BaseCell(3),
        BaseCell(13),
        BaseCell(1),
        BaseCell(7),
        BaseCell(4),
        BaseCell(12),
        BaseCell(0),
    ], // base cell 3
    [
        BaseCell(4),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(15),
        BaseCell(8),
        BaseCell(3),
        BaseCell(0),
        BaseCell(12),
    ], // base cell 4 (pentagon)
    [
        BaseCell(5),
        BaseCell(2),
        BaseCell(18),
        BaseCell(10),
        BaseCell(8),
        BaseCell(0),
        BaseCell(16),
    ], // base cell 5
    [
        BaseCell(6),
        BaseCell(14),
        BaseCell(11),
        BaseCell(17),
        BaseCell(1),
        BaseCell(9),
        BaseCell(2),
    ], // base cell 6
    [
        BaseCell(7),
        BaseCell(21),
        BaseCell(9),
        BaseCell(19),
        BaseCell(3),
        BaseCell(13),
        BaseCell(1),
    ], // base cell 7
    [
        BaseCell(8),
        BaseCell(5),
        BaseCell(22),
        BaseCell(16),
        BaseCell(4),
        BaseCell(0),
        BaseCell(15),
    ], // base cell 8
    [
        BaseCell(9),
        BaseCell(19),
        BaseCell(14),
        BaseCell(20),
        BaseCell(1),
        BaseCell(7),
        BaseCell(6),
    ], // base cell 9
    [
        BaseCell(10),
        BaseCell(11),
        BaseCell(24),
        BaseCell(23),
        BaseCell(5),
        BaseCell(2),
        BaseCell(18),
    ], // base cell 10
    [
        BaseCell(11),
        BaseCell(17),
        BaseCell(23),
        BaseCell(25),
        BaseCell(2),
        BaseCell(6),
        BaseCell(10),
    ], // base cell 11
    [
        BaseCell(12),
        BaseCell(28),
        BaseCell(13),
        BaseCell(26),
        BaseCell(4),
        BaseCell(15),
        BaseCell(3),
    ], // base cell 12
    [
        BaseCell(13),
        BaseCell(26),
        BaseCell(21),
        BaseCell(29),
        BaseCell(3),
        BaseCell(12),
        BaseCell(7),
    ], // base cell 13
    [
        BaseCell(14),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(17),
        BaseCell(27),
        BaseCell(9),
        BaseCell(20),
        BaseCell(6),
    ], // base cell 14 (pentagon)
    [
        BaseCell(15),
        BaseCell(22),
        BaseCell(28),
        BaseCell(31),
        BaseCell(4),
        BaseCell(8),
        BaseCell(12),
    ], // base cell 15
    [
        BaseCell(16),
        BaseCell(18),
        BaseCell(33),
        BaseCell(30),
        BaseCell(8),
        BaseCell(5),
        BaseCell(22),
    ], // base cell 16
    [
        BaseCell(17),
        BaseCell(11),
        BaseCell(14),
        BaseCell(6),
        BaseCell(35),
        BaseCell(25),
        BaseCell(27),
    ], // base cell 17
    [
        BaseCell(18),
        BaseCell(24),
        BaseCell(30),
        BaseCell(32),
        BaseCell(5),
        BaseCell(10),
        BaseCell(16),
    ], // base cell 18
    [
        BaseCell(19),
        BaseCell(34),
        BaseCell(20),
        BaseCell(36),
        BaseCell(7),
        BaseCell(21),
        BaseCell(9),
    ], // base cell 19
    [
        BaseCell(20),
        BaseCell(14),
        BaseCell(19),
        BaseCell(9),
        BaseCell(40),
        BaseCell(27),
        BaseCell(36),
    ], // base cell 20
    [
        BaseCell(21),
        BaseCell(38),
        BaseCell(19),
        BaseCell(34),
        BaseCell(13),
        BaseCell(29),
        BaseCell(7),
    ], // base cell 21
    [
        BaseCell(22),
        BaseCell(16),
        BaseCell(41),
        BaseCell(33),
        BaseCell(15),
        BaseCell(8),
        BaseCell(31),
    ], // base cell 22
    [
        BaseCell(23),
        BaseCell(24),
        BaseCell(11),
        BaseCell(10),
        BaseCell(39),
        BaseCell(37),
        BaseCell(25),
    ], // base cell 23
    [
        BaseCell(24),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(32),
        BaseCell(37),
        BaseCell(10),
        BaseCell(23),
        BaseCell(18),
    ], // base cell 24 (pentagon)
    [
        BaseCell(25),
        BaseCell(23),
        BaseCell(17),
        BaseCell(11),
        BaseCell(45),
        BaseCell(39),
        BaseCell(35),
    ], // base cell 25
    [
        BaseCell(26),
        BaseCell(42),
        BaseCell(29),
        BaseCell(43),
        BaseCell(12),
        BaseCell(28),
        BaseCell(13),
    ], // base cell 26
    [
        BaseCell(27),
        BaseCell(40),
        BaseCell(35),
        BaseCell(46),
        BaseCell(14),
        BaseCell(20),
        BaseCell(17),
    ], // base cell 27
    [
        BaseCell(28),
        BaseCell(31),
        BaseCell(42),
        BaseCell(44),
        BaseCell(12),
        BaseCell(15),
        BaseCell(26),
    ], // base cell 28
    [
        BaseCell(29),
        BaseCell(43),
        BaseCell(38),
        BaseCell(47),
        BaseCell(13),
        BaseCell(26),
        BaseCell(21),
    ], // base cell 29
    [
        BaseCell(30),
        BaseCell(32),
        BaseCell(48),
        BaseCell(50),
        BaseCell(16),
        BaseCell(18),
        BaseCell(33),
    ], // base cell 30
    [
        BaseCell(31),
        BaseCell(41),
        BaseCell(44),
        BaseCell(53),
        BaseCell(15),
        BaseCell(22),
        BaseCell(28),
    ], // base cell 31
    [
        BaseCell(32),
        BaseCell(30),
        BaseCell(24),
        BaseCell(18),
        BaseCell(52),
        BaseCell(50),
        BaseCell(37),
    ], // base cell 32
    [
        BaseCell(33),
        BaseCell(30),
        BaseCell(49),
        BaseCell(48),
        BaseCell(22),
        BaseCell(16),
        BaseCell(41),
    ], // base cell 33
    [
        BaseCell(34),
        BaseCell(19),
        BaseCell(38),
        BaseCell(21),
        BaseCell(54),
        BaseCell(36),
        BaseCell(51),
    ], // base cell 34
    [
        BaseCell(35),
        BaseCell(46),
        BaseCell(45),
        BaseCell(56),
        BaseCell(17),
        BaseCell(27),
        BaseCell(25),
    ], // base cell 35
    [
        BaseCell(36),
        BaseCell(20),
        BaseCell(34),
        BaseCell(19),
        BaseCell(55),
        BaseCell(40),
        BaseCell(54),
    ], // base cell 36
    [
        BaseCell(37),
        BaseCell(39),
        BaseCell(52),
        BaseCell(57),
        BaseCell(24),
        BaseCell(23),
        BaseCell(32),
    ], // base cell 37
    [
        BaseCell(38),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(34),
        BaseCell(51),
        BaseCell(29),
        BaseCell(47),
        BaseCell(21),
    ], // base cell 38 (pentagon)
    [
        BaseCell(39),
        BaseCell(37),
        BaseCell(25),
        BaseCell(23),
        BaseCell(59),
        BaseCell(57),
        BaseCell(45),
    ], // base cell 39
    [
        BaseCell(40),
        BaseCell(27),
        BaseCell(36),
        BaseCell(20),
        BaseCell(60),
        BaseCell(46),
        BaseCell(55),
    ], // base cell 40
    [
        BaseCell(41),
        BaseCell(49),
        BaseCell(53),
        BaseCell(61),
        BaseCell(22),
        BaseCell(33),
        BaseCell(31),
    ], // base cell 41
    [
        BaseCell(42),
        BaseCell(58),
        BaseCell(43),
        BaseCell(62),
        BaseCell(28),
        BaseCell(44),
        BaseCell(26),
    ], // base cell 42
    [
        BaseCell(43),
        BaseCell(62),
        BaseCell(47),
        BaseCell(64),
        BaseCell(26),
        BaseCell(42),
        BaseCell(29),
    ], // base cell 43
    [
        BaseCell(44),
        BaseCell(53),
        BaseCell(58),
        BaseCell(65),
        BaseCell(28),
        BaseCell(31),
        BaseCell(42),
    ], // base cell 44
    [
        BaseCell(45),
        BaseCell(39),
        BaseCell(35),
        BaseCell(25),
        BaseCell(63),
        BaseCell(59),
        BaseCell(56),
    ], // base cell 45
    [
        BaseCell(46),
        BaseCell(60),
        BaseCell(56),
        BaseCell(68),
        BaseCell(27),
        BaseCell(40),
        BaseCell(35),
    ], // base cell 46
    [
        BaseCell(47),
        BaseCell(38),
        BaseCell(43),
        BaseCell(29),
        BaseCell(69),
        BaseCell(51),
        BaseCell(64),
    ], // base cell 47
    [
        BaseCell(48),
        BaseCell(49),
        BaseCell(30),
        BaseCell(33),
        BaseCell(67),
        BaseCell(66),
        BaseCell(50),
    ], // base cell 48
    [
        BaseCell(49),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(61),
        BaseCell(66),
        BaseCell(33),
        BaseCell(48),
        BaseCell(41),
    ], // base cell 49 (pentagon)
    [
        BaseCell(50),
        BaseCell(48),
        BaseCell(32),
        BaseCell(30),
        BaseCell(70),
        BaseCell(67),
        BaseCell(52),
    ], // base cell 50
    [
        BaseCell(51),
        BaseCell(69),
        BaseCell(54),
        BaseCell(71),
        BaseCell(38),
        BaseCell(47),
        BaseCell(34),
    ], // base cell 51
    [
        BaseCell(52),
        BaseCell(57),
        BaseCell(70),
        BaseCell(74),
        BaseCell(32),
        BaseCell(37),
        BaseCell(50),
    ], // base cell 52
    [
        BaseCell(53),
        BaseCell(61),
        BaseCell(65),
        BaseCell(75),
        BaseCell(31),
        BaseCell(41),
        BaseCell(44),
    ], // base cell 53
    [
        BaseCell(54),
        BaseCell(71),
        BaseCell(55),
        BaseCell(73),
        BaseCell(34),
        BaseCell(51),
        BaseCell(36),
    ], // base cell 54
    [
        BaseCell(55),
        BaseCell(40),
        BaseCell(54),
        BaseCell(36),
        BaseCell(72),
        BaseCell(60),
        BaseCell(73),
    ], // base cell 55
    [
        BaseCell(56),
        BaseCell(68),
        BaseCell(63),
        BaseCell(77),
        BaseCell(35),
        BaseCell(46),
        BaseCell(45),
    ], // base cell 56
    [
        BaseCell(57),
        BaseCell(59),
        BaseCell(74),
        BaseCell(78),
        BaseCell(37),
        BaseCell(39),
        BaseCell(52),
    ], // base cell 57
    [
        BaseCell(58),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(62),
        BaseCell(76),
        BaseCell(44),
        BaseCell(65),
        BaseCell(42),
    ], // base cell 58 (pentagon)
    [
        BaseCell(59),
        BaseCell(63),
        BaseCell(78),
        BaseCell(79),
        BaseCell(39),
        BaseCell(45),
        BaseCell(57),
    ], // base cell 59
    [
        BaseCell(60),
        BaseCell(72),
        BaseCell(68),
        BaseCell(80),
        BaseCell(40),
        BaseCell(55),
        BaseCell(46),
    ], // base cell 60
    [
        BaseCell(61),
        BaseCell(53),
        BaseCell(49),
        BaseCell(41),
        BaseCell(81),
        BaseCell(75),
        BaseCell(66),
    ], // base cell 61
    [
        BaseCell(62),
        BaseCell(43),
        BaseCell(58),
        BaseCell(42),
        BaseCell(82),
        BaseCell(64),
        BaseCell(76),
    ], // base cell 62
    [
        BaseCell(63),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(56),
        BaseCell(45),
        BaseCell(79),
        BaseCell(59),
        BaseCell(77),
    ], // base cell 63 (pentagon)
    [
        BaseCell(64),
        BaseCell(47),
        BaseCell(62),
        BaseCell(43),
        BaseCell(84),
        BaseCell(69),
        BaseCell(82),
    ], // base cell 64
    [
        BaseCell(65),
        BaseCell(58),
        BaseCell(53),
        BaseCell(44),
        BaseCell(86),
        BaseCell(76),
        BaseCell(75),
    ], // base cell 65
    [
        BaseCell(66),
        BaseCell(67),
        BaseCell(81),
        BaseCell(85),
        BaseCell(49),
        BaseCell(48),
        BaseCell(61),
    ], // base cell 66
    [
        BaseCell(67),
        BaseCell(66),
        BaseCell(50),
        BaseCell(48),
        BaseCell(87),
        BaseCell(85),
        BaseCell(70),
    ], // base cell 67
    [
        BaseCell(68),
        BaseCell(56),
        BaseCell(60),
        BaseCell(46),
        BaseCell(90),
        BaseCell(77),
        BaseCell(80),
    ], // base cell 68
    [
        BaseCell(69),
        BaseCell(51),
        BaseCell(64),
        BaseCell(47),
        BaseCell(89),
        BaseCell(71),
        BaseCell(84),
    ], // base cell 69
    [
        BaseCell(70),
        BaseCell(67),
        BaseCell(52),
        BaseCell(50),
        BaseCell(83),
        BaseCell(87),
        BaseCell(74),
    ], // base cell 70
    [
        BaseCell(71),
        BaseCell(89),
        BaseCell(73),
        BaseCell(91),
        BaseCell(51),
        BaseCell(69),
        BaseCell(54),
    ], // base cell 71
    [
        BaseCell(72),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(73),
        BaseCell(55),
        BaseCell(80),
        BaseCell(60),
        BaseCell(88),
    ], // base cell 72 (pentagon)
    [
        BaseCell(73),
        BaseCell(91),
        BaseCell(72),
        BaseCell(88),
        BaseCell(54),
        BaseCell(71),
        BaseCell(55),
    ], // base cell 73
    [
        BaseCell(74),
        BaseCell(78),
        BaseCell(83),
        BaseCell(92),
        BaseCell(52),
        BaseCell(57),
        BaseCell(70),
    ], // base cell 74
    [
        BaseCell(75),
        BaseCell(65),
        BaseCell(61),
        BaseCell(53),
        BaseCell(94),
        BaseCell(86),
        BaseCell(81),
    ], // base cell 75
    [
        BaseCell(76),
        BaseCell(86),
        BaseCell(82),
        BaseCell(96),
        BaseCell(58),
        BaseCell(65),
        BaseCell(62),
    ], // base cell 76
    [
        BaseCell(77),
        BaseCell(63),
        BaseCell(68),
        BaseCell(56),
        BaseCell(93),
        BaseCell(79),
        BaseCell(90),
    ], // base cell 77
    [
        BaseCell(78),
        BaseCell(74),
        BaseCell(59),
        BaseCell(57),
        BaseCell(95),
        BaseCell(92),
        BaseCell(79),
    ], // base cell 78
    [
        BaseCell(79),
        BaseCell(78),
        BaseCell(63),
        BaseCell(59),
        BaseCell(93),
        BaseCell(95),
        BaseCell(77),
    ], // base cell 79
    [
        BaseCell(80),
        BaseCell(68),
        BaseCell(72),
        BaseCell(60),
        BaseCell(99),
        BaseCell(90),
        BaseCell(88),
    ], // base cell 80
    [
        BaseCell(81),
        BaseCell(85),
        BaseCell(94),
        BaseCell(101),
        BaseCell(61),
        BaseCell(66),
        BaseCell(75),
    ], // base cell 81
    [
        BaseCell(82),
        BaseCell(96),
        BaseCell(84),
        BaseCell(98),
        BaseCell(62),
        BaseCell(76),
        BaseCell(64),
    ], // base cell 82
    [
        BaseCell(83),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(74),
        BaseCell(70),
        BaseCell(100),
        BaseCell(87),
        BaseCell(92),
    ], // base cell 83 (pentagon)
    [
        BaseCell(84),
        BaseCell(69),
        BaseCell(82),
        BaseCell(64),
        BaseCell(97),
        BaseCell(89),
        BaseCell(98),
    ], // base cell 84
    [
        BaseCell(85),
        BaseCell(87),
        BaseCell(101),
        BaseCell(102),
        BaseCell(66),
        BaseCell(67),
        BaseCell(81),
    ], // base cell 85
    [
        BaseCell(86),
        BaseCell(76),
        BaseCell(75),
        BaseCell(65),
        BaseCell(104),
        BaseCell(96),
        BaseCell(94),
    ], // base cell 86
    [
        BaseCell(87),
        BaseCell(83),
        BaseCell(102),
        BaseCell(100),
        BaseCell(67),
        BaseCell(70),
        BaseCell(85),
    ], // base cell 87
    [
        BaseCell(88),
        BaseCell(72),
        BaseCell(91),
        BaseCell(73),
        BaseCell(99),
        BaseCell(80),
        BaseCell(105),
    ], // base cell 88
    [
        BaseCell(89),
        BaseCell(97),
        BaseCell(91),
        BaseCell(103),
        BaseCell(69),
        BaseCell(84),
        BaseCell(71),
    ], // base cell 89
    [
        BaseCell(90),
        BaseCell(77),
        BaseCell(80),
        BaseCell(68),
        BaseCell(106),
        BaseCell(93),
        BaseCell(99),
    ], // base cell 90
    [
        BaseCell(91),
        BaseCell(73),
        BaseCell(89),
        BaseCell(71),
        BaseCell(105),
        BaseCell(88),
        BaseCell(103),
    ], // base cell 91
    [
        BaseCell(92),
        BaseCell(83),
        BaseCell(78),
        BaseCell(74),
        BaseCell(108),
        BaseCell(100),
        BaseCell(95),
    ], // base cell 92
    [
        BaseCell(93),
        BaseCell(79),
        BaseCell(90),
        BaseCell(77),
        BaseCell(109),
        BaseCell(95),
        BaseCell(106),
    ], // base cell 93
    [
        BaseCell(94),
        BaseCell(86),
        BaseCell(81),
        BaseCell(75),
        BaseCell(107),
        BaseCell(104),
        BaseCell(101),
    ], // base cell 94
    [
        BaseCell(95),
        BaseCell(92),
        BaseCell(79),
        BaseCell(78),
        BaseCell(109),
        BaseCell(108),
        BaseCell(93),
    ], // base cell 95
    [
        BaseCell(96),
        BaseCell(104),
        BaseCell(98),
        BaseCell(110),
        BaseCell(76),
        BaseCell(86),
        BaseCell(82),
    ], // base cell 96
    [
        BaseCell(97),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(98),
        BaseCell(84),
        BaseCell(103),
        BaseCell(89),
        BaseCell(111),
    ], // base cell 97 (pentagon)
    [
        BaseCell(98),
        BaseCell(110),
        BaseCell(97),
        BaseCell(111),
        BaseCell(82),
        BaseCell(96),
        BaseCell(84),
    ], // base cell 98
    [
        BaseCell(99),
        BaseCell(80),
        BaseCell(105),
        BaseCell(88),
        BaseCell(106),
        BaseCell(90),
        BaseCell(113),
    ], // base cell 99
    [
        BaseCell(100),
        BaseCell(102),
        BaseCell(83),
        BaseCell(87),
        BaseCell(108),
        BaseCell(114),
        BaseCell(92),
    ], // base cell 100
    [
        BaseCell(101),
        BaseCell(102),
        BaseCell(107),
        BaseCell(112),
        BaseCell(81),
        BaseCell(85),
        BaseCell(94),
    ], // base cell 101
    [
        BaseCell(102),
        BaseCell(101),
        BaseCell(87),
        BaseCell(85),
        BaseCell(114),
        BaseCell(112),
        BaseCell(100),
    ], // base cell 102
    [
        BaseCell(103),
        BaseCell(91),
        BaseCell(97),
        BaseCell(89),
        BaseCell(116),
        BaseCell(105),
        BaseCell(111),
    ], // base cell 103
    [
        BaseCell(104),
        BaseCell(107),
        BaseCell(110),
        BaseCell(115),
        BaseCell(86),
        BaseCell(94),
        BaseCell(96),
    ], // base cell 104
    [
        BaseCell(105),
        BaseCell(88),
        BaseCell(103),
        BaseCell(91),
        BaseCell(113),
        BaseCell(99),
        BaseCell(116),
    ], // base cell 105
    [
        BaseCell(106),
        BaseCell(93),
        BaseCell(99),
        BaseCell(90),
        BaseCell(117),
        BaseCell(109),
        BaseCell(113),
    ], // base cell 106
    [
        BaseCell(107),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(101),
        BaseCell(94),
        BaseCell(115),
        BaseCell(104),
        BaseCell(112),
    ], // base cell 107 (pentagon)
    [
        BaseCell(108),
        BaseCell(100),
        BaseCell(95),
        BaseCell(92),
        BaseCell(118),
        BaseCell(114),
        BaseCell(109),
    ], // base cell 108
    [
        BaseCell(109),
        BaseCell(108),
        BaseCell(93),
        BaseCell(95),
        BaseCell(117),
        BaseCell(118),
        BaseCell(106),
    ], // base cell 109
    [
        BaseCell(110),
        BaseCell(98),
        BaseCell(104),
        BaseCell(96),
        BaseCell(119),
        BaseCell(111),
        BaseCell(115),
    ], // base cell 110
    [
        BaseCell(111),
        BaseCell(97),
        BaseCell(110),
        BaseCell(98),
        BaseCell(116),
        BaseCell(103),
        BaseCell(119),
    ], // base cell 111
    [
        BaseCell(112),
        BaseCell(107),
        BaseCell(102),
        BaseCell(101),
        BaseCell(120),
        BaseCell(115),
        BaseCell(114),
    ], // base cell 112
    [
        BaseCell(113),
        BaseCell(99),
        BaseCell(116),
        BaseCell(105),
        BaseCell(117),
        BaseCell(106),
        BaseCell(121),
    ], // base cell 113
    [
        BaseCell(114),
        BaseCell(112),
        BaseCell(100),
        BaseCell(102),
        BaseCell(118),
        BaseCell(120),
        BaseCell(108),
    ], // base cell 114
    [
        BaseCell(115),
        BaseCell(110),
        BaseCell(107),
        BaseCell(104),
        BaseCell(120),
        BaseCell(119),
        BaseCell(112),
    ], // base cell 115
    [
        BaseCell(116),
        BaseCell(103),
        BaseCell(119),
        BaseCell(111),
        BaseCell(113),
        BaseCell(105),
        BaseCell(121),
    ], // base cell 116
    [
        BaseCell(117),
        BaseCell::INVALID_BASE_CELL,
        BaseCell(109),
        BaseCell(118),
        BaseCell(113),
        BaseCell(121),
        BaseCell(106),
    ], // base cell 117 (pentagon)
    [
        BaseCell(118),
        BaseCell(120),
        BaseCell(108),
        BaseCell(114),
        BaseCell(117),
        BaseCell(121),
        BaseCell(109),
    ], // base cell 118
    [
        BaseCell(119),
        BaseCell(111),
        BaseCell(115),
        BaseCell(110),
        BaseCell(121),
        BaseCell(116),
        BaseCell(120),
    ], // base cell 119
    [
        BaseCell(120),
        BaseCell(115),
        BaseCell(114),
        BaseCell(112),
        BaseCell(121),
        BaseCell(119),
        BaseCell(118),
    ], // base cell 120
    [
        BaseCell(121),
        BaseCell(116),
        BaseCell(120),
        BaseCell(119),
        BaseCell(117),
        BaseCell(113),
        BaseCell(118),
    ], // base cell 121
];

/** @brief Neighboring base cell rotations in each IJK direction.
 *
 * For each base cell, for each direction, the number of 60 degree
 * CCW rotations to the coordinate system of the neighbor is given.
 * -1 indicates there is no neighbor in that direction.
 */
pub(crate) const baseCellNeighbor60CCWRots: [[BaseCell; 7]; 122] = [
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 0
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 1
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 2
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(2),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 3
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(1),
        BaseCell(0),
        BaseCell(3),
        BaseCell(4),
        BaseCell(2),
    ], // base cell 4 (pentagon)
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 5
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 6
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 7
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 8
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 9
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 10
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 11
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 12
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 13
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 14 (pentagon)
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(4),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 15
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 16
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 17
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 18
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 19
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 20
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 21
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 22
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 23
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 24 (pentagon)
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 25
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 26
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 27
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 28
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 29
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 30
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 31
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 32
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 33
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 34
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 35
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 36
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 37
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 38 (pentagon)
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 39
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 40
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 41
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 42
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 43
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 44
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 45
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 46
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 47
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 48
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 49 (pentagon)
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 50
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 51
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 52
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 53
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 54
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 55
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 56
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 57
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 58 (pentagon)
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 59
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 60
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 61
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 62
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 63 (pentagon)
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 64
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 65
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 66
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 67
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 68
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 69
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 70
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 71
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 72 (pentagon)
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 73
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 74
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 75
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 76
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 77
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 78
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 79
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 80
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 81
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 82
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 83 (pentagon)
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 84
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 85
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 86
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 87
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 88
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 89
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 90
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 91
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 92
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 93
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
    ], // base cell 94
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 95
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 96
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 97 (pentagon)
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 98
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 99
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 100
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
    ], // base cell 101
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 102
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 103
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
    ], // base cell 104
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 105
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(3),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 106
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(2),
        BaseCell(0),
    ], // base cell 107 (pentagon)
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 108
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(4),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 109
    [
        BaseCell(0),
        BaseCell(3),
        BaseCell(3),
        BaseCell(3),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
    ], // base cell 110
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 111
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(3),
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 112
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(2),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 113
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 114
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(3),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 115
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 116
    [
        BaseCell(0),
        BaseCell(-1),
        BaseCell(1),
        BaseCell(0),
        BaseCell(3),
        BaseCell(4),
        BaseCell(2),
    ], // base cell 117 (pentagon)
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 118
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
    ], // base cell 119
    [
        BaseCell(0),
        BaseCell(5),
        BaseCell(0),
        BaseCell(0),
        BaseCell(5),
        BaseCell(5),
        BaseCell(0),
    ], // base cell 120
    [
        BaseCell(0),
        BaseCell(0),
        BaseCell(1),
        BaseCell(0),
        BaseCell(1),
        BaseCell(5),
        BaseCell(1),
    ], // base cell 121
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
pub(crate) const baseCellData: [BaseCellData; BaseCell::NUM_BASE_CELLS] = [
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
    BaseCell::NUM_BASE_CELLS
}

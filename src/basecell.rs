use crate::{
    basecellrotation::faceIjkBaseCells, constants::NUM_ICOSA_FACES, faceijk::FaceIJK, Direction,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BaseCell(pub(crate) i32);

const INVALID_ROTATIONS: i32 = -1;

impl BaseCell {
    /// The number of H3 base cells
    pub(crate) const NUM_BASE_CELLS: usize = 122;

    pub(crate) const INVALID: BaseCell = BaseCell(-1);

    pub(crate) const fn new(cellnum: i32) -> Self {
        Self(cellnum)
    }

    /*
        int H3_EXPORT(h3IsPentagon)(H3Index h) {
        return _isBaseCellPentagon(H3_GET_BASE_CELL(h)) &&
               !_h3LeadingNonZeroDigit(h);
    }

    /** @brief Return whether or not the indicated base cell is a pentagon. */
    int _isBaseCellPentagon(int baseCell) {
        if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {  // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return false;
        }
        return baseCellData[baseCell].isPentagon;
    }
    */

    /// Return whether or not the indicated base cell is a pentagon.
    pub(crate) fn _isBaseCellPentagon(&self) -> bool {
        if self.0 < 0 || self.0 >= Self::NUM_BASE_CELLS as i32 {
            // LCOV_EXCL_BR_LINE
            // Base cells less than zero can not be represented in an index
            return false;
        }

        baseCellData[self.0 as usize].cwOffsetPent.is_some()
    }

    /// Return whether the indicated base cell is a pentagon where all neighbors are oriented towards it.
    pub(crate) fn _isBaseCellPolarPentagon(&self) -> bool {
        self.0 == 4 || self.0 == 117
    }

    /// Return the neighboring base cell in the given direction.
    pub(crate) fn _getBaseCellNeighbor(&self, dir: &Direction) -> BaseCell {
        let d: u64 = (*dir).into();
        baseCellNeighbors[self.0 as usize][d as usize]
    }

    /// Return the direction from the origin base cell to the neighbor.
    /// Returns INVALID_DIGIT if the base cells are not neighbors.
    pub(crate) fn _getBaseCellDirection(&self, neighboringBaseCell: BaseCell) -> Direction {
        for dir in Direction::VALID_DIRECTIONS.iter() {
            let testBaseCell: BaseCell = self._getBaseCellNeighbor(dir);
            if testBaseCell == neighboringBaseCell {
                return *dir;
            }
        }

        Direction::INVALID_DIGIT
    }

    /**
     * @brief Given a base cell and the face it appears on, return
     *        the number of 60' ccw rotations for that base cell's
     *        coordinate system.
     * @returns The number of rotations, or INVALID_ROTATIONS if the base
     *          cell is not found on the given face
     */
    pub(crate) fn _baseCellToCCWrot60(&self, face: usize) -> i32 {
        if face > NUM_ICOSA_FACES {
            return INVALID_ROTATIONS;
        }

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    if faceIjkBaseCells[face][i][j][k].baseCell == self.0 {
                        return faceIjkBaseCells[face][i][j][k].ccwRot60;
                    }
                }
            }
        }

        INVALID_ROTATIONS
    }

    /// Return whether or not the tested face is a cw offset face.
    pub(crate) fn _baseCellIsCwOffset(&self, testface: &FaceIJK) -> bool {
        //baseCellData[self.0 as usize].cwOffsetPent[0] == testFace
        //    || baseCellData[self.0 as usize].cwOffsetPent[1] == testFace
        if let Some(bcd) = baseCellData[self.0 as usize].cwOffsetPent {
            bcd[0] == testface.face || bcd[1] == testface.face
        } else {
            false
        }
    }

    /// Find the FaceIJK given a base cell.
    pub(crate) fn _baseCellToFaceIjk(&self) -> FaceIJK {
        baseCellData[self.0 as usize].homeFijk
    }
}

macro_rules! basecell_impl {
    ($t: ty) => {
        impl From<BaseCell> for $t {
            fn from(bc: BaseCell) -> $t {
                bc.0 as $t
            }
        }

        impl From<$t> for BaseCell {
            fn from(n: $t) -> Self {
                BaseCell(n as i32)
            }
        }

        impl std::cmp::PartialEq<$t> for BaseCell {
            fn eq(&self, other: &$t) -> bool {
                self.0.eq(&(*other as i32))
            }
        }

        impl std::cmp::PartialOrd<$t> for BaseCell {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                Some(self.0.cmp(&(*other as i32)))
            }
        }
    };
}

basecell_impl!(i32);
basecell_impl!(usize);

/// information on a single base cell
pub(crate) struct BaseCellData {
    ///  "home" face and normalized ijk coordinates on that face
    homeFijk: FaceIJK,

    //  is this base cell a pentagon?
    //isPentagon: bool,
    ///  if a pentagon, what are its two clockwise offset faces?
    cwOffsetPent: Option<[i32; 2]>,
    //cwOffsetPent : [i32; 2],
}

impl BaseCellData {
    const fn new(homeFijk: FaceIJK, cwOffsetPent: Option<[i32; 2]>) -> Self {
        Self {
            homeFijk,
            cwOffsetPent,
        }
    }
}

/** Resolution 0 base cell data table.
 *
 * For each base cell, gives the "home" face and ijk+ coordinates on that face,
 * whether or not the base cell is a pentagon. Additionally, if the base cell
 * is a pentagon, the two cw offset rotation adjacent faces are given (-1
 * indicates that no cw offset rotation faces exist for this base cell).
 */
const baseCellData: [BaseCellData; BaseCell::NUM_BASE_CELLS] = [
    BaseCellData::new(FaceIJK::new(1, (1, 0, 0)), None), // base cell 0
    BaseCellData::new(FaceIJK::new(2, (1, 1, 0)), None), // base cell 1
    BaseCellData::new(FaceIJK::new(1, (0, 0, 0)), None), // base cell 2
    BaseCellData::new(FaceIJK::new(2, (1, 0, 0)), None), // base cell 3
    BaseCellData::new(FaceIJK::new(0, (2, 0, 0)), Some([-1, -1])), // base cell 4
    BaseCellData::new(FaceIJK::new(1, (1, 1, 0)), None), // base cell 5
    BaseCellData::new(FaceIJK::new(1, (0, 0, 1)), None), // base cell 6
    BaseCellData::new(FaceIJK::new(2, (0, 0, 0)), None), // base cell 7
    BaseCellData::new(FaceIJK::new(0, (1, 0, 0)), None), // base cell 8
    BaseCellData::new(FaceIJK::new(2, (0, 1, 0)), None), // base cell 9
    BaseCellData::new(FaceIJK::new(1, (0, 1, 0)), None), // base cell 10
    BaseCellData::new(FaceIJK::new(1, (0, 1, 1)), None), // base cell 11
    BaseCellData::new(FaceIJK::new(3, (1, 0, 0)), None), // base cell 12
    BaseCellData::new(FaceIJK::new(3, (1, 1, 0)), None), // base cell 13
    BaseCellData::new(FaceIJK::new(11, (2, 0, 0)), Some([2, 6])), // base cell 14
    BaseCellData::new(FaceIJK::new(4, (1, 0, 0)), None), // base cell 15
    BaseCellData::new(FaceIJK::new(0, (0, 0, 0)), None), // base cell 16
    BaseCellData::new(FaceIJK::new(6, (0, 1, 0)), None), // base cell 17
    BaseCellData::new(FaceIJK::new(0, (0, 0, 1)), None), // base cell 18
    BaseCellData::new(FaceIJK::new(2, (0, 1, 1)), None), // base cell 19
    BaseCellData::new(FaceIJK::new(7, (0, 0, 1)), None), // base cell 20
    BaseCellData::new(FaceIJK::new(2, (0, 0, 1)), None), // base cell 21
    BaseCellData::new(FaceIJK::new(0, (1, 1, 0)), None), // base cell 22
    BaseCellData::new(FaceIJK::new(6, (0, 0, 1)), None), // base cell 23
    BaseCellData::new(FaceIJK::new(10, (2, 0, 0)), Some([1, 5])), // base cell 24
    BaseCellData::new(FaceIJK::new(6, (0, 0, 0)), None), // base cell 25
    BaseCellData::new(FaceIJK::new(3, (0, 0, 0)), None), // base cell 26
    BaseCellData::new(FaceIJK::new(11, (1, 0, 0)), None), // base cell 27
    BaseCellData::new(FaceIJK::new(4, (1, 1, 0)), None), // base cell 28
    BaseCellData::new(FaceIJK::new(3, (0, 1, 0)), None), // base cell 29
    BaseCellData::new(FaceIJK::new(0, (0, 1, 1)), None), // base cell 30
    BaseCellData::new(FaceIJK::new(4, (0, 0, 0)), None), // base cell 31
    BaseCellData::new(FaceIJK::new(5, (0, 1, 0)), None), // base cell 32
    BaseCellData::new(FaceIJK::new(0, (0, 1, 0)), None), // base cell 33
    BaseCellData::new(FaceIJK::new(7, (0, 1, 0)), None), // base cell 34
    BaseCellData::new(FaceIJK::new(11, (1, 1, 0)), None), // base cell 35
    BaseCellData::new(FaceIJK::new(7, (0, 0, 0)), None), // base cell 36
    BaseCellData::new(FaceIJK::new(10, (1, 0, 0)), None), // base cell 37
    BaseCellData::new(FaceIJK::new(12, (2, 0, 0)), Some([3, 7])), // base cell 38
    BaseCellData::new(FaceIJK::new(6, (1, 0, 1)), None), // base cell 39
    BaseCellData::new(FaceIJK::new(7, (1, 0, 1)), None), // base cell 40
    BaseCellData::new(FaceIJK::new(4, (0, 0, 1)), None), // base cell 41
    BaseCellData::new(FaceIJK::new(3, (0, 0, 1)), None), // base cell 42
    BaseCellData::new(FaceIJK::new(3, (0, 1, 1)), None), // base cell 43
    BaseCellData::new(FaceIJK::new(4, (0, 1, 0)), None), // base cell 44
    BaseCellData::new(FaceIJK::new(6, (1, 0, 0)), None), // base cell 45
    BaseCellData::new(FaceIJK::new(11, (0, 0, 0)), None), // base cell 46
    BaseCellData::new(FaceIJK::new(8, (0, 0, 1)), None), // base cell 47
    BaseCellData::new(FaceIJK::new(5, (0, 0, 1)), None), // base cell 48
    BaseCellData::new(FaceIJK::new(14, (2, 0, 0)), Some([0, 9])), // base cell 49
    BaseCellData::new(FaceIJK::new(5, (0, 0, 0)), None), // base cell 50
    BaseCellData::new(FaceIJK::new(12, (1, 0, 0)), None), // base cell 51
    BaseCellData::new(FaceIJK::new(10, (1, 1, 0)), None), // base cell 52
    BaseCellData::new(FaceIJK::new(4, (0, 1, 1)), None), // base cell 53
    BaseCellData::new(FaceIJK::new(12, (1, 1, 0)), None), // base cell 54
    BaseCellData::new(FaceIJK::new(7, (1, 0, 0)), None), // base cell 55
    BaseCellData::new(FaceIJK::new(11, (0, 1, 0)), None), // base cell 56
    BaseCellData::new(FaceIJK::new(10, (0, 0, 0)), None), // base cell 57
    BaseCellData::new(FaceIJK::new(13, (2, 0, 0)), Some([4, 8])), // base cell 58
    BaseCellData::new(FaceIJK::new(10, (0, 0, 1)), None), // base cell 59
    BaseCellData::new(FaceIJK::new(11, (0, 0, 1)), None), // base cell 60
    BaseCellData::new(FaceIJK::new(9, (0, 1, 0)), None), // base cell 61
    BaseCellData::new(FaceIJK::new(8, (0, 1, 0)), None), // base cell 62
    BaseCellData::new(FaceIJK::new(6, (2, 0, 0)), Some([11, 15])), // base cell 63
    BaseCellData::new(FaceIJK::new(8, (0, 0, 0)), None), // base cell 64
    BaseCellData::new(FaceIJK::new(9, (0, 0, 1)), None), // base cell 65
    BaseCellData::new(FaceIJK::new(14, (1, 0, 0)), None), // base cell 66
    BaseCellData::new(FaceIJK::new(5, (1, 0, 1)), None), // base cell 67
    BaseCellData::new(FaceIJK::new(16, (0, 1, 1)), None), // base cell 68
    BaseCellData::new(FaceIJK::new(8, (1, 0, 1)), None), // base cell 69
    BaseCellData::new(FaceIJK::new(5, (1, 0, 0)), None), // base cell 70
    BaseCellData::new(FaceIJK::new(12, (0, 0, 0)), None), // base cell 71
    BaseCellData::new(FaceIJK::new(7, (2, 0, 0)), Some([12, 16])), // base cell 72
    BaseCellData::new(FaceIJK::new(12, (0, 1, 0)), None), // base cell 73
    BaseCellData::new(FaceIJK::new(10, (0, 1, 0)), None), // base cell 74
    BaseCellData::new(FaceIJK::new(9, (0, 0, 0)), None), // base cell 75
    BaseCellData::new(FaceIJK::new(13, (1, 0, 0)), None), // base cell 76
    BaseCellData::new(FaceIJK::new(16, (0, 0, 1)), None), // base cell 77
    BaseCellData::new(FaceIJK::new(15, (0, 1, 1)), None), // base cell 78
    BaseCellData::new(FaceIJK::new(15, (0, 1, 0)), None), // base cell 79
    BaseCellData::new(FaceIJK::new(16, (0, 1, 0)), None), // base cell 80
    BaseCellData::new(FaceIJK::new(14, (1, 1, 0)), None), // base cell 81
    BaseCellData::new(FaceIJK::new(13, (1, 1, 0)), None), // base cell 82
    BaseCellData::new(FaceIJK::new(5, (2, 0, 0)), Some([10, 19])), // base cell 83
    BaseCellData::new(FaceIJK::new(8, (1, 0, 0)), None), // base cell 84
    BaseCellData::new(FaceIJK::new(14, (0, 0, 0)), None), // base cell 85
    BaseCellData::new(FaceIJK::new(9, (1, 0, 1)), None), // base cell 86
    BaseCellData::new(FaceIJK::new(14, (0, 0, 1)), None), // base cell 87
    BaseCellData::new(FaceIJK::new(17, (0, 0, 1)), None), // base cell 88
    BaseCellData::new(FaceIJK::new(12, (0, 0, 1)), None), // base cell 89
    BaseCellData::new(FaceIJK::new(16, (0, 0, 0)), None), // base cell 90
    BaseCellData::new(FaceIJK::new(17, (0, 1, 1)), None), // base cell 91
    BaseCellData::new(FaceIJK::new(15, (0, 0, 1)), None), // base cell 92
    BaseCellData::new(FaceIJK::new(16, (1, 0, 1)), None), // base cell 93
    BaseCellData::new(FaceIJK::new(9, (1, 0, 0)), None), // base cell 94
    BaseCellData::new(FaceIJK::new(15, (0, 0, 0)), None), // base cell 95
    BaseCellData::new(FaceIJK::new(13, (0, 0, 0)), None), // base cell 96
    BaseCellData::new(FaceIJK::new(8, (2, 0, 0)), Some([13, 17])), // base cell 97
    BaseCellData::new(FaceIJK::new(13, (0, 1, 0)), None), // base cell 98
    BaseCellData::new(FaceIJK::new(17, (1, 0, 1)), None), // base cell 99
    BaseCellData::new(FaceIJK::new(19, (0, 1, 0)), None), // base cell 100
    BaseCellData::new(FaceIJK::new(14, (0, 1, 0)), None), // base cell 101
    BaseCellData::new(FaceIJK::new(19, (0, 1, 1)), None), // base cell 102
    BaseCellData::new(FaceIJK::new(17, (0, 1, 0)), None), // base cell 103
    BaseCellData::new(FaceIJK::new(13, (0, 0, 1)), None), // base cell 104
    BaseCellData::new(FaceIJK::new(17, (0, 0, 0)), None), // base cell 105
    BaseCellData::new(FaceIJK::new(16, (1, 0, 0)), None), // base cell 106
    BaseCellData::new(FaceIJK::new(9, (2, 0, 0)), Some([14, 18])), // base cell 107
    BaseCellData::new(FaceIJK::new(15, (1, 0, 1)), None), // base cell 108
    BaseCellData::new(FaceIJK::new(15, (1, 0, 0)), None), // base cell 109
    BaseCellData::new(FaceIJK::new(18, (0, 1, 1)), None), // base cell 110
    BaseCellData::new(FaceIJK::new(18, (0, 0, 1)), None), // base cell 111
    BaseCellData::new(FaceIJK::new(19, (0, 0, 1)), None), // base cell 112
    BaseCellData::new(FaceIJK::new(17, (1, 0, 0)), None), // base cell 113
    BaseCellData::new(FaceIJK::new(19, (0, 0, 0)), None), // base cell 114
    BaseCellData::new(FaceIJK::new(18, (0, 1, 0)), None), // base cell 115
    BaseCellData::new(FaceIJK::new(18, (1, 0, 1)), None), // base cell 116
    BaseCellData::new(FaceIJK::new(19, (2, 0, 0)), Some([-1, -1])), // base cell 117
    BaseCellData::new(FaceIJK::new(19, (1, 0, 0)), None), // base cell 118
    BaseCellData::new(FaceIJK::new(18, (0, 0, 0)), None), // base cell 119
    BaseCellData::new(FaceIJK::new(19, (1, 0, 1)), None), // base cell 120
    BaseCellData::new(FaceIJK::new(18, (1, 0, 0)), None), // base cell 121
];

macro_rules! bc7 {
    [$a:literal, -1, $c:literal, $d:literal, $e:literal, $f:literal, $g:literal] => {
        [ BaseCell($a), BaseCell::INVALID, BaseCell($c), BaseCell($d), BaseCell($e), BaseCell($f), BaseCell($g) ]
    };
    [$a:literal, $b:literal, $c:literal, $d:literal, $e:literal, $f:literal, $g:literal] => {
        [ BaseCell($a), BaseCell($b), BaseCell($c), BaseCell($d), BaseCell($e), BaseCell($f), BaseCell($g) ]
    };
}

/** Neighboring base cell ID in each IJK direction.
 *
 * For each base cell, for each direction, the neighboring base
 * cell ID is given. 127 indicates there is no neighbor in that direction.
 */
pub(crate) const baseCellNeighbors: [[BaseCell; 7]; BaseCell::NUM_BASE_CELLS] = [
    bc7![0, 1, 5, 2, 4, 3, 8],               // base cell 0
    bc7![1, 7, 6, 9, 0, 3, 2],               // base cell 1
    bc7![2, 6, 10, 11, 0, 1, 5],             // base cell 2
    bc7![3, 13, 1, 7, 4, 12, 0],             // base cell 3
    bc7![4, -1, 15, 8, 3, 0, 12],            // base cell 4 pentagon
    bc7![5, 2, 18, 10, 8, 0, 16],            // base cell 5
    bc7![6, 14, 11, 17, 1, 9, 2],            // base cell 6
    bc7![7, 21, 9, 19, 3, 13, 1],            // base cell 7
    bc7![8, 5, 22, 16, 4, 0, 15],            // base cell 8
    bc7![9, 19, 14, 20, 1, 7, 6],            // base cell 9
    bc7![10, 11, 24, 23, 5, 2, 18],          // base cell 10
    bc7![11, 17, 23, 25, 2, 6, 10],          // base cell 11
    bc7![12, 28, 13, 26, 4, 15, 3],          // base cell 12
    bc7![13, 26, 21, 29, 3, 12, 7],          // base cell 13
    bc7![14, -1, 17, 27, 9, 20, 6],          // base cell 14 pentagon
    bc7![15, 22, 28, 31, 4, 8, 12],          // base cell 15
    bc7![16, 18, 33, 30, 8, 5, 22],          // base cell 16
    bc7![17, 11, 14, 6, 35, 25, 27],         // base cell 17
    bc7![18, 24, 30, 32, 5, 10, 16],         // base cell 18
    bc7![19, 34, 20, 36, 7, 21, 9],          // base cell 19
    bc7![20, 14, 19, 9, 40, 27, 36],         // base cell 20
    bc7![21, 38, 19, 34, 13, 29, 7],         // base cell 21
    bc7![22, 16, 41, 33, 15, 8, 31],         // base cell 22
    bc7![23, 24, 11, 10, 39, 37, 25],        // base cell 23
    bc7![24, -1, 32, 37, 10, 23, 18],        // base cell 24 pentagon
    bc7![25, 23, 17, 11, 45, 39, 35],        // base cell 25
    bc7![26, 42, 29, 43, 12, 28, 13],        // base cell 26
    bc7![27, 40, 35, 46, 14, 20, 17],        // base cell 27
    bc7![28, 31, 42, 44, 12, 15, 26],        // base cell 28
    bc7![29, 43, 38, 47, 13, 26, 21],        // base cell 29
    bc7![30, 32, 48, 50, 16, 18, 33],        // base cell 30
    bc7![31, 41, 44, 53, 15, 22, 28],        // base cell 31
    bc7![32, 30, 24, 18, 52, 50, 37],        // base cell 32
    bc7![33, 30, 49, 48, 22, 16, 41],        // base cell 33
    bc7![34, 19, 38, 21, 54, 36, 51],        // base cell 34
    bc7![35, 46, 45, 56, 17, 27, 25],        // base cell 35
    bc7![36, 20, 34, 19, 55, 40, 54],        // base cell 36
    bc7![37, 39, 52, 57, 24, 23, 32],        // base cell 37
    bc7![38, -1, 34, 51, 29, 47, 21],        // base cell 38 pentagon
    bc7![39, 37, 25, 23, 59, 57, 45],        // base cell 39
    bc7![40, 27, 36, 20, 60, 46, 55],        // base cell 40
    bc7![41, 49, 53, 61, 22, 33, 31],        // base cell 41
    bc7![42, 58, 43, 62, 28, 44, 26],        // base cell 42
    bc7![43, 62, 47, 64, 26, 42, 29],        // base cell 43
    bc7![44, 53, 58, 65, 28, 31, 42],        // base cell 44
    bc7![45, 39, 35, 25, 63, 59, 56],        // base cell 45
    bc7![46, 60, 56, 68, 27, 40, 35],        // base cell 46
    bc7![47, 38, 43, 29, 69, 51, 64],        // base cell 47
    bc7![48, 49, 30, 33, 67, 66, 50],        // base cell 48
    bc7![49, -1, 61, 66, 33, 48, 41],        // base cell 49 pentagon
    bc7![50, 48, 32, 30, 70, 67, 52],        // base cell 50
    bc7![51, 69, 54, 71, 38, 47, 34],        // base cell 51
    bc7![52, 57, 70, 74, 32, 37, 50],        // base cell 52
    bc7![53, 61, 65, 75, 31, 41, 44],        // base cell 53
    bc7![54, 71, 55, 73, 34, 51, 36],        // base cell 54
    bc7![55, 40, 54, 36, 72, 60, 73],        // base cell 55
    bc7![56, 68, 63, 77, 35, 46, 45],        // base cell 56
    bc7![57, 59, 74, 78, 37, 39, 52],        // base cell 57
    bc7![58, -1, 62, 76, 44, 65, 42],        // base cell 58 pentagon
    bc7![59, 63, 78, 79, 39, 45, 57],        // base cell 59
    bc7![60, 72, 68, 80, 40, 55, 46],        // base cell 60
    bc7![61, 53, 49, 41, 81, 75, 66],        // base cell 61
    bc7![62, 43, 58, 42, 82, 64, 76],        // base cell 62
    bc7![63, -1, 56, 45, 79, 59, 77],        // base cell 63 pentagon
    bc7![64, 47, 62, 43, 84, 69, 82],        // base cell 64
    bc7![65, 58, 53, 44, 86, 76, 75],        // base cell 65
    bc7![66, 67, 81, 85, 49, 48, 61],        // base cell 66
    bc7![67, 66, 50, 48, 87, 85, 70],        // base cell 67
    bc7![68, 56, 60, 46, 90, 77, 80],        // base cell 68
    bc7![69, 51, 64, 47, 89, 71, 84],        // base cell 69
    bc7![70, 67, 52, 50, 83, 87, 74],        // base cell 70
    bc7![71, 89, 73, 91, 51, 69, 54],        // base cell 71
    bc7![72, -1, 73, 55, 80, 60, 88],        // base cell 72 pentagon
    bc7![73, 91, 72, 88, 54, 71, 55],        // base cell 73
    bc7![74, 78, 83, 92, 52, 57, 70],        // base cell 74
    bc7![75, 65, 61, 53, 94, 86, 81],        // base cell 75
    bc7![76, 86, 82, 96, 58, 65, 62],        // base cell 76
    bc7![77, 63, 68, 56, 93, 79, 90],        // base cell 77
    bc7![78, 74, 59, 57, 95, 92, 79],        // base cell 78
    bc7![79, 78, 63, 59, 93, 95, 77],        // base cell 79
    bc7![80, 68, 72, 60, 99, 90, 88],        // base cell 80
    bc7![81, 85, 94, 101, 61, 66, 75],       // base cell 81
    bc7![82, 96, 84, 98, 62, 76, 64],        // base cell 82
    bc7![83, -1, 74, 70, 100, 87, 92],       // base cell 83 pentagon
    bc7![84, 69, 82, 64, 97, 89, 98],        // base cell 84
    bc7![85, 87, 101, 102, 66, 67, 81],      // base cell 85
    bc7![86, 76, 75, 65, 104, 96, 94],       // base cell 86
    bc7![87, 83, 102, 100, 67, 70, 85],      // base cell 87
    bc7![88, 72, 91, 73, 99, 80, 105],       // base cell 88
    bc7![89, 97, 91, 103, 69, 84, 71],       // base cell 89
    bc7![90, 77, 80, 68, 106, 93, 99],       // base cell 90
    bc7![91, 73, 89, 71, 105, 88, 103],      // base cell 91
    bc7![92, 83, 78, 74, 108, 100, 95],      // base cell 92
    bc7![93, 79, 90, 77, 109, 95, 106],      // base cell 93
    bc7![94, 86, 81, 75, 107, 104, 101],     // base cell 94
    bc7![95, 92, 79, 78, 109, 108, 93],      // base cell 95
    bc7![96, 104, 98, 110, 76, 86, 82],      // base cell 96
    bc7![97, -1, 98, 84, 103, 89, 111],      // base cell 97 pentagon
    bc7![98, 110, 97, 111, 82, 96, 84],      // base cell 98
    bc7![99, 80, 105, 88, 106, 90, 113],     // base cell 99
    bc7![100, 102, 83, 87, 108, 114, 92],    // base cell 100
    bc7![101, 102, 107, 112, 81, 85, 94],    // base cell 101
    bc7![102, 101, 87, 85, 114, 112, 100],   // base cell 102
    bc7![103, 91, 97, 89, 116, 105, 111],    // base cell 103
    bc7![104, 107, 110, 115, 86, 94, 96],    // base cell 104
    bc7![105, 88, 103, 91, 113, 99, 116],    // base cell 105
    bc7![106, 93, 99, 90, 117, 109, 113],    // base cell 106
    bc7![107, -1, 101, 94, 115, 104, 112],   // base cell 107 pentagon
    bc7![108, 100, 95, 92, 118, 114, 109],   // base cell 108
    bc7![109, 108, 93, 95, 117, 118, 106],   // base cell 109
    bc7![110, 98, 104, 96, 119, 111, 115],   // base cell 110
    bc7![111, 97, 110, 98, 116, 103, 119],   // base cell 111
    bc7![112, 107, 102, 101, 120, 115, 114], // base cell 112
    bc7![113, 99, 116, 105, 117, 106, 121],  // base cell 113
    bc7![114, 112, 100, 102, 118, 120, 108], // base cell 114
    bc7![115, 110, 107, 104, 120, 119, 112], // base cell 115
    bc7![116, 103, 119, 111, 113, 105, 121], // base cell 116
    bc7![117, -1, 109, 118, 113, 121, 106],  // base cell 117 pentagon
    bc7![118, 120, 108, 114, 117, 121, 109], // base cell 118
    bc7![119, 111, 115, 110, 121, 116, 120], // base cell 119
    bc7![120, 115, 114, 112, 121, 119, 118], // base cell 120
    bc7![121, 116, 120, 119, 117, 113, 118], // base cell 121
];

/** Neighboring base cell rotations in each IJK direction.
 *
 * For each base cell, for each direction, the number of 60 degree
 * CCW rotations to the coordinate system of the neighbor is given.
 * -1 indicates there is no neighbor in that direction.
 */
pub(crate) const baseCellNeighbor60CCWRots: [[BaseCell; 7]; BaseCell::NUM_BASE_CELLS] = [
    bc7![0, 5, 0, 0, 1, 5, 1],  // base cell 0
    bc7![0, 0, 1, 0, 1, 0, 1],  // base cell 1
    bc7![0, 0, 0, 0, 0, 5, 0],  // base cell 2
    bc7![0, 5, 0, 0, 2, 5, 1],  // base cell 3
    bc7![0, -1, 1, 0, 3, 4, 2], // base cell 4 (pentagon)
    bc7![0, 0, 1, 0, 1, 0, 1],  // base cell 5
    bc7![0, 0, 0, 3, 5, 5, 0],  // base cell 6
    bc7![0, 0, 0, 0, 0, 5, 0],  // base cell 7
    bc7![0, 5, 0, 0, 0, 5, 1],  // base cell 8
    bc7![0, 0, 1, 3, 0, 0, 1],  // base cell 9
    bc7![0, 0, 1, 3, 0, 0, 1],  // base cell 10
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 11
    bc7![0, 5, 0, 0, 3, 5, 1],  // base cell 12
    bc7![0, 0, 1, 0, 1, 0, 1],  // base cell 13
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 14 (pentagon)
    bc7![0, 5, 0, 0, 4, 5, 1],  // base cell 15
    bc7![0, 0, 0, 0, 0, 5, 0],  // base cell 16
    bc7![0, 3, 3, 3, 3, 0, 3],  // base cell 17
    bc7![0, 0, 0, 3, 5, 5, 0],  // base cell 18
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 19
    bc7![0, 3, 3, 3, 0, 3, 0],  // base cell 20
    bc7![0, 0, 0, 3, 5, 5, 0],  // base cell 21
    bc7![0, 0, 1, 0, 1, 0, 1],  // base cell 22
    bc7![0, 3, 3, 3, 0, 3, 0],  // base cell 23
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 24 (pentagon)
    bc7![0, 0, 0, 3, 0, 0, 3],  // base cell 25
    bc7![0, 0, 0, 0, 0, 5, 0],  // base cell 26
    bc7![0, 3, 0, 0, 0, 3, 3],  // base cell 27
    bc7![0, 0, 1, 0, 1, 0, 1],  // base cell 28
    bc7![0, 0, 1, 3, 0, 0, 1],  // base cell 29
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 30
    bc7![0, 0, 0, 0, 0, 5, 0],  // base cell 31
    bc7![0, 3, 3, 3, 3, 0, 3],  // base cell 32
    bc7![0, 0, 1, 3, 0, 0, 1],  // base cell 33
    bc7![0, 3, 3, 3, 3, 0, 3],  // base cell 34
    bc7![0, 0, 3, 0, 3, 0, 3],  // base cell 35
    bc7![0, 0, 0, 3, 0, 0, 3],  // base cell 36
    bc7![0, 3, 0, 0, 0, 3, 3],  // base cell 37
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 38 (pentagon)
    bc7![0, 3, 0, 0, 3, 3, 0],  // base cell 39
    bc7![0, 3, 0, 0, 3, 3, 0],  // base cell 40
    bc7![0, 0, 0, 3, 5, 5, 0],  // base cell 41
    bc7![0, 0, 0, 3, 5, 5, 0],  // base cell 42
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 43
    bc7![0, 0, 1, 3, 0, 0, 1],  // base cell 44
    bc7![0, 0, 3, 0, 0, 3, 3],  // base cell 45
    bc7![0, 0, 0, 3, 0, 3, 0],  // base cell 46
    bc7![0, 3, 3, 3, 0, 3, 0],  // base cell 47
    bc7![0, 3, 3, 3, 0, 3, 0],  // base cell 48
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 49 (pentagon)
    bc7![0, 0, 0, 3, 0, 0, 3],  // base cell 50
    bc7![0, 3, 0, 0, 0, 3, 3],  // base cell 51
    bc7![0, 0, 3, 0, 3, 0, 3],  // base cell 52
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 53
    bc7![0, 0, 3, 0, 3, 0, 3],  // base cell 54
    bc7![0, 0, 3, 0, 0, 3, 3],  // base cell 55
    bc7![0, 3, 3, 3, 0, 0, 3],  // base cell 56
    bc7![0, 0, 0, 3, 0, 3, 0],  // base cell 57
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 58 (pentagon)
    bc7![0, 3, 3, 3, 3, 3, 0],  // base cell 59
    bc7![0, 3, 3, 3, 3, 3, 0],  // base cell 60
    bc7![0, 3, 3, 3, 3, 0, 3],  // base cell 61
    bc7![0, 3, 3, 3, 3, 0, 3],  // base cell 62
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 63 (pentagon)
    bc7![0, 0, 0, 3, 0, 0, 3],  // base cell 64
    bc7![0, 3, 3, 3, 0, 3, 0],  // base cell 65
    bc7![0, 3, 0, 0, 0, 3, 3],  // base cell 66
    bc7![0, 3, 0, 0, 3, 3, 0],  // base cell 67
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 68
    bc7![0, 3, 0, 0, 3, 3, 0],  // base cell 69
    bc7![0, 0, 3, 0, 0, 3, 3],  // base cell 70
    bc7![0, 0, 0, 3, 0, 3, 0],  // base cell 71
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 72 (pentagon)
    bc7![0, 3, 3, 3, 0, 0, 3],  // base cell 73
    bc7![0, 3, 3, 3, 0, 0, 3],  // base cell 74
    bc7![0, 0, 0, 3, 0, 0, 3],  // base cell 75
    bc7![0, 3, 0, 0, 0, 3, 3],  // base cell 76
    bc7![0, 0, 0, 3, 0, 5, 0],  // base cell 77
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 78
    bc7![0, 0, 1, 3, 1, 0, 1],  // base cell 79
    bc7![0, 0, 1, 3, 1, 0, 1],  // base cell 80
    bc7![0, 0, 3, 0, 3, 0, 3],  // base cell 81
    bc7![0, 0, 3, 0, 3, 0, 3],  // base cell 82
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 83 (pentagon)
    bc7![0, 0, 3, 0, 0, 3, 3],  // base cell 84
    bc7![0, 0, 0, 3, 0, 3, 0],  // base cell 85
    bc7![0, 3, 0, 0, 3, 3, 0],  // base cell 86
    bc7![0, 3, 3, 3, 3, 3, 0],  // base cell 87
    bc7![0, 0, 0, 3, 0, 5, 0],  // base cell 88
    bc7![0, 3, 3, 3, 3, 3, 0],  // base cell 89
    bc7![0, 0, 0, 0, 0, 0, 1],  // base cell 90
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 91
    bc7![0, 0, 0, 3, 0, 5, 0],  // base cell 92
    bc7![0, 5, 0, 0, 5, 5, 0],  // base cell 93
    bc7![0, 0, 3, 0, 0, 3, 3],  // base cell 94
    bc7![0, 0, 0, 0, 0, 0, 1],  // base cell 95
    bc7![0, 0, 0, 3, 0, 3, 0],  // base cell 96
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 97 (pentagon)
    bc7![0, 3, 3, 3, 0, 0, 3],  // base cell 98
    bc7![0, 5, 0, 0, 5, 5, 0],  // base cell 99
    bc7![0, 0, 1, 3, 1, 0, 1],  // base cell 100
    bc7![0, 3, 3, 3, 0, 0, 3],  // base cell 101
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 102
    bc7![0, 0, 1, 3, 1, 0, 1],  // base cell 103
    bc7![0, 3, 3, 3, 3, 3, 0],  // base cell 104
    bc7![0, 0, 0, 0, 0, 0, 1],  // base cell 105
    bc7![0, 0, 1, 0, 3, 5, 1],  // base cell 106
    bc7![0, -1, 3, 0, 5, 2, 0], // base cell 107 (pentagon)
    bc7![0, 5, 0, 0, 5, 5, 0],  // base cell 108
    bc7![0, 0, 1, 0, 4, 5, 1],  // base cell 109
    bc7![0, 3, 3, 3, 0, 0, 0],  // base cell 110
    bc7![0, 0, 0, 3, 0, 5, 0],  // base cell 111
    bc7![0, 0, 0, 3, 0, 5, 0],  // base cell 112
    bc7![0, 0, 1, 0, 2, 5, 1],  // base cell 113
    bc7![0, 0, 0, 0, 0, 0, 1],  // base cell 114
    bc7![0, 0, 1, 3, 1, 0, 1],  // base cell 115
    bc7![0, 5, 0, 0, 5, 5, 0],  // base cell 116
    bc7![0, -1, 1, 0, 3, 4, 2], // base cell 117 (pentagon)
    bc7![0, 0, 1, 0, 0, 5, 1],  // base cell 118
    bc7![0, 0, 0, 0, 0, 0, 1],  // base cell 119
    bc7![0, 5, 0, 0, 5, 5, 0],  // base cell 120
    bc7![0, 0, 1, 0, 1, 5, 1],  // base cell 121
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseCellToCCWrot60() {
        // a few random spot-checks
        assert_eq!(
            BaseCell::from(16)._baseCellToCCWrot60(0),
            0,
            "got expected rotation"
        );
        assert_eq!(
            BaseCell::from(32)._baseCellToCCWrot60(0),
            3,
            "got expected rotation"
        );
        assert_eq!(
            BaseCell::from(7)._baseCellToCCWrot60(3),
            1,
            "got expected rotation"
        );
    }

    #[test]
    fn baseCellToCCWrot60_invalid() {
        assert_eq!(
            BaseCell::from(16)._baseCellToCCWrot60(42),
            INVALID_ROTATIONS,
            "should return invalid rotation for invalid face"
        );
        /*
        assert_eq!(
            BaseCell::from(16)._baseCellToCCWrot60(-1),
            INVALID_ROTATIONS,
            "should return invalid rotation for invalid face (negative)"
        );
        */
        assert_eq!(
            BaseCell::from(1)._baseCellToCCWrot60(0),
            INVALID_ROTATIONS,
            "should return invalid rotation for base cell not appearing on face"
        );
    }
}

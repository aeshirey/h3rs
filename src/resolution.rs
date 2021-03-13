#[derive(Copy, Clone, PartialEq)]
pub enum Resolution {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Resolution {
    pub fn areaKm2(&self) -> f64 {
        match self {
            Resolution::R0 => 4250546.848,
            Resolution::R1 => 607220.9782,
            Resolution::R2 => 86745.85403,
            Resolution::R3 => 12392.26486,
            Resolution::R4 => 1770.323552,
            Resolution::R5 => 252.9033645,
            Resolution::R6 => 36.1290521,
            Resolution::R7 => 5.1612932,
            Resolution::R8 => 0.7373276,
            Resolution::R9 => 0.1053325,
            Resolution::R10 => 0.0150475,
            Resolution::R11 => 0.0021496,
            Resolution::R12 => 0.0003071,
            Resolution::R13 => 0.0000439,
            Resolution::R14 => 0.0000063,
            Resolution::R15 => 0.0000009,
        }
    }

    pub fn hexAreaM2(&self) -> f64 {
        match self {
            Resolution::R0 => 4.25055E+12,
            Resolution::R1 => 6.07221E+11,
            Resolution::R2 => 86745854035.,
            Resolution::R3 => 12392264862.,
            Resolution::R4 => 1770323552.,
            Resolution::R5 => 252903364.5,
            Resolution::R6 => 36129052.1,
            Resolution::R7 => 5161293.2,
            Resolution::R8 => 737327.6,
            Resolution::R9 => 105332.5,
            Resolution::R10 => 15047.5,
            Resolution::R11 => 2149.6,
            Resolution::R12 => 307.1,
            Resolution::R13 => 43.9,
            Resolution::R14 => 6.3,
            Resolution::R15 => 0.9,
        }
    }

    pub fn edgeLengthKm(&self) -> f64 {
        match self {
            Resolution::R0 => 1107.712591,
            Resolution::R1 => 418.6760055,
            Resolution::R2 => 158.2446558,
            Resolution::R3 => 59.81085794,
            Resolution::R4 => 22.6063794,
            Resolution::R5 => 8.544408276,
            Resolution::R6 => 3.229482772,
            Resolution::R7 => 1.220629759,
            Resolution::R8 => 0.461354684,
            Resolution::R9 => 0.174375668,
            Resolution::R10 => 0.065907807,
            Resolution::R11 => 0.024910561,
            Resolution::R12 => 0.009415526,
            Resolution::R13 => 0.003559893,
            Resolution::R14 => 0.001348575,
            Resolution::R15 => 0.000509713,
        }
    }

    pub fn edgeLengthM(&self) -> f64 {
        match self {
            Resolution::R0 => 1107712.591,
            Resolution::R1 => 418676.0055,
            Resolution::R2 => 158244.6558,
            Resolution::R3 => 59810.85794,
            Resolution::R4 => 22606.3794,
            Resolution::R5 => 8544.408276,
            Resolution::R6 => 3229.482772,
            Resolution::R7 => 1220.629759,
            Resolution::R8 => 461.3546837,
            Resolution::R9 => 174.3756681,
            Resolution::R10 => 65.90780749,
            Resolution::R11 => 24.9105614,
            Resolution::R12 => 9.415526211,
            Resolution::R13 => 3.559893033,
            Resolution::R14 => 1.348574562,
            Resolution::R15 => 0.509713273,
        }
    }

    pub fn numHexagons(&self) -> usize {
        let n = *self as usize;
        2 + 120 * 7_usize.pow(n as u32)
    }
}

impl From<Resolution> for usize {
    fn from(res: Resolution) -> Self {
        match res {
            Resolution::R0 => 0,
            Resolution::R1 => 1,
            Resolution::R2 => 2,
            Resolution::R3 => 3,
            Resolution::R4 => 4,
            Resolution::R5 => 5,
            Resolution::R6 => 6,
            Resolution::R7 => 7,
            Resolution::R8 => 8,
            Resolution::R9 => 9,
            Resolution::R10 => 10,
            Resolution::R11 => 11,
            Resolution::R12 => 12,
            Resolution::R13 => 13,
            Resolution::R14 => 14,
            Resolution::R15 => 15,
        }
    }
}

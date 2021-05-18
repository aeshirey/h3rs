use crate::{basecell::BaseCell, H3Index};

use super::H3Mode;

impl H3Index {
    /**
     * res0IndexCount returns the number of resolution 0 indexes
     *
     * @return int count of resolution 0 indexes
     */
    pub fn res0IndexCount() -> usize {
        BaseCell::NUM_BASE_CELLS
    }

    /**
     * getRes0Indexes generates all base cells storing them into the provided
     * memory pointer. Buffer must be of size NUM_BASE_CELLS * sizeof(H3Index).
     *
     * @param out H3Index* the memory to store the resulting base cells in
     */
    pub fn getRes0Indexes() -> [H3Index; BaseCell::NUM_BASE_CELLS] {
        let mut result = [H3Index::H3_INIT; BaseCell::NUM_BASE_CELLS];
        for bc in 0..BaseCell::NUM_BASE_CELLS {
            result[bc].set_mode(H3Mode::H3_HEXAGON_MODE);

            let cell = BaseCell::new(bc as i32);
            result[bc].set_base_cell(cell);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn getRes0Indexes() {
        let count = H3Index::res0IndexCount();
        let indexes = H3Index::getRes0Indexes();

        assert_eq!(
            u64::from(indexes[0]),
            0x8001fffffffffff,
            "correct first basecell"
        );
        assert_eq!(
            u64::from(indexes[121]),
            0x80f3fffffffffff,
            "correct last basecell"
        );
    }
}

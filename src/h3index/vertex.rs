use super::H3Index;

/* vertex */
impl H3Index {
    /**
     * Whether the input is a valid H3 vertex
     * @param  vertex H3 index possibly describing a vertex
     * @return        Whether the input is valid
     */
    fn isValidVertex(&self) -> bool {
        if self.H3_GET_MODE() != H3_VERTEX_MODE {
            return false;
        }

        let vertexNum = self.H3_GET_RESERVED_BITS();
        let mut owner: H3Index = vertex;
        owner.H3_SET_MODE(H3_HEXAGON_MODE);
        owner.H3_SET_RESERVED_BITS(0);

        if !owner.h3IsValid() {
            return false;
        }

        // The easiest way to ensure that the owner + vertex number is valid,
        // and that the vertex is canonical, is to recreate and compare.
        let canonical: H3Index = owner.cellToVertex(vertexNum);

        vertex == canonical
    }
}

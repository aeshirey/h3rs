use crate::{geopolygon::GeoBoundary, Direction, H3Index};

use super::H3Mode;

impl H3Index {
    /**
     * Provides the coordinates defining the unidirectional edge.
     * @param edge The unidirectional edge H3Index
     * @param gb The geoboundary object to store the edge coordinates.
     */
    pub fn getH3UnidirectionalEdgeBoundary(&self) -> GeoBoundary {
        // Get the origin and neighbor direction from the edge
        let direction = self.get_reserved_bits();
        let direction = Direction::from(direction as usize);

        let origin = self.getOriginH3IndexFromUnidirectionalEdge();

        /*
           // Get the start vertex for the edge
           int startVertex = vertexNumForDirection(origin, direction);
           if (startVertex == INVALID_VERTEX_NUM) {
               // This is not actually an edge (i.e. no valid direction),
               // so return no vertices.
               gb->numVerts = 0;
               return;
           }

           // Get the geo boundary for the appropriate vertexes of the origin. Note
           // that while there are always 2 topological vertexes per edge, the
           // resulting edge boundary may have an additional distortion vertex if it
           // crosses an edge of the icosahedron.
           FaceIJK fijk;
           _h3ToFaceIjk(origin, &fijk);
           int res = H3_GET_RESOLUTION(origin);
           int isPentagon = H3_EXPORT(h3IsPentagon)(origin);

           if (isPentagon) {
               _faceIjkPentToGeoBoundary(&fijk, res, startVertex, 2, gb);
           } else {
               _faceIjkToGeoBoundary(&fijk, res, startVertex, 2, gb);
           }
        */
        todo!()
    }

    /**
     * Returns the origin hexagon from the unidirectional edge H3Index
     * @param edge The edge H3 index
     * @return The origin H3 hexagon index, or H3_NULL on failure
     */
    pub fn getOriginH3IndexFromUnidirectionalEdge(&self) -> Self {
        if self.get_mode() != H3Mode::H3_UNIEDGE_MODE {
            return Self::H3_NULL;
        }
        let mut origin = *self;
        origin.set_mode(H3Mode::H3_HEXAGON_MODE);
        origin.set_reserved_bits(0);
        origin
    }
}

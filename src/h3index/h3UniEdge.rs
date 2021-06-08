use crate::{geopolygon::GeoBoundary, Direction, H3Index, Resolution};

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

    /**
     * Returns whether or not the provided H3Indexes are neighbors.
     * @param origin The origin H3 index.
     * @param destination The destination H3 index.
     * @return 1 if the indexes are neighbors, 0 otherwise;
     */
    pub fn h3IndexesAreNeighbors(&self, destination: H3Index) -> bool {
        // Make sure they're hexagon indexes
        if self.get_mode() != H3Mode::H3_HEXAGON_MODE
            || destination.get_mode() != H3Mode::H3_HEXAGON_MODE
        {
            return false;
        }

        // Hexagons cannot be neighbors with themselves
        if *self == destination {
            return false;
        }

        // Only hexagons in the same resolution can be neighbors
        let res = self.get_resolution();
        if res != destination.get_resolution() {
            return false;
        }

        // H3 Indexes that share the same parent are very likely to be neighbors
        // Child 0 is neighbor with all of its parent's 'offspring', the other
        // children are neighbors with 3 of the 7 children. So a simple comparison
        // of origin and destination parents and then a lookup table of the children
        // is a super-cheap way to possibly determine they are neighbors.
        if res != Resolution::R0 {
            let parentRes = res - 1;

            let mut origin = *self;
            let mut dest = destination.clone();

            if origin.h3ToParent(parentRes) == dest.h3ToParent(parentRes) {
                let originResDigit = self.get_index_digit(res);
                let destinationResDigit = destination.get_index_digit(res);
                use Direction::*;
                if originResDigit == CENTER_DIGIT || destinationResDigit == CENTER_DIGIT {
                    return true;
                }
                // These sets are the relevant neighbors in the clockwise
                // and counter-clockwise
                const NEIGHBOR_SET_CLOCKWISE: [Direction; 7] = [
                    CENTER_DIGIT,
                    JK_AXES_DIGIT,
                    IJ_AXES_DIGIT,
                    J_AXES_DIGIT,
                    IK_AXES_DIGIT,
                    K_AXES_DIGIT,
                    I_AXES_DIGIT,
                ];
                const NEIGHBOR_SET_COUNTERCLOCKWISE: [Direction; 7] = [
                    CENTER_DIGIT,
                    IK_AXES_DIGIT,
                    JK_AXES_DIGIT,
                    K_AXES_DIGIT,
                    IJ_AXES_DIGIT,
                    I_AXES_DIGIT,
                    J_AXES_DIGIT,
                ];
                if NEIGHBOR_SET_CLOCKWISE[originResDigit as usize] == destinationResDigit
                    || NEIGHBOR_SET_COUNTERCLOCKWISE[originResDigit as usize] == destinationResDigit
                {
                    return true;
                }
            }
        }

        // Otherwise, we have to determine the neighbor relationship the "hard" way.
        todo!();
        /*
        let neighborRing = origin.kRing(1);
        for neighbor in neighborRing {
            if neighborRing == destination {
                return true;
            }
        }
        */
        /*
            H3Index neighborRing[7] = {0};
            H3_EXPORT(kRing)(origin, 1, neighborRing);
            for (int i = 0; i < 7; i++) {
                if (neighborRing[i] == destination) {
                    return true;
                }
            }
        */
        // Made it here, they definitely aren't neighbors
        false
    }

    /**
     * Determines if the provided H3Index is a valid unidirectional edge index
     * @param edge The unidirectional edge H3Index
     * @return 1 if it is a unidirectional edge H3Index, otherwise 0.
     */
    pub fn h3UnidirectionalEdgeIsValid(&self) -> bool {
        if self.get_mode() != H3Mode::H3_UNIEDGE_MODE {
            return false;
        }

        let neighborDirection: Direction = self.get_reserved_bits().into();
        if neighborDirection == Direction::CENTER_DIGIT {
            //if neighborDirection <= Direction::CENTER_DIGIT || neighborDirection >= Direction::NUM_DIGITS {
            return false;
        }

        let origin = self.getOriginH3IndexFromUnidirectionalEdge();
        if origin.is_pentagon() && neighborDirection == Direction::K_AXES_DIGIT {
            return false;
        }

        origin.is_valid() //return H3_EXPORT(h3IsValid)(origin);
    }
}

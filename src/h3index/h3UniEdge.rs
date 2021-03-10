use crate::Direction;

use super::H3Index;

impl H3Index {
    /**
     * Returns whether or not the provided H3Indexes are neighbors.
     * @param origin The origin H3 index.
     * @param destination The destination H3 index.
     * @return 1 if the indexes are neighbors, 0 otherwise;
     */
    fn h3IndexesAreNeighbors(&self, destination: &Self) -> bool {
        // Make sure they're hexagon indexes
        if self.H3_GET_MODE() != H3_HEXAGON_MODE || destination.H3_GET_MODE() != H3_HEXAGON_MODE {
            return false;
        }

        // Hexagons cannot be neighbors with themselves
        if self == destination {
            return false;
        }

        // Only hexagons in the same resolution can be neighbors
        if self.H3_GET_RESOLUTION() != destination.H3_GET_RESOLUTION() {
            return false;
        }

        // H3 Indexes that share the same parent are very likely to be neighbors
        // Child 0 is neighbor with all of its parent's 'offspring', the other
        // children are neighbors with 3 of the 7 children. So a simple comparison
        // of origin and destination parents and then a lookup table of the children
        // is a super-cheap way to possibly determine they are neighbors.
        let parentRes = self.H3_GET_RESOLUTION() - 1;
        if parentRes > 0 && (self.h3ToParent(parentRes) == destination.h3ToParent(parentRes)) {
            let originResDigit: Direction = self.H3_GET_INDEX_DIGIT(parentRes + 1);
            let destinationResDigit: Direction = destination.H3_GET_INDEX_DIGIT(parentRes + 1);
            if originResDigit == CENTER_DIGIT || destinationResDigit == CENTER_DIGIT {
                return true;
            }

            // These sets are the relevant neighbors in the clockwise
            // and counter-clockwise
            const neighborSetClockwise: [Direction; 7] = [
                CENTER_DIGIT,
                JK_AXES_DIGIT,
                IJ_AXES_DIGIT,
                J_AXES_DIGIT,
                IK_AXES_DIGIT,
                K_AXES_DIGIT,
                I_AXES_DIGIT,
            ];
            const neighborSetCounterclockwise: [Direction; 7] = [
                CENTER_DIGIT,
                IK_AXES_DIGIT,
                JK_AXES_DIGIT,
                K_AXES_DIGIT,
                IJ_AXES_DIGIT,
                I_AXES_DIGIT,
                J_AXES_DIGIT,
            ];
            if neighborSetClockwise[originResDigit as usize] == destinationResDigit
                || neighborSetCounterclockwise[originResDigit as usize] == destinationResDigit
            {
                return true;
            }
        }

        // Otherwise, we have to determine the neighbor relationship the "hard" way.
        let neighborRing: [H3Index; 7] = [H3Index::default(); 7];
        kRing(self, 1, neighborRing);
        for i in 0..7 {
            if neighborRing[i] == destination {
                return true;
            }
        }

        // Made it here, they definitely aren't neighbors
        false
    }

    /**
     * Returns a unidirectional edge H3 index based on the provided origin and
     * destination
     * @param origin The origin H3 hexagon index
     * @param destination The destination H3 hexagon index
     * @return The unidirectional edge H3Index, or H3_NULL on failure.
     */
    fn getH3UnidirectionalEdge(&self, destination: &Self) -> Self {
        // Determine the IJK direction from the origin to the destination
        let direction: Direction = self.directionForNeighbor(destination);

        // The direction will be invalid if the cells are not neighbors
        if direction == INVALID_DIGIT {
            return Self::H3_NULL;
        }

        // Create the edge index for the neighbor direction
        let mut output = self.clone();
        output.H3_SET_MODE(H3_UNIEDGE_MODE);
        output.H3_SET_RESERVED_BITS(direction);

        output
    }

    /**
     * Returns the origin hexagon from the unidirectional edge H3Index
     * @param edge The edge H3 index
     * @return The origin H3 hexagon index, or H3_NULL on failure
     */
    fn getOriginH3IndexFromUnidirectionalEdge(&self) -> Self {
        if self.H3_GET_MODE() != H3_UNIEDGE_MODE {
            return Self::H3_NULL;
        }
        let mut origin = self.clone();
        origin.H3_SET_MODE(H3_HEXAGON_MODE);
        origin.H3_SET_RESERVED_BITS(0);
        origin
    }

    /**
     * Returns the destination hexagon from the unidirectional edge H3Index
     * @param edge The edge H3 index
     * @return The destination H3 hexagon index, or H3_NULL on failure
     */
    fn getDestinationH3IndexFromUnidirectionalEdge(&self) -> Self {
        if self.H3_GET_MODE() != H3_UNIEDGE_MODE {
            return Self::H3_NULL;
        }

        let direction: Direction = self.H3_GET_RESERVED_BITS();
        let mut rotations = 0;
        let destination = h3NeighborRotations(
            self.getOriginH3IndexFromUnidirectionalEdge(),
            direction,
            &mut rotations,
        );

        destination
    }

    /**
     * Determines if the provided H3Index is a valid unidirectional edge index
     * @param edge The unidirectional edge H3Index
     * @return 1 if it is a unidirectional edge H3Index, otherwise 0.
     */
    fn h3UnidirectionalEdgeIsValid(&self) -> bool {
        if self.H3_GET_MODE() != H3_UNIEDGE_MODE {
            return false;
        }

        let neighborDirection: Direction = self.H3_GET_RESERVED_BITS();
        if neighborDirection <= CENTER_DIGIT || neighborDirection >= NUM_DIGITS {
            return false;
        }

        let origin = self.getOriginH3IndexFromUnidirectionalEdge();
        if h3IsPentagon(origin) && neighborDirection == K_AXES_DIGIT {
            return false;
        }

        origin.h3IsValid()
    }

    /**
     * Returns the origin, destination pair of hexagon IDs for the given edge ID
     * @param edge The unidirectional edge H3Index
     * @param originDestination Pointer to memory to store origin and destination
     * IDs
     */
    fn getH3IndexesFromUnidirectionalEdge(&self) -> (Self, Self) {
        (
            self.getOriginH3IndexFromUnidirectionalEdge(),
            self.getDestinationH3IndexFromUnidirectionalEdge(),
        )
    }

    /**
     * Provides all of the unidirectional edges from the current H3Index.
     * @param origin The origin hexagon H3Index to find edges for.
     * @param edges The memory to store all of the edges inside.
     */
    pub(crate) fn getH3UnidirectionalEdgesFromHexagon(&self) {
        // Determine if the origin is a pentagon and special treatment needed.
        let isPentagon = self.h3IsPentagon();

        let mut edges: [H3Index; 6] = [Self::H3_Null; 6];

        // This is actually quite simple. Just modify the bits of the origin
        // slightly for each direction, except the 'k' direction in pentagons,
        // which is zeroed.
        for i in 0..6 {
            if isPentagon && i == 0 {
                edges[i] = H3_NULL;
            } else {
                edges[i] = origin;
                edges[i].H3_SET_MODE(H3_UNIEDGE_MODE);
                edges[i].H3_SET_RESERVED_BITS(i + 1);
            }
        }

        edges
    }

    /**
     * Provides the coordinates defining the unidirectional edge.
     * @param edge The unidirectional edge H3Index
     * @param gb The geoboundary object to store the edge coordinates.
     */
    fn getH3UnidirectionalEdgeBoundary(&self) -> GeoBoundary {
        // Get the origin and neighbor direction from the edge
        let direction: Direction = self.H3_GET_RESERVED_BITS();
        let origin = self.getOriginH3IndexFromUnidirectionalEdge();

        // Get the start vertex for the edge
        let startVertex = origin.vertexNumForDirection(direction);
        if startVertex == INVALID_VERTEX_NUM {
            // This is not actually an edge (i.e. no valid direction), so return no vertices.
            return GeoBoundary::default();
        }

        // Get the geo boundary for the appropriate vertexes of the origin. Note
        // that while there are always 2 topological vertexes per edge, the
        // resulting edge boundary may have an additional distortion vertex if it
        // crosses an edge of the icosahedron.
        let fijk: FaceIJK = origin._h3ToFaceIjk();
        let res = origin.H3_GET_RESOLUTION();
        let isPentagon = origin.h3IsPentagon();

        if isPentagon {
            fijk._faceIjkPentToGeoBoundary(res, startVertex, 2)
        } else {
            fijk._faceIjkToGeoBoundary(res, startVertex, 2)
        }
    }
}

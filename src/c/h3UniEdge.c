/*
 * Copyright 2017-2018 Uber Technologies, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *         http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/** @file  h3UniEdge.c
 * @brief H3UniEdge functions for manipulating unidirectional edge indexes.
 */

#include <inttypes.h>
#include <stdbool.h>

#include "algos.h"
#include "constants.h"
#include "coordijk.h"
#include "geoCoord.h"
#include "h3Index.h"
#include "vertex.h"

/**
 * Returns a unidirectional edge H3 index based on the provided origin and
 * destination
 * @param origin The origin H3 hexagon index
 * @param destination The destination H3 hexagon index
 * @return The unidirectional edge H3Index, or H3_NULL on failure.
 */
H3Index H3_EXPORT(getH3UnidirectionalEdge)(H3Index origin,
                                           H3Index destination) {
    // Determine the IJK direction from the origin to the destination
    Direction direction = directionForNeighbor(origin, destination);

    // The direction will be invalid if the cells are not neighbors
    if (direction == INVALID_DIGIT) {
        return H3_NULL;
    }

    // Create the edge index for the neighbor direction
    H3Index output = origin;
    H3_SET_MODE(output, H3_UNIEDGE_MODE);
    H3_SET_RESERVED_BITS(output, direction);

    return output;
}


/**
 * Returns the destination hexagon from the unidirectional edge H3Index
 * @param edge The edge H3 index
 * @return The destination H3 hexagon index, or H3_NULL on failure
 */
H3Index H3_EXPORT(getDestinationH3IndexFromUnidirectionalEdge)(H3Index edge) {
    if (H3_GET_MODE(edge) != H3_UNIEDGE_MODE) {
        return H3_NULL;
    }
    Direction direction = H3_GET_RESERVED_BITS(edge);
    int rotations = 0;
    H3Index destination = h3NeighborRotations(
        H3_EXPORT(getOriginH3IndexFromUnidirectionalEdge)(edge), direction,
        &rotations);
    return destination;
}


/**
 * Returns the origin, destination pair of hexagon IDs for the given edge ID
 * @param edge The unidirectional edge H3Index
 * @param originDestination Pointer to memory to store origin and destination
 * IDs
 */
void H3_EXPORT(getH3IndexesFromUnidirectionalEdge)(H3Index edge,
                                                   H3Index* originDestination) {
    originDestination[0] =
        H3_EXPORT(getOriginH3IndexFromUnidirectionalEdge)(edge);
    originDestination[1] =
        H3_EXPORT(getDestinationH3IndexFromUnidirectionalEdge)(edge);
}

/**
 * Provides all of the unidirectional edges from the current H3Index.
 * @param origin The origin hexagon H3Index to find edges for.
 * @param edges The memory to store all of the edges inside.
 */
void H3_EXPORT(getH3UnidirectionalEdgesFromHexagon)(H3Index origin,
                                                    H3Index* edges) {
    // Determine if the origin is a pentagon and special treatment needed.
    int isPentagon = H3_EXPORT(h3IsPentagon)(origin);

    // This is actually quite simple. Just modify the bits of the origin
    // slightly for each direction, except the 'k' direction in pentagons,
    // which is zeroed.
    for (int i = 0; i < 6; i++) {
        if (isPentagon && i == 0) {
            edges[i] = H3_NULL;
        } else {
            edges[i] = origin;
            H3_SET_MODE(edges[i], H3_UNIEDGE_MODE);
            H3_SET_RESERVED_BITS(edges[i], i + 1);
        }
    }
}


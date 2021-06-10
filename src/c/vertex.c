/*
 * Copyright 2020-2021 Uber Technologies, Inc.
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
/** @file  vertex.h
 *  @brief Functions for working with cell vertexes.
 */

#include "vertex.h"

#include <assert.h>
#include <stdbool.h>

#include "algos.h"
#include "baseCells.h"
#include "faceijk.h"
#include "geoCoord.h"
#include "h3Index.h"

#define DIRECTION_INDEX_OFFSET 2


/**
 * Get the number of CCW rotations of the cell's vertex numbers
 * compared to the directional layout of its neighbors.
 * @return Number of CCW rotations for the cell
 */
static int vertexRotations(H3Index cell) {
    // Get the face and other info for the origin
    FaceIJK fijk;
    _h3ToFaceIjk(cell, &fijk);
    int baseCell = H3_EXPORT(h3GetBaseCell)(cell);
    int cellLeadingDigit = _h3LeadingNonZeroDigit(cell);

    // get the base cell face
    FaceIJK baseFijk;
    _baseCellToFaceIjk(baseCell, &baseFijk);

    int ccwRot60 = _baseCellToCCWrot60(baseCell, fijk.face);

    if (_isBaseCellPentagon(baseCell)) {
        // Find the appropriate direction-to-face mapping
        PentagonDirectionFaces dirFaces;
        // Excluding from branch coverage as we never hit the end condition
        for (int p = 0; p < NUM_PENTAGONS; p++) {  // LCOV_EXCL_BR_LINE
            if (pentagonDirectionFaces[p].baseCell == baseCell) {
                dirFaces = pentagonDirectionFaces[p];
                break;
            }
        }

        // additional CCW rotation for polar neighbors or IK neighbors
        if (fijk.face != baseFijk.face &&
            (_isBaseCellPolarPentagon(baseCell) ||
             fijk.face ==
                 dirFaces.faces[IK_AXES_DIGIT - DIRECTION_INDEX_OFFSET])) {
            ccwRot60 = (ccwRot60 + 1) % 6;
        }

        // Check whether the cell crosses a deleted pentagon subsequence
        if (cellLeadingDigit == JK_AXES_DIGIT &&
            fijk.face ==
                dirFaces.faces[IK_AXES_DIGIT - DIRECTION_INDEX_OFFSET]) {
            // Crosses from JK to IK: Rotate CW
            ccwRot60 = (ccwRot60 + 5) % 6;
        } else if (cellLeadingDigit == IK_AXES_DIGIT &&
                   fijk.face ==
                       dirFaces.faces[JK_AXES_DIGIT - DIRECTION_INDEX_OFFSET]) {
            // Crosses from IK to JK: Rotate CCW
            ccwRot60 = (ccwRot60 + 1) % 6;
        }
    }
    return ccwRot60;
}




/**
 * Get the geocoordinates of an H3 vertex
 * @param vertex H3 index describing a vertex
 * @param coord  Output geo coordinate
 */
void H3_EXPORT(vertexToPoint)(H3Index vertex, GeoCoord* coord) {
    // Get the vertex number and owner from the vertex
    int vertexNum = H3_GET_RESERVED_BITS(vertex);
    H3Index owner = vertex;
    H3_SET_MODE(owner, H3_HEXAGON_MODE);
    H3_SET_RESERVED_BITS(owner, 0);

    // Get the single vertex from the boundary
    GeoBoundary gb;
    FaceIJK fijk;
    _h3ToFaceIjk(owner, &fijk);
    int res = H3_GET_RESOLUTION(owner);

    if (H3_EXPORT(h3IsPentagon)(owner)) {
        _faceIjkPentToGeoBoundary(&fijk, res, vertexNum, 1, &gb);
    } else {
        _faceIjkToGeoBoundary(&fijk, res, vertexNum, 1, &gb);
    }

    // Copy from boundary to output coord
    *coord = gb.verts[0];
}

/**
 * Whether the input is a valid H3 vertex
 * @param  vertex H3 index possibly describing a vertex
 * @return        Whether the input is valid
 */
int H3_EXPORT(isValidVertex)(H3Index vertex) {
    if (H3_GET_MODE(vertex) != H3_VERTEX_MODE) {
        return 0;
    }

    int vertexNum = H3_GET_RESERVED_BITS(vertex);
    H3Index owner = vertex;
    H3_SET_MODE(owner, H3_HEXAGON_MODE);
    H3_SET_RESERVED_BITS(owner, 0);

    if (!H3_EXPORT(h3IsValid)(owner)) {
        return 0;
    }

    // The easiest way to ensure that the owner + vertex number is valid,
    // and that the vertex is canonical, is to recreate and compare.
    H3Index canonical = H3_EXPORT(cellToVertex)(owner, vertexNum);

    return vertex == canonical ? 1 : 0;
}

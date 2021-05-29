/*
 * Copyright 2016-2019 Uber Technologies, Inc.
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
/** @file h3Index.c
 * @brief   H3Index utility functions
 *          (see h3api.h for the main library entry functions)
 */
#include "h3Index.h"

#include <faceijk.h>
#include <inttypes.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "alloc.h"
#include "baseCells.h"
#include "faceijk.h"
#include "mathExtensions.h"

/**
 * Returns the H3 resolution of an H3 index.
 * @param h The H3 index.
 * @return The resolution of the H3 index argument.
 */
int H3_EXPORT(h3GetResolution)(H3Index h) { return H3_GET_RESOLUTION(h); }

/**
 * Returns the H3 base cell "number" of an H3 cell (hexagon or pentagon).
 *
 * Note: Technically works on H3 edges, but will return base cell of the
 * origin cell.
 *
 * @param h The H3 cell.
 * @return The base cell "number" of the H3 cell argument.
 */
int H3_EXPORT(h3GetBaseCell)(H3Index h) { return H3_GET_BASE_CELL(h); }

/**
 * Converts a string representation of an H3 index into an H3 index.
 * @param str The string representation of an H3 index.
 * @return The H3 index corresponding to the string argument, or H3_NULL if
 * invalid.
 */
H3Index H3_EXPORT(stringToH3)(const char* str) {
    H3Index h = H3_NULL;
    // If failed, h will be unmodified and we should return H3_NULL anyways.
    sscanf(str, "%" PRIx64, &h);
    return h;
}

/**
 * Converts an H3 index into a string representation.
 * @param h The H3 index to convert.
 * @param str The string representation of the H3 index.
 * @param sz Size of the buffer `str`
 */
void H3_EXPORT(h3ToString)(H3Index h, char* str, size_t sz) {
    // An unsigned 64 bit integer will be expressed in at most
    // 16 digits plus 1 for the null terminator.
    if (sz < 17) {
        // Buffer is potentially not large enough.
        return;
    }
    sprintf(str, "%" PRIx64, h);
}




/**
 * maxUncompactSize takes a compacted set of hexagons are provides an
 * upper-bound estimate of the size of the uncompacted set of hexagons.
 * @param compactedSet Set of hexagons
 * @param numHexes The number of hexes in the input set
 * @param res The hexagon resolution to decompress to
 * @return The number of hexagons to allocate memory for, or a negative
 * number if an error occurs.
 */
int H3_EXPORT(maxUncompactSize)(const H3Index* compactedSet, const int numHexes,
                                const int res) {
    int maxNumHexagons = 0;
    for (int i = 0; i < numHexes; i++) {
        if (compactedSet[i] == 0) continue;
        int currentRes = H3_GET_RESOLUTION(compactedSet[i]);
        if (!_isValidChildRes(currentRes, res)) {
            // Nonsensical. Abort.
            return -1;
        }
        if (currentRes == res) {
            maxNumHexagons++;
        } else {
            // Bigger hexagon to reduce in size
            int numHexesToGen =
                H3_EXPORT(maxH3ToChildrenSize)(compactedSet[i], res);
            maxNumHexagons += numHexesToGen;
        }
    }
    return maxNumHexagons;
}





/**
 * Find all icosahedron faces intersected by a given H3 index, represented
 * as integers from 0-19. The array is sparse; since 0 is a valid value,
 * invalid array values are represented as -1. It is the responsibility of
 * the caller to filter out invalid values.
 *
 * @param h3 The H3 index
 * @param out Output array. Must be of size maxFaceCount(h3).
 */
void H3_EXPORT(h3GetFaces)(H3Index h3, int* out) {
    int res = H3_GET_RESOLUTION(h3);
    int isPentagon = H3_EXPORT(h3IsPentagon)(h3);

    // We can't use the vertex-based approach here for class II pentagons,
    // because all their vertices are on the icosahedron edges. Their
    // direct child pentagons cross the same faces, so use those instead.
    if (isPentagon && !isResClassIII(res)) {
        // Note that this would not work for res 15, but this is only run on
        // Class II pentagons, it should never be invoked for a res 15 index.
        H3Index childPentagon = makeDirectChild(h3, 0);
        H3_EXPORT(h3GetFaces)(childPentagon, out);
        return;
    }

    // convert to FaceIJK
    FaceIJK fijk;
    _h3ToFaceIjk(h3, &fijk);

    // Get all vertices as FaceIJK addresses. For simplicity, always
    // initialize the array with 6 verts, ignoring the last one for pentagons
    FaceIJK fijkVerts[NUM_HEX_VERTS];
    int vertexCount;

    if (isPentagon) {
        vertexCount = NUM_PENT_VERTS;
        _faceIjkPentToVerts(&fijk, &res, fijkVerts);
    } else {
        vertexCount = NUM_HEX_VERTS;
        _faceIjkToVerts(&fijk, &res, fijkVerts);
    }

    // We may not use all of the slots in the output array,
    // so fill with invalid values to indicate unused slots
    int faceCount = H3_EXPORT(maxFaceCount)(h3);
    for (int i = 0; i < faceCount; i++) {
        out[i] = INVALID_FACE;
    }

    // add each vertex face, using the output array as a hash set
    for (int i = 0; i < vertexCount; i++) {
        FaceIJK* vert = &fijkVerts[i];

        // Adjust overage, determining whether this vertex is
        // on another face
        if (isPentagon) {
            _adjustPentVertOverage(vert, res);
        } else {
            _adjustOverageClassII(vert, res, 0, 1);
        }

        // Save the face to the output array
        int face = vert->face;
        int pos = 0;
        // Find the first empty output position, or the first position
        // matching the current face
        while (out[pos] != INVALID_FACE && out[pos] != face) pos++;
        out[pos] = face;
    }
}


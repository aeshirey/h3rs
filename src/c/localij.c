/*
 * Copyright 2018-2019 Uber Technologies, Inc.
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
/** @file localij.c
 * @brief   Local IJ coordinate space functions
 *
 * These functions try to provide a useful coordinate space in the vicinity of
 * an origin index.
 */
#include <assert.h>
#include <inttypes.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "baseCells.h"
#include "faceijk.h"
#include "h3Index.h"
#include "mathExtensions.h"


/**
 * Produces ijk+ coordinates for an index anchored by an origin.
 *
 * The coordinate space used by this function may have deleted
 * regions or warping due to pentagonal distortion.
 *
 * Coordinates are only comparable if they come from the same
 * origin index.
 *
 * Failure may occur if the index is too far away from the origin
 * or if the index is on the other side of a pentagon.
 *
 * @param origin An anchoring index for the ijk+ coordinate system.
 * @param index Index to find the coordinates of
 * @param out ijk+ coordinates of the index will be placed here on success
 * @return 0 on success, or another value on failure.
 */
int h3ToLocalIjk(H3Index origin, H3Index h3, CoordIJK* out) {
    int res = H3_GET_RESOLUTION(origin);

    if (res != H3_GET_RESOLUTION(h3)) {
        return 1;
    }

    int originBaseCell = H3_GET_BASE_CELL(origin);
    int baseCell = H3_GET_BASE_CELL(h3);

    if (originBaseCell < 0 ||  // LCOV_EXCL_BR_LINE
        originBaseCell >= NUM_BASE_CELLS) {
        // Base cells less than zero can not be represented in an index
        return 1;
    }
    if (baseCell < 0 || baseCell >= NUM_BASE_CELLS) {  // LCOV_EXCL_BR_LINE
        // Base cells less than zero can not be represented in an index
        return 1;
    }

    // Direction from origin base cell to index base cell
    Direction dir = CENTER_DIGIT;
    Direction revDir = CENTER_DIGIT;
    if (originBaseCell != baseCell) {
        dir = _getBaseCellDirection(originBaseCell, baseCell);
        if (dir == INVALID_DIGIT) {
            // Base cells are not neighbors, can't unfold.
            return 2;
        }
        revDir = _getBaseCellDirection(baseCell, originBaseCell);
        assert(revDir != INVALID_DIGIT);
    }

    int originOnPent = _isBaseCellPentagon(originBaseCell);
    int indexOnPent = _isBaseCellPentagon(baseCell);

    FaceIJK indexFijk = {0};
    if (dir != CENTER_DIGIT) {
        // Rotate index into the orientation of the origin base cell.
        // cw because we are undoing the rotation into that base cell.
        int baseCellRotations = baseCellNeighbor60CCWRots[originBaseCell][dir];
        if (indexOnPent) {
            for (int i = 0; i < baseCellRotations; i++) {
                h3 = _h3RotatePent60cw(h3);

                revDir = _rotate60cw(revDir);
                if (revDir == K_AXES_DIGIT) revDir = _rotate60cw(revDir);
            }
        } else {
            for (int i = 0; i < baseCellRotations; i++) {
                h3 = _h3Rotate60cw(h3);

                revDir = _rotate60cw(revDir);
            }
        }
    }
    // Face is unused. This produces coordinates in base cell coordinate space.
    _h3ToFaceIjkWithInitializedFijk(h3, &indexFijk);

    if (dir != CENTER_DIGIT) {
        assert(baseCell != originBaseCell);
        assert(!(originOnPent && indexOnPent));

        int pentagonRotations = 0;
        int directionRotations = 0;

        if (originOnPent) {
            int originLeadingDigit = _h3LeadingNonZeroDigit(origin);

            if (FAILED_DIRECTIONS[originLeadingDigit][dir]) {
                // TODO: We may be unfolding the pentagon incorrectly in this
                // case; return an error code until this is guaranteed to be
                // correct.
                return 3;
            }

            directionRotations = PENTAGON_ROTATIONS[originLeadingDigit][dir];
            pentagonRotations = directionRotations;
        } else if (indexOnPent) {
            int indexLeadingDigit = _h3LeadingNonZeroDigit(h3);

            if (FAILED_DIRECTIONS[indexLeadingDigit][revDir]) {
                // TODO: We may be unfolding the pentagon incorrectly in this
                // case; return an error code until this is guaranteed to be
                // correct.
                return 4;
            }

            pentagonRotations = PENTAGON_ROTATIONS[revDir][indexLeadingDigit];
        }

        assert(pentagonRotations >= 0);
        assert(directionRotations >= 0);

        for (int i = 0; i < pentagonRotations; i++) {
            _ijkRotate60cw(&indexFijk.coord);
        }

        CoordIJK offset = {0};
        _neighbor(&offset, dir);
        // Scale offset based on resolution
        for (int r = res - 1; r >= 0; r--) {
            if (isResClassIII(r + 1)) {
                // rotate ccw
                _downAp7(&offset);
            } else {
                // rotate cw
                _downAp7r(&offset);
            }
        }

        for (int i = 0; i < directionRotations; i++) {
            _ijkRotate60cw(&offset);
        }

        // Perform necessary translation
        _ijkAdd(&indexFijk.coord, &offset, &indexFijk.coord);
        _ijkNormalize(&indexFijk.coord);
    } else if (originOnPent && indexOnPent) {
        // If the origin and index are on pentagon, and we checked that the base
        // cells are the same or neighboring, then they must be the same base
        // cell.
        assert(baseCell == originBaseCell);

        int originLeadingDigit = _h3LeadingNonZeroDigit(origin);
        int indexLeadingDigit = _h3LeadingNonZeroDigit(h3);

        if (FAILED_DIRECTIONS[originLeadingDigit][indexLeadingDigit]) {
            // TODO: We may be unfolding the pentagon incorrectly in this case;
            // return an error code until this is guaranteed to be correct.
            return 5;
        }

        int withinPentagonRotations =
            PENTAGON_ROTATIONS[originLeadingDigit][indexLeadingDigit];

        for (int i = 0; i < withinPentagonRotations; i++) {
            _ijkRotate60cw(&indexFijk.coord);
        }
    }

    *out = indexFijk.coord;
    return 0;
}


/**
 * Produces ij coordinates for an index anchored by an origin.
 *
 * The coordinate space used by this function may have deleted
 * regions or warping due to pentagonal distortion.
 *
 * Coordinates are only comparable if they come from the same
 * origin index.
 *
 * Failure may occur if the index is too far away from the origin
 * or if the index is on the other side of a pentagon.
 *
 * This function is experimental, and its output is not guaranteed
 * to be compatible across different versions of H3.
 *
 * @param origin An anchoring index for the ij coordinate system.
 * @param index Index to find the coordinates of
 * @param out ij coordinates of the index will be placed here on success
 * @return 0 on success, or another value on failure.
 */
int H3_EXPORT(experimentalH3ToLocalIj)(H3Index origin, H3Index h3,
                                       CoordIJ* out) {
    // This function is currently experimental. Once ready to be part of the
    // non-experimental API, this function (with the experimental prefix) will
    // be marked as deprecated and to be removed in the next major version. It
    // will be replaced with a non-prefixed function name.
    CoordIJK ijk;
    int failed = h3ToLocalIjk(origin, h3, &ijk);
    if (failed) {
        return failed;
    }

    ijkToIj(&ijk, out);

    return 0;
}



/**
 * Given two H3 indexes, return the line of indexes between them (inclusive).
 *
 * This function may fail to find the line between two indexes, for
 * example if they are very far apart. It may also fail when finding
 * distances for indexes on opposite sides of a pentagon.
 *
 * Notes:
 *
 *  - The specific output of this function should not be considered stable
 *    across library versions. The only guarantees the library provides are
 *    that the line length will be `h3Distance(start, end) + 1` and that
 *    every index in the line will be a neighbor of the preceding index.
 *  - Lines are drawn in grid space, and may not correspond exactly to either
 *    Cartesian lines or great arcs.
 *
 * @param start Start index of the line
 * @param end End index of the line
 * @param out Output array, which must be of size h3LineSize(start, end)
 * @return 0 on success, or another value on failure.
 */
int H3_EXPORT(h3Line)(H3Index start, H3Index end, H3Index* out) {
    int distance = H3_EXPORT(h3Distance)(start, end);
    // Early exit if we can't calculate the line
    if (distance < 0) {
        return distance;
    }

    // Get IJK coords for the start and end. We've already confirmed
    // that these can be calculated with the distance check above.
    CoordIJK startIjk = {0};
    CoordIJK endIjk = {0};

    // Convert H3 addresses to IJK coords
    h3ToLocalIjk(start, start, &startIjk);
    h3ToLocalIjk(start, end, &endIjk);

    // Convert IJK to cube coordinates suitable for linear interpolation
    ijkToCube(&startIjk);
    ijkToCube(&endIjk);

    double iStep =
        distance ? (double)(endIjk.i - startIjk.i) / (double)distance : 0;
    double jStep =
        distance ? (double)(endIjk.j - startIjk.j) / (double)distance : 0;
    double kStep =
        distance ? (double)(endIjk.k - startIjk.k) / (double)distance : 0;

    CoordIJK currentIjk = {startIjk.i, startIjk.j, startIjk.k};
    for (int n = 0; n <= distance; n++) {
        cubeRound((double)startIjk.i + iStep * n,
                  (double)startIjk.j + jStep * n,
                  (double)startIjk.k + kStep * n, &currentIjk);
        // Convert cube -> ijk -> h3 index
        cubeToIjk(&currentIjk);
        localIjkToH3(start, &currentIjk, &out[n]);
    }

    return 0;
}

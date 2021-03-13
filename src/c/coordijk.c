/*
 * Copyright 2016-2018 Uber Technologies, Inc.
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
/** @file coordijk.c
 * @brief   Hex IJK coordinate systems functions including conversions to/from
 * lat/lon.
 */

#include "coordijk.h"

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "constants.h"
#include "geoCoord.h"
#include "mathExtensions.h"

/**
 * Sets an IJK coordinate to the specified component values.
 *
 * @param ijk The IJK coordinate to set.
 * @param i The desired i component value.
 * @param j The desired j component value.
 * @param k The desired k component value.
 */
void _setIJK(CoordIJK* ijk, int i, int j, int k) {
    ijk->i = i;
    ijk->j = j;
    ijk->k = k;
}







/**
 * Rotates ijk coordinates 60 degrees counter-clockwise. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
void _ijkRotate60ccw(CoordIJK* ijk) {
    // unit vector rotations
    CoordIJK iVec = {1, 1, 0};
    CoordIJK jVec = {0, 1, 1};
    CoordIJK kVec = {1, 0, 1};

    _ijkScale(&iVec, ijk->i);
    _ijkScale(&jVec, ijk->j);
    _ijkScale(&kVec, ijk->k);

    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Rotates ijk coordinates 60 degrees clockwise. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
void _ijkRotate60cw(CoordIJK* ijk) {
    // unit vector rotations
    CoordIJK iVec = {1, 0, 1};
    CoordIJK jVec = {1, 1, 0};
    CoordIJK kVec = {0, 1, 1};

    _ijkScale(&iVec, ijk->i);
    _ijkScale(&jVec, ijk->j);
    _ijkScale(&kVec, ijk->k);

    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);

    _ijkNormalize(ijk);
}



/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 3 counter-clockwise resolution. Works in
 * place.
 *
 * @param ijk The ijk coordinates.
 */
void _downAp3(CoordIJK* ijk) {
    // res r unit vectors in res r+1
    CoordIJK iVec = {2, 0, 1};
    CoordIJK jVec = {1, 2, 0};
    CoordIJK kVec = {0, 1, 2};

    _ijkScale(&iVec, ijk->i);
    _ijkScale(&jVec, ijk->j);
    _ijkScale(&kVec, ijk->k);

    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Find the normalized ijk coordinates of the hex centered on the indicated
 * hex at the next finer aperture 3 clockwise resolution. Works in place.
 *
 * @param ijk The ijk coordinates.
 */
void _downAp3r(CoordIJK* ijk) {
    // res r unit vectors in res r+1
    CoordIJK iVec = {2, 1, 0};
    CoordIJK jVec = {0, 2, 1};
    CoordIJK kVec = {1, 0, 2};

    _ijkScale(&iVec, ijk->i);
    _ijkScale(&jVec, ijk->j);
    _ijkScale(&kVec, ijk->k);

    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);

    _ijkNormalize(ijk);
}

/**
 * Finds the distance between the two coordinates. Returns result.
 *
 * @param c1 The first set of ijk coordinates.
 * @param c2 The second set of ijk coordinates.
 */
int ijkDistance(const CoordIJK* c1, const CoordIJK* c2) {
    CoordIJK diff;
    _ijkSub(c1, c2, &diff);
    _ijkNormalize(&diff);
    CoordIJK absDiff = {abs(diff.i), abs(diff.j), abs(diff.k)};
    return MAX(absDiff.i, MAX(absDiff.j, absDiff.k));
}

/**
 * Transforms coordinates from the IJK+ coordinate system to the IJ coordinate
 * system.
 *
 * @param ijk The input IJK+ coordinates
 * @param ij The output IJ coordinates
 */
void ijkToIj(const CoordIJK* ijk, CoordIJ* ij) {
    ij->i = ijk->i - ijk->k;
    ij->j = ijk->j - ijk->k;
}

/**
 * Transforms coordinates from the IJ coordinate system to the IJK+ coordinate
 * system.
 *
 * @param ij The input IJ coordinates
 * @param ijk The output IJK+ coordinates
 */
void ijToIjk(const CoordIJ* ij, CoordIJK* ijk) {
    ijk->i = ij->i;
    ijk->j = ij->j;
    ijk->k = 0;

    _ijkNormalize(ijk);
}

/**
 * Convert IJK coordinates to cube coordinates, in place
 * @param ijk Coordinate to convert
 */
void ijkToCube(CoordIJK* ijk) {
    ijk->i = -ijk->i + ijk->k;
    ijk->j = ijk->j - ijk->k;
    ijk->k = -ijk->i - ijk->j;
}

/**
 * Convert cube coordinates to IJK coordinates, in place
 * @param ijk Coordinate to convert
 */
void cubeToIjk(CoordIJK* ijk) {
    ijk->i = -ijk->i;
    ijk->k = 0;
    _ijkNormalize(ijk);
}

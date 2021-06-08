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


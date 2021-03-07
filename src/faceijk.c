/*
 * Copyright 2016-2020 Uber Technologies, Inc.
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
/** @file faceijk.c
 * @brief   Functions for working with icosahedral face-centered hex IJK
 *  coordinate systems.
 */

#include "faceijk.h"

#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "constants.h"
#include "coordijk.h"
#include "geoCoord.h"
#include "h3Index.h"
#include "vec3d.h"



/**
 * Get the vertices of a cell as substrate FaceIJK addresses
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell. This may be adjusted if
 *            necessary for the substrate grid resolution.
 * @param fijkVerts Output array for the vertices
 */
void _faceIjkToVerts(FaceIJK* fijk, int* res, FaceIJK* fijkVerts) {
    // the vertexes of an origin-centered cell in a Class II resolution on a
    // substrate grid with aperture sequence 33r. The aperture 3 gets us the
    // vertices, and the 3r gets us back to Class II.
    // vertices listed ccw from the i-axes
    CoordIJK vertsCII[NUM_HEX_VERTS] = {
        {2, 1, 0},  // 0
        {1, 2, 0},  // 1
        {0, 2, 1},  // 2
        {0, 1, 2},  // 3
        {1, 0, 2},  // 4
        {2, 0, 1}   // 5
    };

    // the vertexes of an origin-centered cell in a Class III resolution on a
    // substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
    // vertices, and the 3r7r gets us to Class II.
    // vertices listed ccw from the i-axes
    CoordIJK vertsCIII[NUM_HEX_VERTS] = {
        {5, 4, 0},  // 0
        {1, 5, 0},  // 1
        {0, 5, 4},  // 2
        {0, 1, 5},  // 3
        {4, 0, 5},  // 4
        {5, 0, 1}   // 5
    };

    // get the correct set of substrate vertices for this resolution
    CoordIJK* verts;
    if (isResClassIII(*res))
        verts = vertsCIII;
    else
        verts = vertsCII;

    // adjust the center point to be in an aperture 33r substrate grid
    // these should be composed for speed
    _downAp3(&fijk->coord);
    _downAp3r(&fijk->coord);

    // if res is Class III we need to add a cw aperture 7 to get to
    // icosahedral Class II
    if (isResClassIII(*res)) {
        _downAp7r(&fijk->coord);
        *res += 1;
    }

    // The center point is now in the same substrate grid as the origin
    // cell vertices. Add the center point substate coordinates
    // to each vertex to translate the vertices to that cell.
    for (int v = 0; v < NUM_HEX_VERTS; v++) {
        fijkVerts[v].face = fijk->face;
        _ijkAdd(&fijk->coord, &verts[v], &fijkVerts[v].coord);
        _ijkNormalize(&fijkVerts[v].coord);
    }
}


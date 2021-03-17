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

/** square root of 7 */
#define M_SQRT7 2.6457513110645905905016157536392604257102L

/**
 * Encodes a coordinate on the sphere to the FaceIJK address of the containing
 * cell at the specified resolution.
 *
 * @param g The spherical coordinates to encode.
 * @param res The desired H3 resolution for the encoding.
 * @param h The FaceIJK address of the containing cell at resolution res.
 */
void _geoToFaceIjk(const GeoCoord* g, int res, FaceIJK* h) {
    // first convert to hex2d
    Vec2d v;
    _geoToHex2d(g, res, &h->face, &v);

    // then convert to ijk+
    _hex2dToCoordIJK(&v, &h->coord);
}

/**
 * Encodes a coordinate on the sphere to the corresponding icosahedral face and
 * containing 2D hex coordinates relative to that face center.
 *
 * @param g The spherical coordinates to encode.
 * @param res The desired H3 resolution for the encoding.
 * @param face The icosahedral face containing the spherical coordinates.
 * @param v The 2D hex coordinates of the cell containing the point.
 */
void _geoToHex2d(const GeoCoord* g, int res, int* face, Vec2d* v) {
    Vec3d v3d;
    _geoToVec3d(g, &v3d);

    // determine the icosahedron face
    *face = 0;
    double sqd = _pointSquareDist(&faceCenterPoint[0], &v3d);
    for (int f = 1; f < NUM_ICOSA_FACES; f++) {
        double sqdT = _pointSquareDist(&faceCenterPoint[f], &v3d);
        if (sqdT < sqd) {
            *face = f;
            sqd = sqdT;
        }
    }

    // cos(r) = 1 - 2 * sin^2(r/2) = 1 - 2 * (sqd / 4) = 1 - sqd/2
    double r = acos(1 - sqd / 2);

    if (r < EPSILON) {
        v->x = v->y = 0.0L;
        return;
    }

    // now have face and r, now find CCW theta from CII i-axis
    double theta =
        _posAngleRads(faceAxesAzRadsCII[*face][0] -
                      _posAngleRads(_geoAzimuthRads(&faceCenterGeo[*face], g)));

    // adjust theta for Class III (odd resolutions)
    if (isResClassIII(res)) theta = _posAngleRads(theta - M_AP7_ROT_RADS);

    // perform gnomonic scaling of r
    r = tan(r);

    // scale for current resolution length u
    r /= RES0_U_GNOMONIC;
    for (int i = 0; i < res; i++) r *= M_SQRT7;

    // we now have (r, theta) in hex2d with theta ccw from x-axes

    // convert to local x,y
    v->x = r * cos(theta);
    v->y = r * sin(theta);
}




/**
 * Get the vertices of a pentagon cell as substrate FaceIJK addresses
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell. This may be adjusted if
 *            necessary for the substrate grid resolution.
 * @param fijkVerts Output array for the vertices
 */
void _faceIjkPentToVerts(FaceIJK* fijk, int* res, FaceIJK* fijkVerts) {
    // the vertexes of an origin-centered pentagon in a Class II resolution on a
    // substrate grid with aperture sequence 33r. The aperture 3 gets us the
    // vertices, and the 3r gets us back to Class II.
    // vertices listed ccw from the i-axes
    CoordIJK vertsCII[NUM_PENT_VERTS] = {
        {2, 1, 0},  // 0
        {1, 2, 0},  // 1
        {0, 2, 1},  // 2
        {0, 1, 2},  // 3
        {1, 0, 2},  // 4
    };

    // the vertexes of an origin-centered pentagon in a Class III resolution on
    // a substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
    // vertices, and the 3r7r gets us to Class II. vertices listed ccw from the
    // i-axes
    CoordIJK vertsCIII[NUM_PENT_VERTS] = {
        {5, 4, 0},  // 0
        {1, 5, 0},  // 1
        {0, 5, 4},  // 2
        {0, 1, 5},  // 3
        {4, 0, 5},  // 4
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
    for (int v = 0; v < NUM_PENT_VERTS; v++) {
        fijkVerts[v].face = fijk->face;
        _ijkAdd(&fijk->coord, &verts[v], &fijkVerts[v].coord);
        _ijkNormalize(&fijkVerts[v].coord);
    }
}


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

/**
 * Adjusts a FaceIJK address in place so that the resulting cell address is
 * relative to the correct icosahedral face.
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 * @param pentLeading4 Whether or not the cell is a pentagon with a leading
 *        digit 4.
 * @param substrate Whether or not the cell is in a substrate grid.
 * @return 0 if on original face (no overage); 1 if on face edge (only occurs
 *         on substrate grids); 2 if overage on new face interior
 */
Overage _adjustOverageClassII(FaceIJK* fijk, int res, int pentLeading4,
                              int substrate) {
    Overage overage = NO_OVERAGE;

    CoordIJK* ijk = &fijk->coord;

    // get the maximum dimension value; scale if a substrate grid
    int maxDim = maxDimByCIIres[res];
    if (substrate) maxDim *= 3;

    // check for overage
    if (substrate && ijk->i + ijk->j + ijk->k == maxDim)  // on edge
        overage = FACE_EDGE;
    else if (ijk->i + ijk->j + ijk->k > maxDim)  // overage
    {
        overage = NEW_FACE;

        const FaceOrientIJK* fijkOrient;
        if (ijk->k > 0) {
            if (ijk->j > 0)  // jk "quadrant"
                fijkOrient = &faceNeighbors[fijk->face][JK];
            else  // ik "quadrant"
            {
                fijkOrient = &faceNeighbors[fijk->face][KI];

                // adjust for the pentagonal missing sequence
                if (pentLeading4) {
                    // translate origin to center of pentagon
                    CoordIJK origin;
                    _setIJK(&origin, maxDim, 0, 0);
                    CoordIJK tmp;
                    _ijkSub(ijk, &origin, &tmp);
                    // rotate to adjust for the missing sequence
                    _ijkRotate60cw(&tmp);
                    // translate the origin back to the center of the triangle
                    _ijkAdd(&tmp, &origin, ijk);
                }
            }
        } else  // ij "quadrant"
            fijkOrient = &faceNeighbors[fijk->face][IJ];

        fijk->face = fijkOrient->face;

        // rotate and translate for adjacent face
        for (int i = 0; i < fijkOrient->ccwRot60; i++) _ijkRotate60ccw(ijk);

        CoordIJK transVec = fijkOrient->translate;
        int unitScale = unitScaleByCIIres[res];
        if (substrate) unitScale *= 3;
        _ijkScale(&transVec, unitScale);
        _ijkAdd(ijk, &transVec, ijk);
        _ijkNormalize(ijk);

        // overage points on pentagon boundaries can end up on edges
        if (substrate && ijk->i + ijk->j + ijk->k == maxDim)  // on edge
            overage = FACE_EDGE;
    }

    return overage;
}

/**
 * Adjusts a FaceIJK address for a pentagon vertex in a substrate grid in
 * place so that the resulting cell address is relative to the correct
 * icosahedral face.
 *
 * @param fijk The FaceIJK address of the cell.
 * @param res The H3 resolution of the cell.
 */
Overage _adjustPentVertOverage(FaceIJK* fijk, int res) {
    int pentLeading4 = 0;
    Overage overage;
    do {
        overage = _adjustOverageClassII(fijk, res, pentLeading4, 1);
    } while (overage == NEW_FACE);
    return overage;
}

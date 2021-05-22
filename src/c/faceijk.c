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
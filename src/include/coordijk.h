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
/** @file coordijk.h
 * @brief   Header file for CoordIJK functions including conversion from lat/lon
 *
 * References two Vec2d cartesian coordinate systems:
 *
 *    1. gnomonic: face-centered polyhedral gnomonic projection space with
 *             traditional scaling and x-axes aligned with the face Class II
 *             i-axes.
 *
 *    2. hex2d: local face-centered coordinate system scaled a specific H3 grid
 *             resolution unit length and with x-axes aligned with the local
 *             i-axes
 */

#ifndef COORDIJK_H
#define COORDIJK_H

#include "geoCoord.h"
#include "h3api.h"
#include "vec2d.h"


// Internal functions

void _setIJK(CoordIJK* ijk, int i, int j, int k);
void _hex2dToCoordIJK(const Vec2d* v, CoordIJK* h);
void _ijkToHex2d(const CoordIJK* h, Vec2d* v);
int _ijkMatches(const CoordIJK* c1, const CoordIJK* c2);
void _ijkAdd(const CoordIJK* h1, const CoordIJK* h2, CoordIJK* sum);
void _ijkSub(const CoordIJK* h1, const CoordIJK* h2, CoordIJK* diff);
void _ijkScale(CoordIJK* c, int factor);
void _ijkNormalize(CoordIJK* c);
Direction _unitIjkToDigit(const CoordIJK* ijk);
void _upAp7(CoordIJK* ijk);
void _upAp7r(CoordIJK* ijk);
void _downAp7(CoordIJK* ijk);
void _downAp7r(CoordIJK* ijk);
void _downAp3(CoordIJK* ijk);
void _downAp3r(CoordIJK* ijk);
void _neighbor(CoordIJK* ijk, Direction digit);
void _ijkRotate60ccw(CoordIJK* ijk);
void _ijkRotate60cw(CoordIJK* ijk);
Direction _rotate60ccw(Direction digit);
Direction _rotate60cw(Direction digit);
int ijkDistance(const CoordIJK* a, const CoordIJK* b);
void ijkToIj(const CoordIJK* ijk, CoordIJ* ij);
void ijToIjk(const CoordIJ* ij, CoordIJK* ijk);
void ijkToCube(CoordIJK* ijk);
void cubeToIjk(CoordIJK* ijk);

#endif

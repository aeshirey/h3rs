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
/** @file h3Index.h
 * @brief   H3Index functions.
 */

#ifndef H3INDEX_H
#define H3INDEX_H

#include "faceijk.h"
#include "h3api.h"






/*
 * Return codes for compact
 */

#define COMPACT_SUCCESS 0
#define COMPACT_LOOP_EXCEEDED -1
#define COMPACT_DUPLICATE -2
#define COMPACT_ALLOC_FAILED -3

void setH3Index(H3Index* h, int res, int baseCell, Direction initDigit);
int isResClassIII(int res);

// Internal functions

int _h3ToFaceIjkWithInitializedFijk(H3Index h, FaceIJK* fijk);
void _h3ToFaceIjk(H3Index h, FaceIJK* fijk);
H3Index _faceIjkToH3(const FaceIJK* fijk, int res);
Direction _h3LeadingNonZeroDigit(H3Index h);
H3Index _h3RotatePent60ccw(H3Index h);
H3Index _h3RotatePent60cw(H3Index h);
H3Index _h3Rotate60ccw(H3Index h);
H3Index _h3Rotate60cw(H3Index h);

#endif

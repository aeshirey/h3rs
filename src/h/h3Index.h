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


/**
 * Sets the highest bit of the h3 to v.
 */
#define H3_SET_HIGH_BIT(h3, v)                 \
    (h3) = (((h3)&H3_HIGH_BIT_MASK_NEGATIVE) | \
            (((uint64_t)(v)) << H3_MAX_OFFSET))


/**
 * Gets the resolution res integer digit (0-7) of h3.
 */
#define H3_GET_INDEX_DIGIT(h3, res)                                        \
    ((Direction)((((h3) >> ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)) & \
                  H3_DIGIT_MASK)))

/**
 * Sets a value in the reserved space. Setting to non-zero may produce invalid
 * indexes.
 */
#define H3_SET_RESERVED_BITS(h3, v)            \
    (h3) = (((h3)&H3_RESERVED_MASK_NEGATIVE) | \
            (((uint64_t)(v)) << H3_RESERVED_OFFSET))

/**
 * Gets a value in the reserved space. Should always be zero for valid indexes.
 */
#define H3_GET_RESERVED_BITS(h3) \
    ((int)((((h3)&H3_RESERVED_MASK) >> H3_RESERVED_OFFSET)))

/**
 * Sets the resolution res digit of h3 to the integer digit (0-7)
 */
#define H3_SET_INDEX_DIGIT(h3, res, digit)                                  \
    (h3) = (((h3) & ~((H3_DIGIT_MASK                                        \
                       << ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)))) | \
            (((uint64_t)(digit))                                            \
             << ((MAX_H3_RES - (res)) * H3_PER_DIGIT_OFFSET)))

/**
 * Invalid index used to indicate an error from geoToH3 and related functions
 * or missing data in arrays of h3 indices. Analogous to NaN in floating point.
 */
#define H3_NULL 0

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

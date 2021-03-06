/*
 * Copyright 2017-2019 Uber Technologies, Inc.
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

#include <stdlib.h>

#include "h3Index.h"
#include "test.h"

#define PADDED_COUNT 10


SUITE(h3ToChildren) {
    GeoCoord sf = {0.659966917655, 2 * 3.14159 - 2.1364398519396};
    H3Index sfHex8 = H3_EXPORT(geoToH3)(&sf, 8);

    TEST(oneResStep) {
        const int expectedCount = 7;
        const int paddedCount = 10;

        H3Index sfHex9s[PADDED_COUNT] = {0};
        H3_EXPORT(h3ToChildren)(sfHex8, 9, sfHex9s);

        GeoCoord center;
        H3_EXPORT(h3ToGeo)(sfHex8, &center);
        H3Index sfHex9_0 = H3_EXPORT(geoToH3)(&center, 9);

        int numFound = 0;

        // Find the center
        for (int i = 0; i < paddedCount; i++) {
            if (sfHex9_0 == sfHex9s[i]) {
                numFound++;
            }
        }
        t_assert(numFound == 1, "found the center hex");

        // Get the neighbor hexagons by averaging the center point and outer
        // points then query for those independently

        GeoBoundary outside;
        H3_EXPORT(h3ToGeoBoundary)(sfHex8, &outside);
        for (int i = 0; i < outside.numVerts; i++) {
            GeoCoord avg = {0};
            avg.lat = (outside.verts[i].lat + center.lat) / 2;
            avg.lon = (outside.verts[i].lon + center.lon) / 2;
            H3Index avgHex9 = H3_EXPORT(geoToH3)(&avg, 9);
            for (int j = 0; j < expectedCount; j++) {
                if (avgHex9 == sfHex9s[j]) {
                    numFound++;
                }
            }
        }

        t_assert(numFound == expectedCount, "found all expected children");
    }



    TEST(pentagonChildren) {
        H3Index pentagon;
        setH3Index(&pentagon, 1, 4, 0);

        const int expectedCount = (5 * 7) + 6;
        const int paddedCount = H3_EXPORT(maxH3ToChildrenSize)(pentagon, 3);

        H3Index* children = calloc(paddedCount, sizeof(H3Index));
        H3_EXPORT(h3ToChildren)(sfHex8, 10, children);
        H3_EXPORT(h3ToChildren)(pentagon, 3, children);

        verifyCountAndUniqueness(children, paddedCount, expectedCount);
        free(children);
    }
}

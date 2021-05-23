
SUITE(h3ToLocalIj) {
	// working on this
    TEST(experimentalH3ToLocalIjInvalid) {
        CoordIJ ij;
        H3Index invalidIndex = 0x7fffffffffffffff;
        H3_SET_RESOLUTION(invalidIndex, H3_GET_RESOLUTION(bc1));
        t_assert(
            H3_EXPORT(experimentalH3ToLocalIj)(bc1, invalidIndex, &ij) != 0,
            "invalid index");
        t_assert(H3_EXPORT(experimentalH3ToLocalIj)(0x7fffffffffffffff, bc1,
                                                    &ij) != 0,
                 "invalid origin");
        t_assert(H3_EXPORT(experimentalH3ToLocalIj)(
                     0x7fffffffffffffff, 0x7fffffffffffffff, &ij) != 0,
                 "invalid origin and index");
    }

    TEST(experimentalLocalIjToH3Invalid) {
        CoordIJ ij = {0, 0};
        H3Index index;
        t_assert(H3_EXPORT(experimentalLocalIjToH3)(0x7fffffffffffffff, &ij,
                                                    &index) != 0,
                 "invalid origin for ijToH3");
    }

    /**
     * Test that coming from the same direction outside the pentagon is handled
     * the same as coming from the same direction inside the pentagon.
     */
    TEST(onOffPentagonSame) {
        for (int bc = 0; bc < NUM_BASE_CELLS; bc++) {
            for (int res = 1; res <= MAX_H3_RES; res++) {
                // K_AXES_DIGIT is the first internal direction, and it's also
                // invalid for pentagons, so skip to next.
                Direction startDir = K_AXES_DIGIT;
                if (_isBaseCellPentagon(bc)) {
                    startDir++;
                }

                for (Direction dir = startDir; dir < NUM_DIGITS; dir++) {
                    H3Index internalOrigin;
                    setH3Index(&internalOrigin, res, bc, dir);

                    H3Index externalOrigin;
                    setH3Index(&externalOrigin, res,
                               _getBaseCellNeighbor(bc, dir), CENTER_DIGIT);

                    for (Direction testDir = startDir; testDir < NUM_DIGITS;
                         testDir++) {
                        H3Index testIndex;
                        setH3Index(&testIndex, res, bc, testDir);

                        CoordIJ internalIj;
                        int internalIjFailed =
                            H3_EXPORT(experimentalH3ToLocalIj)(
                                internalOrigin, testIndex, &internalIj);
                        CoordIJ externalIj;
                        int externalIjFailed =
                            H3_EXPORT(experimentalH3ToLocalIj)(
                                externalOrigin, testIndex, &externalIj);

                        t_assert(
                            (bool)internalIjFailed == (bool)externalIjFailed,
                            "internal/external failed matches when getting IJ");

                        if (internalIjFailed) {
                            continue;
                        }

                        H3Index internalIndex;
                        int internalIjFailed2 =
                            H3_EXPORT(experimentalLocalIjToH3)(
                                internalOrigin, &internalIj, &internalIndex);
                        H3Index externalIndex;
                        int externalIjFailed2 =
                            H3_EXPORT(experimentalLocalIjToH3)(
                                externalOrigin, &externalIj, &externalIndex);

                        t_assert(
                            (bool)internalIjFailed2 == (bool)externalIjFailed2,
                            "internal/external failed matches when getting "
                            "index");

                        if (internalIjFailed2) {
                            continue;
                        }

                        t_assert(internalIndex == externalIndex,
                                 "internal/external index matches");
                    }
                }
            }
        }
    }
}

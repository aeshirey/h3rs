

/**
 * _hexRadiusKm returns the radius of a given hexagon in Km
 *
 * @param h3Index the index of the hexagon
 * @return the radius of the hexagon in Km
 */
double _hexRadiusKm(H3Index h3Index) {
    // There is probably a cheaper way to determine the radius of a
    // hexagon, but this way is conceptually simple
    GeoCoord h3Center;
    GeoBoundary h3Boundary;
    H3_EXPORT(h3ToGeo)(h3Index, &h3Center);
    H3_EXPORT(h3ToGeoBoundary)(h3Index, &h3Boundary);
    return H3_EXPORT(pointDistKm)(&h3Center, h3Boundary.verts);
}

/**
 * lineHexEstimate returns an estimated number of hexagons that trace
 *                 the cartesian-projected line
 *
 *  @param origin the origin coordinates
 *  @param destination the destination coordinates
 *  @param res the resolution of the H3 hexagons to trace the line
 *  @return the estimated number of hexagons required to trace the line
 */
int lineHexEstimate(const GeoCoord* origin, const GeoCoord* destination,
                    int res) {
    // Get the area of the pentagon as the maximally-distorted area possible
    H3Index pentagons[12] = {0};
    H3_EXPORT(getPentagonIndexes)(res, pentagons);
    double pentagonRadiusKm = _hexRadiusKm(pentagons[0]);

    double dist = H3_EXPORT(pointDistKm)(origin, destination);
    int estimate = (int)ceil(dist / (2 * pentagonRadiusKm));
    if (estimate == 0) estimate = 1;
    return estimate;
}

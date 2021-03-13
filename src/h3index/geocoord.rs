
/**
 * Area of H3 cell in radians^2.
 *
 * The area is calculated by breaking the cell into spherical triangles and
 * summing up their areas. Note that some H3 cells (hexagons and pentagons)
 * are irregular, and have more than 6 or 5 sides.
 *
 * todo: optimize the computation by re-using the edges shared between triangles
 *
 * @param   cell  H3 cell
 *
 * @return        cell area in radians^2
 */
/*
fn H3_EXPORT(cellAreaRads2)(H3Index cell) {
    GeoCoord c;
    GeoBoundary gb;
    H3_EXPORT(h3ToGeo)(cell, &c);
    H3_EXPORT(h3ToGeoBoundary)(cell, &gb);

    double area = 0.0;
    for (int i = 0; i < gb.numVerts; i++) {
        int j = (i + 1) % gb.numVerts;
        area += triangleArea(&gb.verts[i], &gb.verts[j], &c);
    }

    return area;
}

/**
 * Area of H3 cell in kilometers^2.
 */
double H3_EXPORT(cellAreaKm2)(H3Index h) {
    return H3_EXPORT(cellAreaRads2)(h) * EARTH_RADIUS_KM * EARTH_RADIUS_KM;
}

/**
 * Area of H3 cell in meters^2.
 */
double H3_EXPORT(cellAreaM2)(H3Index h) {
    return H3_EXPORT(cellAreaKm2)(h) * 1000 * 1000;
}

/**
 * Length of a unidirectional edge in radians.
 *
 * @param   edge  H3 unidirectional edge
 *
 * @return        length in radians
 */
double H3_EXPORT(exactEdgeLengthRads)(H3Index edge) {
    GeoBoundary gb;

    H3_EXPORT(getH3UnidirectionalEdgeBoundary)(edge, &gb);

    double length = 0.0;
    for (int i = 0; i < gb.numVerts - 1; i++) {
        length += H3_EXPORT(pointDistRads)(&gb.verts[i], &gb.verts[i + 1]);
    }

    return length;
}

/**
 * Length of a unidirectional edge in kilometers.
 */
double H3_EXPORT(exactEdgeLengthKm)(H3Index edge) {
    return H3_EXPORT(exactEdgeLengthRads)(edge) * EARTH_RADIUS_KM;
}

/**
 * Length of a unidirectional edge in meters.
 */
double H3_EXPORT(exactEdgeLengthM)(H3Index edge) {
    return H3_EXPORT(exactEdgeLengthKm)(edge) * 1000;
}

*/

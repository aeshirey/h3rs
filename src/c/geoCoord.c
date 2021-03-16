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
/** @file geoCoord.c
 * @brief   Functions for working with lat/lon coordinates.
 */

#include "geoCoord.h"

#include <math.h>
#include <stdbool.h>

#include "constants.h"
#include "h3api.h"
#include "mathExtensions.h"

/**
 * Normalizes radians to a value between 0.0 and two PI.
 *
 * @param rads The input radians value.
 * @return The normalized radians value.
 */
double _posAngleRads(double rads) {
    double tmp = ((rads < 0.0L) ? rads + M_2PI : rads);
    if (rads >= M_2PI) tmp -= M_2PI;
    return tmp;
}

/**
 * Determines if the components of two spherical coordinates are within some
 * threshold distance of each other.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @param threshold The threshold distance.
 * @return Whether or not the two coordinates are within the threshold distance
 *         of each other.
 */
bool geoAlmostEqualThreshold(const GeoCoord *p1, const GeoCoord *p2,
                             double threshold) {
    return fabs(p1->lat - p2->lat) < threshold &&
           fabs(p1->lon - p2->lon) < threshold;
}

/**
 * Determines if the components of two spherical coordinates are within our
 * standard epsilon distance of each other.
 *
 * @param p1 The first spherical coordinates.
 * @param p2 The second spherical coordinates.
 * @return Whether or not the two coordinates are within the epsilon distance
 *         of each other.
 */
bool geoAlmostEqual(const GeoCoord *p1, const GeoCoord *p2) {
    return geoAlmostEqualThreshold(p1, p2, EPSILON_RAD);
}

/**
 * Set the components of spherical coordinates in decimal degrees.
 *
 * @param p The spherical coordinates.
 * @param latDegs The desired latitude in decimal degrees.
 * @param lonDegs The desired longitude in decimal degrees.
 */
void setGeoDegs(GeoCoord *p, double latDegs, double lonDegs) {
    _setGeoRads(p, H3_EXPORT(degsToRads)(latDegs),
                H3_EXPORT(degsToRads)(lonDegs));
}

/**
 * Set the components of spherical coordinates in radians.
 *
 * @param p The spherical coordinates.
 * @param latRads The desired latitude in decimal radians.
 * @param lonRads The desired longitude in decimal radians.
 */
void _setGeoRads(GeoCoord *p, double latRads, double lonRads) {
    p->lat = latRads;
    p->lon = lonRads;
}

/**
 * Convert from decimal degrees to radians.
 *
 * @param degrees The decimal degrees.
 * @return The corresponding radians.
 */
double H3_EXPORT(degsToRads)(double degrees) { return degrees * M_PI_180; }

/**
 * Convert from radians to decimal degrees.
 *
 * @param radians The radians.
 * @return The corresponding decimal degrees.
 */
double H3_EXPORT(radsToDegs)(double radians) { return radians * M_180_PI; }

/**
 * constrainLat makes sure latitudes are in the proper bounds
 *
 * @param lat The original lat value
 * @return The corrected lat value
 */
double constrainLat(double lat) {
    while (lat > M_PI_2) {
        lat = lat - M_PI;
    }
    return lat;
}

/**
 * constrainLng makes sure longitudes are in the proper bounds
 *
 * @param lng The origin lng value
 * @return The corrected lng value
 */
double constrainLng(double lng) {
    while (lng > M_PI) {
        lng = lng - (2 * M_PI);
    }
    while (lng < -M_PI) {
        lng = lng + (2 * M_PI);
    }
    return lng;
}










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
double H3_EXPORT(cellAreaRads2)(H3Index cell) {
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

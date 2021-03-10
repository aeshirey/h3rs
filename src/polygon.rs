
/** @struct GeoPolygon
 *  @brief Simplified core of GeoJSON Polygon coordinates definition
 */
pub struct GeoPolygon{
    /// exterior boundary of the polygon
    geofence: Geofence , 

    /// number of elements in the array pointed to by holes
     numHoles: i32, 

     /// interior boundaries (holes) in the polygon
    holes : Vec<Geofence  >,
}


// Macros for use with polygonAlgos.h
/** Macro: Init iteration vars for Geofence */
//#define INIT_ITERATION_GEOFENCE int loopIndex = -1
const loopIndex : i32 = -1;

/** Macro: Increment Geofence loop iteration, or break if done. */
/*
#define ITERATE_GEOFENCE(geofence, vertexA, vertexB) \
    if (++loopIndex >= geofence->numVerts) break;    \
    vertexA = geofence->verts[loopIndex];            \
    vertexB = geofence->verts[(loopIndex + 1) % geofence->numVerts]
    */

/** Macro: Whether a Geofence is empty */
fn IS_EMPTY_GEOFENCE(geofence: GeoFence) -> bool {
    geofence.numVerts == 0
}


impl GeoFence {
    /**
     * Create a bounding box from a GeoPolygon
     * @param polygon Input GeoPolygon
     * @param bboxes  Output bboxes, one for the outer loop and one for each hole
     */
    fn bboxesFromGeoPolygon(&self) -> Vec<BBox> {
        todo!()
        /*
        bboxFromGeofence(&polygon->geofence, &bboxes[0]);
        for (int i = 0; i < polygon->numHoles; i++) {
            bboxFromGeofence(&polygon->holes[i], &bboxes[i + 1]);
        }
        */
    }



    /**
     * pointInsidePolygon takes a given GeoPolygon data structure and
     * checks if it contains a given geo coordinate.
     *
     * @param geoPolygon The geofence and holes defining the relevant area
     * @param bboxes     The bboxes for the main geofence and each of its holes
     * @param coord      The coordinate to check
     * @return           Whether the point is contained
     */
    fn pointInsidePolygon(geoPolygon: GeoPolygon, bboxes: Vec<BBox>, coord: &GeoCoord) -> bool {
        // Start with contains state of primary geofence
        let contains = pointInsideGeofence(&geoPolygon.geofence, &bboxes[0], coord);

        // If the point is contained in the primary geofence, but there are holes in
        // the geofence iterate through all holes and return false if the point is
        // contained in any hole
        if contains && geoPolygon->numHoles > 0 {
            for i in 0..geoPolygon.numholes {
                todo!()
                    /*
                if (pointInsideGeofence(&(geoPolygon->holes[i]), &bboxes[i + 1], coord)) {
                    return false;
                }
                    */
            }
        }

        contains
    }
}
